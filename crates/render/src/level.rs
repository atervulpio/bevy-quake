//! What a level is made into for Bevy once, when it begins: its pictures as Bevy's images and
//! materials, its brush models as meshes lit by Bevy's lightmaps, its probes as Bevy's irradiance
//! volume (each light style's lamps added as bright as the style is now); and the pools the frame
//! draws its things from.

use crate::surfaces::{LiquidMaterial, SkyMaterial};
use crate::{BrushMesh, FaceLight, GLOW, LevelMeshes, LightData, MOST, NewLevel, PictureData, REFLECTANCE, SKY, SURFACE, Skin, Slot};
use bevy::asset::RenderAssetUsages;
use bevy::camera::primitives::Aabb;
use bevy::image::{ImageAddressMode, ImageFilterMode, ImageSampler, ImageSamplerDescriptor};
use bevy::light::{IrradianceVolume, LightProbe};
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::prelude::*;
use bevy::render::render_resource::{Extent3d, TextureDimension, TextureFormat};
use std::collections::HashMap;

/// The entity everything of the level stands under: the game's space, turned so its z is Bevy's y.
#[derive(Component)]
pub(crate) struct Quake;

/// The turn from the game's space to Bevy's: a quarter about x, the game's z up becoming y.
pub(crate) fn quake_turn() -> Quat {
    Quat::from_rotation_x(-std::f32::consts::FRAC_PI_2)
}

/// A level's lightmap whose light changes with its styles: the faces whose slots carry a style
/// other than nought, each block laid in a picture of their own, and each slot's light kept here
/// to be laid together again whenever a style's brightness changes.
pub(crate) struct Changing {
    pub image: Handle<Image>,
    pub width: u32,
    pub faces: Vec<ChangingFace>,
    /// The styles' brightness the picture was last laid with.
    pub laid: Vec<f32>,
}

pub(crate) struct ChangingFace {
    /// Its block's corner (its border included) in the picture, its size, its styles, and each
    /// slot's light, row by row.
    pub at: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub styles: [u8; 4],
    pub light: [Vec<Vec3>; 4],
}

/// The still part of the light: every face whose light is style nought's alone, laid once (again
/// only if style nought's own brightness changes, which id's game never does).
pub(crate) struct Still {
    pub image: Handle<Image>,
    pub layer: Vec<u32>,
    pub laid: f32,
}

/// A brush model's faces, a mesh for each texture slot and kind of light.
pub(crate) struct BrushModel {
    pub groups: Vec<(Handle<Mesh>, u32, Lit)>,
    /// One copy for each time it is drawn in a frame.
    pub pool: Vec<BrushCopy>,
}

/// A brush model's copy: its entity, and its groups' entities and the skins they wear now.
pub(crate) struct BrushCopy {
    pub entity: Entity,
    pub groups: Vec<(Entity, Option<Skin>)>,
}

/// Which light a group of faces is lit by.
#[derive(Clone, Copy, PartialEq, Eq, Hash)]
pub(crate) enum Lit {
    /// The probes, as a model is (the sky, the liquids, a pick-up's box of its own file).
    Probes,
    Still,
    Changing,
}

/// A model of the game's own and its copies, each with a mesh of its own, the pose it shows.
pub(crate) struct AliasModel {
    pub mesh: crate::AliasMesh,
    pub bounds: Aabb,
    pub pool: Vec<AliasCopy>,
}

pub(crate) struct AliasCopy {
    pub entity: Entity,
    pub mesh: Handle<Mesh>,
    pub pose: (usize, usize, f32),
    pub picture: Option<u32>,
    /// Whether it is drawn as the weapon in hand (it then throws no shadow).
    pub in_hand: bool,
}

/// The level as Bevy holds it.
#[derive(Resource)]
pub(crate) struct Level {
    pub root: Entity,
    pub slots: Vec<Slot>,
    pub pictures: Vec<PictureData>,
    pub surfaces: HashMap<u32, Handle<StandardMaterial>>,
    pub skies: HashMap<u32, Handle<SkyMaterial>>,
    pub liquids: HashMap<u32, Handle<LiquidMaterial>>,
    pub sprites: HashMap<u32, (Handle<StandardMaterial>, Handle<Mesh>)>,
    pub brush: Vec<BrushModel>,
    pub alias: Vec<AliasModel>,
    pub still: Option<Still>,
    pub changing: Option<Changing>,
    pub volume: Option<Volume>,
    /// The moving lights' entities, the sprites', and the particles' meshes, made as they are
    /// first needed.
    pub moving: Vec<Entity>,
    pub billboards: Vec<Entity>,
    pub dots: Vec<(Entity, Handle<Mesh>)>,
    /// The particles' materials: lit, or giving off light of a colour (its bits); and their
    /// picture, a disc soft at its edge.
    pub dot_materials: HashMap<Option<[u32; 3]>, Handle<StandardMaterial>>,
    pub disc: Handle<Image>,
}

