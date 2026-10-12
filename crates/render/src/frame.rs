//! One frame shown: the eye placed, the light's styles laid into the lightmaps and the lamps, each
//! thing drawn given its copy's place, pose and material, the moving lights, the sprites and the
//! particles, and what the passes after the tone curve lay over the picture.

use crate::level::{AliasCopy, BrushCopy, Level, Lit, SHADOW_DEPTH_BIAS, colour_image, lumens, quake_turn, reach};
use crate::screen::Overlay;
use crate::surfaces::{LiquidMaterial, SkyMaterial};
use crate::view::ADVANCE;
use crate::{Draw, GLOW, MOST, REFLECTANCE, SKY, SURFACE, Shown, View};
use bevy::asset::RenderAssetUsages;
use bevy::camera::visibility::NoFrustumCulling;
use bevy::core_pipeline::tonemapping::Tonemapping;
use bevy::light::NotShadowCaster;
use bevy::mesh::{Indices, PrimitiveTopology};
use bevy::pbr::Lightmap;
use bevy::prelude::*;
use bevy::camera::{Exposure, Hdr};
use shaders::Letter;
use std::sync::OnceLock;

/// The camera: Bevy's eye where the game's is.
#[derive(Component)]
pub(crate) struct Eye;

/// The eye, made once: drawn in light's own range, exposed as `exposure` says; the tone curve is
/// the game's own, in the passes after (`overlay_fs`), so Bevy lays none.
pub(crate) fn begin(mut commands: Commands, mut offscreen: ResMut<crate::Offscreen>, mut images: ResMut<Assets<Image>>) {
    let mut eye = commands.spawn((
        Eye,
        Camera3d::default(),
        Camera { clear_color: ClearColorConfig::Custom(Color::BLACK), ..default() },
        Hdr,
        Tonemapping::None,
        // the picture leaves Bevy's pass in light's own range, so Bevy has no bands to break up
        bevy::core_pipeline::tonemapping::DebandDither::Disabled,
        Exposure { ev100: ev100(1.0) },
        Projection::Perspective(PerspectiveProjection { fov: 1.0, near: 1.0, far: 65536.0, ..default() }),
        Transform::default(),
    ));
    if let Some((w, h)) = offscreen.size {
        let image = images.add(Image::new_target_texture(w, h, bevy::render::render_resource::TextureFormat::Rgba8UnormSrgb, None));
        eye.insert(bevy::camera::RenderTarget::Image(image.clone().into()));
        offscreen.image = Some(image);
    }
}

/// Bevy's exposure for the game's (row 41): the light multiplied by `exposure / REFLECTANCE`
/// before the tone curve, as the renderer before Bevy did. Bevy multiplies by `2^−ev100 / 1.2`.
fn ev100(exposure: f32) -> f32 {
    (REFLECTANCE / (1.2 * exposure)).log2()
}

/// Each printable letter's first stroke and count (`letters::table`).
fn glyphs() -> &'static [[u32; 2]] {
    static GLYPHS: OnceLock<Vec<[u32; 2]>> = OnceLock::new();
    GLYPHS.get_or_init(|| letters::table().1)
}

/// The view's lines of text as letters for `text_vs`; the screen's colours are already the
/// screen's, so a letter's colour is given in the screen's own values.
fn letters_of(view: &View) -> Vec<Letter> {
    let glyphs = glyphs();
    let mut out = Vec::new();
    for t in &view.texts {
        let scale = t.size / 6.0;
        let mut x = t.x;
        for c in t.text.chars() {
            let code = c.to_ascii_uppercase() as u32;
            let g = if (32..127).contains(&code) { glyphs[(code - 32) as usize] } else { glyphs[('?' as u32 - 32) as usize] };
            if g[1] > 0 {
                let lin = |v: f32| v.powf(2.2);
                out.push(Letter { place: Vec4::new(x, t.y, scale, 0.55), colour: Vec4::new(lin(t.colour.x), lin(t.colour.y), lin(t.colour.z), t.colour.w), strokes: UVec4::new(g[0], g[1], 0, 0) });
            }
            x += ADVANCE * scale;
        }
    }
    out
}

/// The model's place and normal at every corner between two poses.
fn pose(mesh: &crate::AliasMesh, from: usize, to: usize, t: f32) -> (Vec<[f32; 3]>, Vec<[f32; 3]>) {
    let (a, b) = (&mesh.poses[from.min(mesh.poses.len() - 1)], &mesh.poses[to.min(mesh.poses.len() - 1)]);
    a.iter().zip(b).map(|(a, b)| (a.position.lerp(b.position, t).to_array(), a.normal.lerp(b.normal, t).normalize_or_zero().to_array())).unzip()
}

#[allow(clippy::too_many_arguments)]
pub(crate) fn show(
    mut commands: Commands,
    shown: Res<Shown>,
    level: Option<ResMut<Level>>,
    mut eye: Query<(&mut Transform, &mut Projection, &mut Exposure), With<Eye>>,
    mut placed: Query<(&mut Transform, &mut Visibility), Without<Eye>>,
    mut points: Query<&mut PointLight>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut images: ResMut<Assets<Image>>,
    mut standard: ResMut<Assets<StandardMaterial>>,
    mut skies: ResMut<Assets<SkyMaterial>>,
    mut liquids: ResMut<Assets<LiquidMaterial>>,
    mut overlay: ResMut<Overlay>,
) {
    let view = &shown.view;
    let (f, r, u) = crate::angle_vectors(view.angles);
    if let Ok((mut t, mut p, mut x)) = eye.single_mut() {
        *t = Transform { translation: quake_turn() * view.origin, rotation: quake_turn() * Quat::from_mat3(&view.turn()), ..default() };
        if let Projection::Perspective(p) = &mut *p {
            p.fov = view.fov_y();
        }
        x.ev100 = ev100(view.exposure);
    }
    *overlay = Overlay { blend: view.blend, crt: view.crt, letters: letters_of(view) };
    let Some(mut level) = level else { return };
    let level = &mut *level;
    let styles = &view.styles;
    let style = |s: u8| styles.get(s as usize & 63).copied().unwrap_or(1.0);
    // the light's styles: the lightmaps and the probes laid again where a style changed
    if let Some(s) = &mut level.still
        && s.laid != style(0)
    {
        s.lay(style(0), &mut images);
    }
    if let Some(c) = &mut level.changing
        && c.stale(styles)
    {
        c.lay(styles, &mut images);
    }
    if let Some(v) = &mut level.volume
        && v.stale(styles)
    {
        v.lay(styles, &mut images);
    }
    // the sky and the liquids: where the eye is, the time, how bright
    let light = Vec4::new(GLOW * view.exposure / REFLECTANCE, 0.0, 0.0, 0.0);
    let eye_at = view.origin.extend(view.time);
    for h in level.skies.values() {
        if let Some(mut m) = skies.get_mut(h) {
            (m.eye, m.light) = (eye_at, light);
        }
    }
    for h in level.liquids.values() {
        if let Some(mut m) = liquids.get_mut(h) {
            (m.eye, m.light) = (eye_at, light);
        }
    }
    let root = level.root;
    // the brush models: each drawn copy placed, its groups wearing their slots' frames now
    let mut used = vec![0usize; level.brush.len()];
    let mut alias_used = vec![0usize; level.alias.len()];
    for (i, d) in shown.draws.iter().enumerate() {
        match *d {
            Draw::Brush { mesh, transform, alternate } => {
                let Some(b) = level.brush.get_mut(mesh) else { continue };
                let k = used[mesh];
                used[mesh] += 1;
                if b.pool.len() <= k {
                    let parent = commands.spawn((ChildOf(root), Transform::default(), Visibility::Hidden)).id();
                    let mut groups = Vec::new();
                    for (h, _, lit) in &b.groups {
                        let mut g = commands.spawn((ChildOf(parent), Mesh3d(h.clone()), Transform::default()));
                        let image = match lit {
                            Lit::Still => level.still.as_ref().map(|s| s.image.clone()),
                            Lit::Changing => level.changing.as_ref().map(|c| c.image.clone()),
                            Lit::Probes => None,
                        };
                        if let Some(image) = image {
                            g.insert(Lightmap { image, uv_rect: Rect::new(0.0, 0.0, 1.0, 1.0), bicubic_sampling: false });
                        }
                        groups.push((g.id(), None));
                    }
                    b.pool.push(BrushCopy { entity: parent, groups });
                }
                let BrushCopy { entity: parent, groups } = &mut b.pool[k];
                if let Ok((mut t, mut v)) = placed.get_mut(*parent) {
                    *t = Transform::from_matrix(transform);
                    *v = Visibility::Inherited;
                }
                for ((e, worn), (_, slot, _)) in groups.iter_mut().zip(&b.groups) {
                    let skin = level.slots[*slot as usize].at(alternate, view.time);
                    if *worn == Some(skin) {
                        continue;
                    }
                    *worn = Some(skin);
                    let mut g = commands.entity(*e);
                    match skin.kind {
                        SURFACE => {
                            if let Some(m) = level.surfaces.get(&skin.picture) {
                                g.insert(MeshMaterial3d(m.clone()));
                            }
                        }
                        SKY => {
                            if let Some(m) = level.skies.get(&skin.picture) {
                                g.insert(MeshMaterial3d(m.clone()));
                            }
                        }
                        _ => {
                            if let Some(m) = level.liquids.get(&skin.picture) {
                                g.insert(MeshMaterial3d(m.clone()));
                            }
                        }
                    }
                }
            }
            Draw::Alias { mesh, transform, from, to, t, skin } => {
                let Some(a) = level.alias.get_mut(mesh) else { continue };
                let k = alias_used[mesh];
                alias_used[mesh] += 1;
                if a.pool.len() <= k {
                    let (p, n) = pose(&a.mesh, from, to, t);
                    let m = meshes.add(
                        Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
                            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, p)
                            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, n)
                            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, a.mesh.uvs.iter().map(|v| v.to_array()).collect::<Vec<_>>())
                            .with_inserted_indices(Indices::U32(a.mesh.indices.clone())),
                    );
                    let entity = commands.spawn((ChildOf(root), Mesh3d(m.clone()), a.bounds, Transform::default(), Visibility::Hidden)).id();
                    a.pool.push(AliasCopy { entity, mesh: m, pose: (from, to, t), picture: None, in_hand: false });
                }
                let c = &mut a.pool[k];
                if let Ok((mut tr, mut v)) = placed.get_mut(c.entity) {
                    *tr = Transform::from_matrix(transform);
                    *v = Visibility::Inherited;
                }
                if c.pose != (from, to, t)
                    && let Some(mut m) = meshes.get_mut(&c.mesh)
                {
                    let (p, n) = pose(&a.mesh, from, to, t);
                    m.insert_attribute(Mesh::ATTRIBUTE_POSITION, p);
                    m.insert_attribute(Mesh::ATTRIBUTE_NORMAL, n);
                    c.pose = (from, to, t);
                }
                // a skin the model does not have is its first (QuakeC sets `skin` freely)
                let picture = a.mesh.skins.get(skin).or(a.mesh.skins.first()).copied();
                if c.picture != picture {
                    c.picture = picture;
                    if let Some(m) = picture.and_then(|p| level.surfaces.get(&p)) {
                        commands.entity(c.entity).insert(MeshMaterial3d(m.clone()));
                    }
                }
                // the weapon in hand stands in no light's way: it is not in the world
                let hand = view.in_hand.contains(&i);
                if c.in_hand != hand {
                    c.in_hand = hand;
                    if hand {
                        commands.entity(c.entity).insert(NotShadowCaster);
                    } else {
                        commands.entity(c.entity).remove::<NotShadowCaster>();
                    }
                }
            }
        }
    }
    // the copies not drawn this frame
    let hide = |e: Entity, placed: &mut Query<(&mut Transform, &mut Visibility), Without<Eye>>| {
        if let Ok((_, mut v)) = placed.get_mut(e) {
            *v = Visibility::Hidden;
        }
    };
    for (b, &n) in level.brush.iter().zip(&used) {
        for c in &b.pool[n.min(b.pool.len())..] {
            hide(c.entity, &mut placed);
        }
    }
    for (a, &n) in level.alias.iter().zip(&alias_used) {
        for c in &a.pool[n.min(a.pool.len())..] {
            hide(c.entity, &mut placed);
        }
    }
    // the moving lights (`dlight_t`): Bevy's lights lighting everything, the four that give the
    // eye the most light casting their shadows
    let mut order: Vec<usize> = (0..view.lights.len()).collect();
    order.sort_by(|&a, &b| {
        let at = |i: usize| view.lights[i].1 / (view.lights[i].0 - view.origin).length_squared().max(1.0);
        at(b).total_cmp(&at(a))
    });
    while level.moving.len() < view.lights.len() {
        let e = commands.spawn((ChildOf(root), PointLight { intensity: 0.0, shadow_depth_bias: SHADOW_DEPTH_BIAS, ..default() }, Transform::default(), Visibility::Hidden)).id();
        level.moving.push(e);
    }
    for (n, &e) in level.moving.iter().enumerate() {
        let Some(&i) = order.get(n) else {
            hide(e, &mut placed);
            continue;
        };
        let (at, intensity, colour) = view.lights[i];
        if let Ok((mut t, mut v)) = placed.get_mut(e) {
            t.translation = at;
            *v = Visibility::Inherited;
        }
        if let Ok(mut l) = points.get_mut(e) {
            l.intensity = lumens(intensity);
            l.color = Color::linear_rgb(colour.x, colour.y, colour.z);
            l.range = reach(intensity);
            l.shadow_maps_enabled = n < 4;
        }
    }
    // id's sprites, each a picture facing the eye at full brightness (`SPR_VP_PARALLEL`)
    let facing = Quat::from_mat3(&view.turn());
    for (n, b) in view.sprites.iter().enumerate() {
        let (material, mesh) = level
            .sprites
            .entry(b.picture)
            .or_insert_with(|| {
                let picture = images.add(colour_image(&level.pictures[b.picture as usize]));
                let [left, right, down, up] = b.edges;
                let quad = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
                    .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![[left, down, 0.0], [right, down, 0.0], [right, up, 0.0], [left, up, 0.0]])
                    .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, vec![[0.0, 0.0, 1.0]; 4])
                    .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, vec![[0.0, 1.0], [1.0, 1.0], [1.0, 0.0], [0.0, 0.0]])
                    .with_inserted_indices(Indices::U32(vec![0, 1, 2, 0, 2, 3]));
                let m = StandardMaterial {
                    base_color: Color::BLACK,
                    base_color_texture: Some(picture.clone()),
                    emissive: LinearRgba::rgb(GLOW, GLOW, GLOW),
                    emissive_texture: Some(picture),
                    emissive_exposure_weight: 1.0,
                    reflectance: 0.0,
                    alpha_mode: AlphaMode::Blend,
                    double_sided: true,
                    cull_mode: None,
                    ..default()
                };
                (standard.add(m), meshes.add(quad))
            })
            .clone();
        if level.billboards.len() <= n {
            let e = commands.spawn((ChildOf(root), Mesh3d(mesh.clone()), MeshMaterial3d(material.clone()), NotShadowCaster, Transform::default(), Visibility::Hidden)).id();
            level.billboards.push(e);
        }
        let e = level.billboards[n];
        commands.entity(e).insert((Mesh3d(mesh), MeshMaterial3d(material)));
        if let Ok((mut t, mut v)) = placed.get_mut(e) {
            *t = Transform { translation: b.origin, rotation: facing, ..default() };
            *v = Visibility::Inherited;
        }
    }
    for &e in &level.billboards[view.sprites.len().min(level.billboards.len())..] {
        hide(e, &mut placed);
    }
    // the particles: soft discs facing the eye, lit by the light where they are; those that glow
    // giving off their colour, a material for each colour
    let mut groups: Vec<(Option<[u32; 3]>, Vec<&crate::Dot>)> = Vec::new();
    for d in &view.dots {
        let key = d.glows.then(|| d.colour.to_array().map(f32::to_bits));
        match groups.iter_mut().find(|g| g.0 == key) {
            Some(g) => g.1.push(d),
            None => groups.push((key, vec![d])),
        }
    }
    for (n, (key, dots)) in groups.iter().enumerate() {
        let disc = level.disc.clone();
        let material = level
            .dot_materials
            .entry(*key)
            .or_insert_with(|| {
                let glow = key.map(|c| Vec3::from_array(c.map(f32::from_bits)));
                standard.add(StandardMaterial {
                    base_color: glow.map(|c| { let k = (c * REFLECTANCE).min(Vec3::splat(MOST)); Color::linear_rgb(k.x, k.y, k.z) }).unwrap_or(Color::WHITE),
                    base_color_texture: Some(disc),
                    emissive: glow.map(|c| LinearRgba::from(Color::linear_rgb(c.x, c.y, c.z)) * (2.0 * GLOW)).unwrap_or(LinearRgba::BLACK),
                    emissive_exposure_weight: 1.0,
                    reflectance: 0.0,
                    alpha_mode: AlphaMode::Blend,
                    double_sided: true,
                    cull_mode: None,
                    ..default()
                })
            })
            .clone();
        let (mut p, mut nrm, mut uv, mut col, mut idx) = (Vec::new(), Vec::new(), Vec::new(), Vec::new(), Vec::new());
        for d in dots {
            let half = d.size * 0.5;
            let base = p.len() as u32;
            for (cx, cy) in [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)] {
                p.push((d.place + r * (cx * half) + u * (cy * half)).to_array());
                nrm.push((-f).to_array());
                uv.push([(cx + 1.0) * 0.5, (1.0 - cy) * 0.5]);
                let c = if key.is_some() { Vec3::ONE } else { (d.colour * REFLECTANCE).min(Vec3::splat(MOST)) };
                col.push([c.x, c.y, c.z, 1.0]);
            }
            idx.extend_from_slice(&[base, base + 1, base + 2, base, base + 2, base + 3]);
        }
        let mesh = Mesh::new(PrimitiveTopology::TriangleList, RenderAssetUsages::default())
            .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, p)
            .with_inserted_attribute(Mesh::ATTRIBUTE_NORMAL, nrm)
            .with_inserted_attribute(Mesh::ATTRIBUTE_UV_0, uv)
            .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, col)
            .with_inserted_indices(Indices::U32(idx));
        if level.dots.len() <= n {
            let h = meshes.add(mesh.clone());
            let e = commands.spawn((ChildOf(root), Mesh3d(h.clone()), NotShadowCaster, NoFrustumCulling, Transform::default(), Visibility::Hidden)).id();
            level.dots.push((e, h));
        }
        let (e, h) = level.dots[n].clone();
        if let Some(mut m) = meshes.get_mut(&h) {
            *m = mesh;
        }
        commands.entity(e).insert(MeshMaterial3d(material));
        if let Ok((_, mut v)) = placed.get_mut(e) {
            *v = Visibility::Inherited;
        }
    }
    for (e, _) in &level.dots[groups.len().min(level.dots.len())..] {
        hide(*e, &mut placed);
    }
}