/// A particle's picture (`particle_fs` before row 41): a disc whose edge softens over its outer
/// half, white, its share shown in `a`.
fn disc() -> Image {
    const SIZE: u32 = 64;
    let texels: Vec<u8> = (0..SIZE * SIZE)
        .flat_map(|k| {
            let (x, y) = ((k % SIZE) as f32 + 0.5, (k / SIZE) as f32 + 0.5);
            let r = Vec2::new(x / SIZE as f32 * 2.0 - 1.0, y / SIZE as f32 * 2.0 - 1.0).length();
            let a = ((1.0 - r).clamp(0.0, 1.0) * 2.0).min(1.0);
            [255, 255, 255, (a * 255.0).round() as u8]
        })
        .collect();
    let mut image = Image::new(Extent3d { width: SIZE, height: SIZE, depth_or_array_layers: 1 }, TextureDimension::D2, texels, TextureFormat::Rgba8Unorm, RenderAssetUsages::RENDER_WORLD);
    image.sampler = ImageSampler::linear();
    image
}

/// The surfaces' sampler, as id's renderer filtered: a texel shown as it is when near, the levels
/// of detail blended when far.
fn surfaces_sampler() -> ImageSampler {
    ImageSampler::Descriptor(ImageSamplerDescriptor {
        address_mode_u: ImageAddressMode::Repeat,
        address_mode_v: ImageAddressMode::Repeat,
        mag_filter: ImageFilterMode::Nearest,
        min_filter: ImageFilterMode::Linear,
        mipmap_filter: ImageFilterMode::Linear,
        ..default()
    })
}

/// An image of every level of detail given, `Rgba8` of `format`.
fn image_of(width: u32, height: u32, levels: &[Vec<[u8; 4]>], format: TextureFormat, sampler: ImageSampler) -> Image {
    let mut image = Image::new_uninit(Extent3d { width, height, depth_or_array_layers: 1 }, TextureDimension::D2, format, RenderAssetUsages::RENDER_WORLD);
    image.data = Some(levels.iter().flat_map(|l| bytemuck::cast_slice::<[u8; 4], u8>(l).iter().copied()).collect());
    image.texture_descriptor.mip_level_count = levels.len() as u32;
    image.sampler = sampler;
    image
}

fn linear(c: u8) -> f32 {
    data::srgb_to_linear(c as f32 / 255.0)
}

fn srgb(v: f32) -> u8 {
    (data::linear_to_srgb(v.clamp(0.0, 1.0)) * 255.0).round() as u8
}

/// A picture's colours as they are (the sky, the liquids, the sprites).
pub(crate) fn colour_image(p: &PictureData) -> Image {
    image_of(p.width, p.height, &p.levels, TextureFormat::Rgba8UnormSrgb, surfaces_sampler())
}

/// A surface's material (row 41): its reflectance, the palette's colour times `REFLECTANCE`, never
/// past `MOST`; what of it glows by itself (`a`), at `GLOW`; flat and matte as id's surfaces are
/// (roughness 0.92, a non-metal's 4% gloss straight on).
fn surface_material(p: &PictureData, images: &mut Assets<Image>) -> StandardMaterial {
    let each = |f: &dyn Fn([u8; 4]) -> [u8; 4]| -> Vec<Vec<[u8; 4]>> { p.levels.iter().map(|l| l.iter().map(|&c| f(c)).collect()).collect() };
    let base = each(&|[r, g, b, _]| {
        let [r, g, b] = [r, g, b].map(|c| srgb((linear(c) * REFLECTANCE).min(MOST)));
        [r, g, b, 255]
    });
    let glows = p.levels[0].iter().any(|c| c[3] > 0);
    let emissive = glows.then(|| {
        let levels = each(&|[r, g, b, a]| {
            let [r, g, b] = [r, g, b].map(|c| srgb(linear(c) * a as f32 / 255.0));
            [r, g, b, 255]
        });
        images.add(image_of(p.width, p.height, &levels, TextureFormat::Rgba8UnormSrgb, surfaces_sampler()))
    });
    StandardMaterial {
        base_color_texture: Some(images.add(image_of(p.width, p.height, &base, TextureFormat::Rgba8UnormSrgb, surfaces_sampler()))),
        emissive: if glows { LinearRgba::rgb(GLOW, GLOW, GLOW) } else { LinearRgba::BLACK },
        emissive_texture: emissive,
        emissive_exposure_weight: 1.0,
        perceptual_roughness: 0.92,
        metallic: 0.0,
        reflectance: 0.5,
        ..default()
    }
}

/// The card's shared-exponent colour back to three numbers.
pub(crate) fn from_rgb9e5(v: u32) -> Vec3 {
    let e = (v >> 27) as i32 - 15 - 9;
    let s = 2f32.powi(e);
    Vec3::new((v & 511) as f32 * s, ((v >> 9) & 511) as f32 * s, ((v >> 18) & 511) as f32 * s)
}

fn rgb9e5_image(width: u32, height: u32, depth: u32, texels: &[u32], dimension: TextureDimension) -> Image {
    let mut image = Image::new(Extent3d { width, height, depth_or_array_layers: depth }, dimension, bytemuck::cast_slice(texels).to_vec(), TextureFormat::Rgb9e5Ufloat, RenderAssetUsages::RENDER_WORLD | RenderAssetUsages::MAIN_WORLD);
    image.sampler = ImageSampler::linear();
    image
}

/// The probes as Bevy's irradiance volume: one 3D picture of (x, 2y, 3z) points, each axis of
/// Bevy's world in its slab (x, y, z), its negative half below its positive. Bevy's axes are the
/// game's turned: its +x the game's +x, its +y the game's +z, its +z the game's −y. Each other
/// style's lamps' light is kept apart at the points it reaches, and added again, as bright as the
/// style is now, whenever a style's brightness changes (the lamps of style nought are in the
/// probes themselves).
pub(crate) struct Volume {
    pub image: Handle<Image>,
    pub touched: Vec<Touched>,
    /// The styles' brightness the picture was last laid with.
    pub laid: Vec<f32>,
}

/// A point some style's lamps reach: its six texels in the picture, its light of style nought on
/// each, and each style's.
pub(crate) struct Touched {
    pub texels: [usize; 6],
    pub base: [Vec3; 6],
    pub styles: Vec<(u8, [Vec3; 6])>,
}

/// Bevy's (+x, −x, +y, −y, +z, −z), each from the game's axis of the same direction, and the slab
/// it lies in.
const FROM_GAME: [(usize, bool, usize); 6] = [(0, true, 0), (1, false, 0), (4, true, 1), (5, false, 1), (3, true, 2), (2, false, 2)];

/// Where Bevy's axis `j` of the game's probe `k` lies in the picture.
fn texel(dims: [usize; 3], k: usize, j: usize) -> usize {
    let [dx, dy, dz] = dims;
    let (x, y, z) = (k % dx, (k / dx) % dy, k / (dx * dy));
    let (_, positive, slab) = FROM_GAME[j];
    let t = y + if positive { dy } else { 0 };
    let p = z + slab * dz;
    (p * 2 * dy + t) * dx + x
}

fn volume(l: &LightData, images: &mut Assets<Image>) -> Volume {
    let dims = l.probe_dims.map(|d| d as usize);
    let [dx, dy, dz] = dims;
    let mut texels = vec![0u32; dx * 2 * dy * 3 * dz];
    for k in 0..dx * dy * dz {
        for (j, &(game, _, _)) in FROM_GAME.iter().enumerate() {
            texels[texel(dims, k, j)] = l.probes[game][k];
        }
    }
    let mut touched: Vec<Touched> = Vec::new();
    let mut at: HashMap<u32, usize> = HashMap::new();
    for s in &l.styled_probes {
        for (&k, c) in s.points.iter().zip(&s.cube) {
            let n = *at.entry(k).or_insert_with(|| {
                let k = k as usize;
                touched.push(Touched {
                    texels: std::array::from_fn(|j| texel(dims, k, j)),
                    base: std::array::from_fn(|j| from_rgb9e5(l.probes[FROM_GAME[j].0][k])),
                    styles: Vec::new(),
                });
                touched.len() - 1
            });
            touched[n].styles.push((s.style, std::array::from_fn(|j| c[FROM_GAME[j].0])));
        }
    }
    Volume { image: images.add(rgb9e5_image(dx as u32, 2 * dy as u32, 3 * dz as u32, &texels, TextureDimension::D3)), touched, laid: Vec::new() }
}

impl Volume {
    /// Whether a style these points carry has changed its brightness since they were laid.
    pub(crate) fn stale(&self, styles: &[f32]) -> bool {
        self.touched.iter().any(|t| t.styles.iter().any(|(s, _)| self.laid.get(*s as usize & 63) != styles.get(*s as usize & 63)))
    }

    pub(crate) fn lay(&mut self, styles: &[f32], images: &mut Assets<Image>) {
        let Some(mut image) = images.get_mut(&self.image) else { return };
        let Some(data) = image.data.as_mut() else { return };
        let texels: &mut [u32] = bytemuck::cast_slice_mut(data);
        for t in &self.touched {
            for j in 0..6 {
                let e = t.styles.iter().fold(t.base[j], |e, (s, c)| e + c[j] * styles[*s as usize & 63]);
                texels[t.texels[j]] = light::atlas::rgb9e5(e);
            }
        }
        self.laid = styles.to_vec();
    }
}

/// The changing faces' own picture: its size, its faces, and by the map's face where its block
/// stands in it.
struct ChangingLayout {
    width: u32,
    height: u32,
    faces: Vec<ChangingFace>,
    places: Vec<Option<(u32, u32)>>,
}

/// The changing faces laid in a picture of their own, rows of blocks as tall as the tallest.
fn changing_faces(l: &LightData) -> Option<ChangingLayout> {
    let mut faces: Vec<(usize, FaceLight)> = l.faces.iter().enumerate().filter_map(|(i, f)| f.filter(|f| f.styles.iter().any(|&s| s != 0 && s != 255)).map(|f| (i, f))).collect();
    if faces.is_empty() {
        return None;
    }
    faces.sort_by_key(|(_, f)| std::cmp::Reverse(f.height));
    let area: u32 = faces.iter().map(|(_, f)| (f.width + 2) * (f.height + 2)).sum();
    let width = ((area as f32).sqrt().ceil() as u32).max(faces.iter().map(|(_, f)| f.width + 2).max().unwrap_or(1)).next_power_of_two();
    let (mut x, mut y, mut shelf) = (0, 0, 0);
    let mut places = vec![None; l.faces.len()];
    let mut out = Vec::new();
    for (i, f) in faces {
        let (w, h) = (f.width + 2, f.height + 2);
        if x + w > width {
            x = 0;
            y += shelf;
            shelf = 0;
        }
        places[i] = Some((x, y));
        // the block and its border, as the atlas holds it, each slot's light read back
        let light = std::array::from_fn(|slot| {
            if f.styles[slot] == 255 {
                return Vec::new();
            }
            let mut v = Vec::with_capacity((w * h) as usize);
            for j in 0..h {
                for k in 0..w {
                    v.push(from_rgb9e5(l.layers[slot][((f.place.1 - 1 + j) * l.width + f.place.0 - 1 + k) as usize]));
                }
            }
            v
        });
        out.push(ChangingFace { at: (x, y), width: w, height: h, styles: f.styles, light });
        x += w;
        shelf = shelf.max(h);
    }
    Some(ChangingLayout { width, height: (y + shelf).max(1), faces: out, places })
}

impl Changing {
    /// The picture laid with the styles' brightness now: each texel each slot's light times its
    /// style's brightness, summed.
    pub(crate) fn lay(&mut self, styles: &[f32], images: &mut Assets<Image>) {
        let Some(mut image) = images.get_mut(&self.image) else { return };
        let Some(data) = image.data.as_mut() else { return };
        let texels: &mut [u32] = bytemuck::cast_slice_mut(data);
        for f in &self.faces {
            for j in 0..f.height {
                for k in 0..f.width {
                    let n = (j * f.width + k) as usize;
                    let mut e = Vec3::ZERO;
                    for slot in 0..4 {
                        if f.styles[slot] != 255 {
                            e += f.light[slot][n] * styles[f.styles[slot] as usize & 63];
                        }
                    }
                    texels[((f.at.1 + j) * self.width + f.at.0 + k) as usize] = light::atlas::rgb9e5(e);
                }
            }
        }
        self.laid = styles.to_vec();
    }

    /// Whether a style these faces carry has changed its brightness since they were laid.
    pub(crate) fn stale(&self, styles: &[f32]) -> bool {
        self.faces.iter().any(|f| f.styles.iter().any(|&s| s != 255 && self.laid.get(s as usize & 63) != styles.get(s as usize & 63)))
    }
}

impl Still {
    pub(crate) fn lay(&mut self, v: f32, images: &mut Assets<Image>) {
        let Some(mut image) = images.get_mut(&self.image) else { return };
        let Some(data) = image.data.as_mut() else { return };
        let texels: &mut [u32] = bytemuck::cast_slice_mut(data);
        for (t, &s) in texels.iter_mut().zip(&self.layer) {
            *t = if v == 1.0 { s } else { light::atlas::rgb9e5(from_rgb9e5(s) * v) };
        }
        self.laid = v;
    }
}

/// A group's corners: places, normals, places on the texture, places in the lightmap.
#[derive(Default)]
struct Group(Vec<[f32; 3]>, Vec<[f32; 3]>, Vec<[f32; 2]>, Vec<[f32; 2]>);

/// A brush model's faces grouped by texture slot and light, each group a Bevy mesh: its places,
/// normals, places on the texture and, lit by a lightmap, places in it (`UV_1`).
fn brush_groups(b: &BrushMesh, light: Option<&LightData>, changing: &[Option<(u32, u32)>], changing_size: (u32, u32), meshes: &mut Assets<Mesh>) -> Vec<(Handle<Mesh>, u32, Lit)> {
    let mut groups: HashMap<(u32, Lit), Group> = HashMap::new();
    let mut order: Vec<(u32, Lit)> = Vec::new();
    for tri in b.indices.chunks_exact(3) {
        let first = &b.vertices[tri[0] as usize];
        let face = first.face as usize;
        let lit = match (light, first.face) {
            (Some(_), f) if f != u32::MAX && changing.get(face).copied().flatten().is_some() => Lit::Changing,
            (Some(_), f) if f != u32::MAX => Lit::Still,
            _ => Lit::Probes,
        };
        let key = (first.slot, lit);
        let g = groups.entry(key).or_insert_with(|| {
            order.push(key);
            Default::default()
        });
        for &i in tri {
            let v = &b.vertices[i as usize];
            g.0.push(v.position.to_array());
            g.1.push(v.normal.to_array());
            g.2.push(v.uv.to_array());
            let uv1 = match (lit, light) {
                (Lit::Still, Some(l)) => v.light / Vec2::new(l.width as f32, l.height as f32),
                (Lit::Changing, Some(l)) => {
                    let f = l.faces[face].expect("a lit face's light");
                    let to = changing[face].expect("a changing face's place");
                    // the block's corner with its border moves from the atlas to its own picture
                    (v.light - Vec2::new(f.place.0 as f32 - 1.0, f.place.1 as f32 - 1.0) + Vec2::new(to.0 as f32, to.1 as f32)) / Vec2::new(changing_size.0 as f32, changing_size.1 as f32)
                }
                _ => Vec2::ZERO,
            };
            g.3.push(uv1.to_array());
        }
    }
    order
        .into_iter()
        .map(|key| {
            let Group(p, n, uv, uv1) = groups.remove(&key).expect("each group made");
            let count = p.len() as u32;
            let mut mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
                .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, p)
                .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, n)
                .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
                .with_inserted_indices(Indices::U32((0..count).collect()));
            if key.1 != Lit::Probes {
                mesh.insert_attribute(Mesh::ATTRIBUTE_UV_1, uv1);
            }
            (meshes.add(mesh), key.0, key.1)
        })
        .collect()
}

/// The intensity Bevy is given for a lamp of ours (row 41). Ours is the irradiance a surface
/// square to the lamp gets at one unit (`E = I cos θ / d²`), and a surface's brightness is its
/// reflectance times `E`. Bevy's point light is given in lumens, divided by 4π for its candela,
/// and its diffuse light is the reflectance over π times the irradiance: so 4π² of ours.
pub(crate) fn lumens(intensity: f32) -> f32 {
    4.0 * std::f32::consts::PI * std::f32::consts::PI * intensity
}

/// How far a lamp's light is followed: to where it gives less than a two-thousandth of a painted
/// surface's full light, as the light's own computation stops (`DIM` / 4 in `crates/light`).
pub(crate) fn reach(intensity: f32) -> f32 {
    (intensity.abs() / 0.0005).sqrt().max(1.0)
}

/// How far Bevy's shadows are pushed off a surface: its default of 0.08, in metres, brought to the
/// game's units (a man 56 units tall, some 1.8 m: about 31 units a metre).
pub(crate) const SHADOW_DEPTH_BIAS: f32 = 0.08 * 31.0;

/// The new level made, the old one gone.
#[allow(clippy::too_many_arguments)]
pub(crate) fn take(
    mut commands: Commands,
    mut new: ResMut<NewLevel>,
    old: Option<Res<Level>>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut standard: ResMut<Assets<StandardMaterial>>,
    mut skies: ResMut<Assets<SkyMaterial>>,
    mut liquids: ResMut<Assets<LiquidMaterial>>,
) {
    let Some(m) = new.0.take() else { return };
    if let Some(old) = old {
        commands.entity(old.root).despawn();
    }
    let root = commands.spawn((Quake, Transform::from_rotation(quake_turn()), Visibility::Visible)).id();
    let LevelMeshes { brush, alias, light, pictures, slots } = m;
    // the light: the still faces' picture, the changing faces' own, the probes
    let (mut still, mut changing, mut volume_made, mut changing_places, mut changing_size) = (None, None, None, Vec::new(), (1, 1));
    if let Some(l) = &light {
        let mut s = Still { image: images.add(rgb9e5_image(l.width, l.height, 1, &l.layers[0], TextureDimension::D2)), layer: l.layers[0].clone(), laid: 1.0 };
        s.lay(1.0, &mut images);
        still = Some(s);
        if let Some(ChangingLayout { width: w, height: h, faces, places }) = changing_faces(l) {
            let image = images.add(rgb9e5_image(w, h, 1, &vec![0; (w * h) as usize], TextureDimension::D2));
            changing = Some(Changing { image, width: w, faces, laid: Vec::new() });
            changing_places = places;
            changing_size = (w, h);
        }
        let [dx, dy, dz] = l.probe_dims.map(|d| d as f32);
        let size = Vec3::new(dx, dy, dz) * l.probe_spacing;
        let centre = l.probe_origin + (Vec3::new(dx, dy, dz) - Vec3::ONE) * l.probe_spacing * 0.5;
        let v = volume(l, &mut images);
        commands.spawn((
            ChildOf(root),
            LightProbe::default(),
            IrradianceVolume { voxels: v.image.clone(), intensity: 1.0, affects_lightmapped_meshes: false },
            Transform::from_translation(centre).with_scale(size),
        ));
        volume_made = Some(v);
    }
    // every picture a texture slot shows, as what its kind asks
    let mut surfaces = HashMap::new();
    let mut sky_mats = HashMap::new();
    let mut liquid_mats = HashMap::new();
    for skin in slots.iter().flat_map(|s| s.frames.iter().chain(&s.alternate)) {
        let p = &pictures[skin.picture as usize];
        match skin.kind {
            SURFACE => {
                surfaces.entry(skin.picture).or_insert_with(|| standard.add(surface_material(p, &mut images)));
            }
            SKY => {
                sky_mats.entry(skin.picture).or_insert_with(|| skies.add(SkyMaterial { eye: Vec4::ZERO, light: Vec4::ZERO, picture: images.add(colour_image(p)) }));
            }
            _ => {
                liquid_mats.entry(skin.picture).or_insert_with(|| liquids.add(LiquidMaterial { eye: Vec4::ZERO, light: Vec4::ZERO, picture: images.add(colour_image(p)) }));
            }
        }
    }
    for a in &alias {
        for &picture in &a.skins {
            surfaces.entry(picture).or_insert_with(|| standard.add(surface_material(&pictures[picture as usize], &mut images)));
        }
    }
    let brush = brush.iter().map(|b| BrushModel { groups: brush_groups(b, light.as_ref(), &changing_places, changing_size, &mut meshes), pool: Vec::new() }).collect();
    let alias = alias
        .into_iter()
        .map(|mesh| {
            let (lo, hi) = mesh.poses.iter().flatten().fold((Vec3::splat(f32::MAX), Vec3::splat(f32::MIN)), |(lo, hi), v| (lo.min(v.position), hi.max(v.position)));
            AliasModel { bounds: Aabb::from_min_max(lo, hi), mesh, pool: Vec::new() }
        })
        .collect();
    let disc = images.add(disc());
    commands.insert_resource(Level {
        root,
        slots,
        pictures,
        surfaces,
        skies: sky_mats,
        liquids: liquid_mats,
        sprites: HashMap::new(),
        brush,
        alias,
        still,
        changing,
        volume: volume_made,
        moving: Vec::new(),
        billboards: Vec::new(),
        dots: Vec::new(),
        dot_materials: HashMap::new(),
        disc,
    });
}
