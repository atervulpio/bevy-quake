//! What the level draws, built from the game's own files: the map's brush models and the
//! models of things (`.mdl`), each turned into triangles for the card.

use data::bsp::Bsp;
use data::mdl::Mdl;
use data::pak::Packs;
use glam::Vec3;
use render::{AliasMesh, BrushMesh, BrushVertex, LevelMeshes, PictureData, PoseVertex, Skin, Slot};
use std::collections::HashMap;

/// Each texture's mean colour, linear: what the light is computed with (row 21), the surfaces
/// taking their own colours from the same textures.
pub fn means(bsp: &Bsp, palette: &data::Palette) -> Vec<Vec3> {
    bsp.textures
        .iter()
        .map(|t| match t {
            Some(t) if !t.pixels.is_empty() => t.pixels.iter().fold(Vec3::ZERO, |s, &i| s + palette.linear(i)) / t.pixels.len() as f32,
            _ => Vec3::splat(0.5),
        })
        .collect()
}

/// What light knows of each texture: its colour, the sky and lava giving off light, liquids letting
/// it through.
pub fn surfaces(bsp: &Bsp, palette: &data::Palette) -> Vec<light::Surface> {
    let means = means(bsp, palette);
    bsp.textures
        .iter()
        .zip(&means)
        .map(|(t, &c)| {
            let name = t.as_ref().map(|t| t.name.as_str()).unwrap_or("");
            let sky = name.starts_with("sky");
            let lava = name.contains("lava");
            // the reflectance is the palette's colour times `REFLECTANCE` (row 22); what the sky
            // and lava give off is as before, the light reaching the eye from them unchanged
            light::Surface { albedo: (c * render::REFLECTANCE).min(Vec3::splat(0.95)), emission: if sky || lava { c * 4.0 } else { Vec3::ZERO }, sky, liquid: name.starts_with('*') }
        })
        .collect()
}

/// The map's light, from where it was kept or computed now and kept (`cache/light/`).
pub fn light(bsp_bytes: &[u8], bsp: &Bsp, palette: &data::Palette, map: &str) -> light::Lightmaps {
    let surfaces = surfaces(bsp, palette);
    let settings = light::Settings::default();
    let key = light::fingerprint(bsp_bytes, &surfaces, &settings);
    let dir = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("../../cache/light");
    let path = dir.join(format!("{map}-{key:016x}.bin"));
    if let Some(lm) = std::fs::read(&path).ok().and_then(|b| light::Lightmaps::from_bytes(&b)) {
        return lm;
    }
    eprintln!("bevy-quake: computing {map}'s light, once (about a minute)…");
    let t = std::time::Instant::now();
    let lm = light::bake(bsp, &surfaces, &settings);
    eprintln!("bevy-quake: {map}'s light computed in {:.0} s", t.elapsed().as_secs_f32());
    let _ = std::fs::create_dir_all(&dir);
    // an older bake of the same map is outdated by this one
    if let Ok(entries) = std::fs::read_dir(&dir) {
        for e in entries.flatten() {
            if e.file_name().to_string_lossy().starts_with(&format!("{map}-")) {
                let _ = std::fs::remove_file(e.path());
            }
        }
    }
    let _ = std::fs::write(&path, lm.to_bytes());
    lm
}

/// A face's light, where its samples stand in the atlas, for the brush meshes.
pub struct Lit<'a> {
    pub maps: &'a light::Lightmaps,
    pub atlas: &'a light::atlas::Atlas,
}

/// A brush model's faces as triangles, each corner with its place on its texture and its place in
/// the light atlas, in samples. With no light given (a pick-up's box from its own file), the face
/// is lit by the probes. `skins` is where the map's textures begin among the level's slots.
pub fn brush_mesh(bsp: &Bsp, model: usize, lit: Option<&Lit>, skins: u32) -> BrushMesh {
    let m = &bsp.models[model];
    let mut vertices = Vec::new();
    let mut indices = Vec::new();
    for fi in m.first_face as usize..(m.first_face + m.num_faces) as usize {
        let f = &bsp.faces[fi];
        let plane = &bsp.planes[f.plane as usize];
        let normal = if f.side != 0 { -plane.normal } else { plane.normal };
        let ti = &bsp.texinfo[f.texinfo as usize];
        let tex = ti.miptex as usize;
        let (tw, th) = match bsp.textures.get(tex) {
            Some(Some(t)) => (t.width as f32, t.height as f32),
            _ => (64.0, 64.0),
        };
        let (sv, tv) = (Vec3::new(ti.s[0], ti.s[1], ti.s[2]), Vec3::new(ti.t[0], ti.t[1], ti.t[2]));
        let base = vertices.len() as u32;
        let points = bsp.face_points(f);
        for p in &points {
            let s = p.dot(sv) + ti.s[3];
            let t = p.dot(tv) + ti.t[3];
            let (light, face) = match lit {
                Some(l) => match l.atlas.places[fi] {
                    Some((px, py)) => {
                        let fl = &l.maps.faces[fi];
                        let x = px as f32 + (s - fl.s0) / l.maps.luxel + 0.5;
                        let y = py as f32 + (t - fl.t0) / l.maps.luxel + 0.5;
                        (glam::Vec2::new(x, y), fi as u32)
                    }
                    None => (glam::Vec2::ZERO, u32::MAX),
                },
                None => (glam::Vec2::ZERO, u32::MAX),
            };
            vertices.push(BrushVertex { position: *p, normal, uv: glam::Vec2::new(s / tw, t / th), light, face, slot: skins + tex as u32 });
        }
        // each triangle turning anticlockwise seen from the side the face faces, Bevy's front:
        // the polygon's own turn (Newell's sum over its edges) against the face's normal
        let turn = points.iter().zip(points.iter().cycle().skip(1)).fold(Vec3::ZERO, |n, (a, b)| n + a.cross(*b));
        let anticlockwise = turn.dot(normal) > 0.0;
        for i in 1..points.len() as u32 - 1 {
            indices.extend_from_slice(&if anticlockwise { [base, base + i, base + i + 1] } else { [base, base + i + 1, base + i] });
        }
    }
    BrushMesh { vertices, indices }
}

/// A map's textures added to the level's pictures: one slot for each texture, its animation's
/// frames and its other animation (`Mod_LoadTextures`: `+0name` to `+9name` in turn, `+aname` to
/// `+jname` the other), each frame what it is drawn as (a surface, the sky, a liquid). A texture
/// the map lacks is a grey picture of one texel. Gives where the map's slots begin.
pub fn add_skins(bsp: &Bsp, palette: &data::Palette, meshes: &mut LevelMeshes) -> u32 {
    let first = meshes.slots.len() as u32;
    let mut skin_of = Vec::with_capacity(bsp.textures.len());
    for t in &bsp.textures {
        let Some(t) = t else {
            meshes.pictures.push(PictureData { width: 1, height: 1, levels: vec![vec![[128, 128, 128, 0]]] });
            skin_of.push(Skin { picture: meshes.pictures.len() as u32 - 1, kind: render::SURFACE });
            continue;
        };
        let n = t.name.to_ascii_lowercase();
        let kind = if n.starts_with("sky") {
            render::SKY
        } else if !n.starts_with('*') {
            render::SURFACE
        } else if n.contains("lava") {
            render::LAVA
        } else if n.contains("slime") {
            render::SLIME
        } else if n.contains("tele") {
            render::TELEPORT
        } else {
            render::WATER
        };
        meshes.pictures.push(native(t.width, t.height, &t.pixels, palette, if kind == render::SKY { Mark::SkyFront } else { Mark::Glow }));
        skin_of.push(Skin { picture: meshes.pictures.len() as u32 - 1, kind });
    }
    // the animations: a texture named `+` and a frame's mark, grouped by the rest of the name
    let names: Vec<Option<&str>> = bsp.textures.iter().map(|t| t.as_ref().map(|t| t.name.as_str())).collect();
    let group = |rest: &str, alternate: bool| -> Vec<Skin> {
        let mut frames: Vec<(u8, Skin)> = names
            .iter()
            .enumerate()
            .filter_map(|(i, n)| {
                let n = (*n)?.as_bytes();
                (n.len() > 2 && n[0] == b'+' && n[2..].eq_ignore_ascii_case(rest.as_bytes())).then_some((n[1].to_ascii_lowercase(), skin_of[i]))
            })
            .filter(|(mark, _)| if alternate { mark.is_ascii_lowercase() } else { mark.is_ascii_digit() })
            .collect();
        frames.sort_by_key(|f| f.0);
        frames.into_iter().map(|f| f.1).collect()
    };
    for (i, n) in names.iter().enumerate() {
        let slot = match n {
            Some(n) if n.starts_with('+') && n.len() > 2 => {
                let (mark, rest) = (n.as_bytes()[1].to_ascii_lowercase(), &n[2..]);
                let (digits, letters) = (group(rest, false), group(rest, true));
                // an alternate's other animation is the first one
                if mark.is_ascii_digit() { Slot { frames: digits, alternate: letters } } else { Slot { frames: letters, alternate: digits } }
            }
            _ => Slot { frames: vec![skin_of[i]], alternate: Vec::new() },
        };
        meshes.slots.push(slot);
    }
    first
}

/// How a model's frames map to its poses, for choosing the pose to show.
pub struct AliasFrames {
    /// For each frame: its first pose and, for a timed group, the time each pose ends at.
    pub frames: Vec<(usize, Vec<f32>)>,
    pub sync_random: bool,
    /// The model's trail and rotation flags (`EF_ROCKET` and the rest).
    pub flags: i32,
}

impl AliasFrames {
    /// The pose a frame shows at a time (`R_AliasSetupFrame`): a group's poses in turn, each for
    /// its interval, the whole repeating. A model marked `ST_RAND` starts its groups at a moment
    /// of its own (`syncbase`), so a row of torches does not flicker in step; here that moment is
    /// drawn from the thing's number.
    pub fn pose_of(&self, frame: usize, time: f32, thing: usize) -> usize {
        let base = if self.sync_random { (thing as f32 * 0.618_034).fract() * 10.0 } else { 0.0 };
        self.pose(frame, time + base)
    }

    pub fn pose(&self, frame: usize, time: f32) -> usize {
        let Some((first, intervals)) = self.frames.get(frame).or(self.frames.first()) else { return 0 };
        if intervals.is_empty() {
            return *first;
        }
        let full = *intervals.last().unwrap();
        let t = time - (time / full).floor() * full;
        first + intervals.iter().position(|&i| i > t).unwrap_or(intervals.len() - 1)
    }
}

/// What a picture's `a` marks (row 39): what id drew at full brightness (the palette's last 32
/// colours) on a texture or a skin; where the sky's front layer is (all but colour nought); where
/// a sprite shows (all but colour 255).
#[derive(Clone, Copy, PartialEq)]
pub enum Mark {
    Glow,
    SkyFront,
    Shown,
}

/// One of id's pictures as the card takes it (row 39): its colours the palette's, each level of
/// detail the mean of four below it in light's own measure, its `a` as `mark` says.
pub fn native(width: u32, height: u32, pixels: &[u8], palette: &data::Palette, mark: Mark) -> PictureData {
    let srgb = |v: f32| (data::linear_to_srgb(v.clamp(0.0, 1.0)) * 255.0).round() as u8;
    let (mut w, mut h) = (width.max(1) as usize, height.max(1) as usize);
    let mut level: Vec<glam::Vec4> = pixels
        .iter()
        .map(|&i| {
            let a = match mark {
                Mark::Glow => i >= 224,
                Mark::SkyFront => i != 0,
                Mark::Shown => i != 255,
            };
            // a sprite's see-through colour takes nothing of the palette's, so its edges stay clean
            let c = if mark == Mark::Shown && i == 255 { glam::Vec3::ZERO } else { palette.linear(i) };
            c.extend(a as u8 as f32)
        })
        .collect();
    let mut levels = Vec::new();
    loop {
        levels.push(level.iter().map(|c| [srgb(c.x), srgb(c.y), srgb(c.z), (c.w.clamp(0.0, 1.0) * 255.0).round() as u8]).collect());
        if w == 1 && h == 1 {
            break;
        }
        let (nw, nh) = ((w / 2).max(1), (h / 2).max(1));
        level = (0..nw * nh)
            .map(|k| {
                let (x, y) = (k % nw, k / nw);
                let at = |dx: usize, dy: usize| level[(y * 2 + dy).min(h - 1) * w + (x * 2 + dx).min(w - 1)];
                (at(0, 0) + at(1, 0) + at(0, 1) + at(1, 1)) * 0.25
            })
            .collect();
        (w, h) = (nw, nh);
    }
    PictureData { width, height, levels }
}

/// A model's corners as drawn, each corner's place on its skin, and all its poses (`.mdl`). A
/// corner on the skin's seam used by a triangle facing back is a corner of its own, on the skin's
/// back half, as the original draws it (`ALIAS_ONSEAM`). Normals are worked out from the pose's
/// own triangles. `skins` are its skins' places among the level's pictures.
pub fn alias_mesh(mdl: &Mdl, skins: Vec<u32>) -> (AliasMesh, AliasFrames) {
    let mut corners: Vec<(u32, bool)> = Vec::new();
    let mut index_of: HashMap<(u32, bool), u32> = HashMap::new();
    let mut indices = Vec::new();
    for t in &mdl.triangles {
        // id's triangles wind clockwise seen from outside; Bevy's front is anticlockwise
        for &v in &[t.verts[0], t.verts[2], t.verts[1]] {
            let back = !t.faces_front && mdl.st[v as usize].on_seam;
            let key = (v, back);
            let i = *index_of.entry(key).or_insert_with(|| {
                corners.push(key);
                corners.len() as u32 - 1
            });
            indices.push(i);
        }
    }
    let (w, h) = (mdl.skin_width.max(1) as f32, mdl.skin_height.max(1) as f32);
    let uvs = corners
        .iter()
        .map(|&(v, back)| {
            let st = &mdl.st[v as usize];
            let s = st.s as f32 + if back { w * 0.5 } else { 0.0 } + 0.5;
            glam::Vec2::new(s / w, (st.t as f32 + 0.5) / h)
        })
        .collect();
    let mut poses = Vec::new();
    let mut frames = Vec::new();
    for f in &mdl.frames {
        frames.push((poses.len(), f.intervals.clone()));
        for p in &f.poses {
            let mut normals = vec![Vec3::ZERO; p.verts.len()];
            for t in &mdl.triangles {
                let [a, b, c] = t.verts.map(|i| i as usize);
                // id's triangles wind clockwise seen from outside
                let n = (p.verts[c] - p.verts[a]).cross(p.verts[b] - p.verts[a]);
                normals[a] += n;
                normals[b] += n;
                normals[c] += n;
            }
            poses.push(
                corners
                    .iter()
                    .map(|&(v, _)| PoseVertex { position: p.verts[v as usize], normal: normals[v as usize].normalize_or_zero() })
                    .collect::<Vec<_>>(),
            );
        }
    }
    (AliasMesh { uvs, skins, indices, poses }, AliasFrames { frames, sync_random: mdl.sync_random, flags: mdl.flags })
}

pub enum MeshRef {
    Brush(usize),
    Alias(usize),
    /// A sprite (`.spr`): each frame's picture among the level's, and where its edges stand from
    /// the sprite's origin (left, right, down, up, `R_DrawSprite`).
    Sprite { frames: Vec<(u32, [f32; 4])> },
    None,
}

impl MeshRef {
    /// A sprite (`MeshRef::Sprite`) at `origin` in frame `frame`, as id draws it: facing the eye.
    pub fn sprite(&self, origin: Vec3, frame: f32) -> Option<render::Billboard> {
        let MeshRef::Sprite { frames } = self else { return None };
        let (picture, edges) = *frames.get((frame.max(0.0) as usize).min(frames.len().saturating_sub(1)))?;
        Some(render::Billboard { origin, picture, edges })
    }
}

/// Every model of the level's precache list made into meshes, in one set for the card.
pub struct Built {
    pub meshes: LevelMeshes,
    /// By the game's model number.
    pub refs: Vec<MeshRef>,
    /// By model number among the models of the game's own (`MeshRef::Alias`).
    pub frames: Vec<AliasFrames>,
    /// Names to model numbers, for the weapon in hand and the frozen scenery.
    pub by_name: HashMap<String, usize>,
}


pub fn build(files: &Packs, world: &Bsp, lit: &Lit, models: &[game::Model], palette: &data::Palette) -> Built {
    let mut meshes = LevelMeshes::default();
    let world_skins = add_skins(world, palette, &mut meshes);
    meshes.light = Some(render::LightData {
        width: lit.atlas.width,
        height: lit.atlas.height,
        layers: lit.atlas.layers.clone(),
        faces: lit.maps.faces.iter().zip(&lit.atlas.places).map(|(f, p)| p.map(|place| render::FaceLight { place, width: f.width, height: f.height, styles: f.styles })).collect(),
        probe_origin: lit.maps.probes.origin,
        probe_spacing: lit.maps.probes.spacing,
        probe_dims: lit.maps.probes.dims,
        probes: std::array::from_fn(|a| lit.maps.probes.cube.iter().map(|c| light::atlas::rgb9e5(c[a])).collect()),
        styled_probes: lit.maps.probes.styled.clone(),
    });
    let mut refs = Vec::new();
    let mut frames = Vec::new();
    let mut by_name = HashMap::new();
    for (i, m) in models.iter().enumerate() {
        by_name.insert(m.name.to_string(), i);
        let r = if let Some(sub) = m.brush {
            meshes.brush.push(brush_mesh(world, sub, Some(lit), world_skins));
            MeshRef::Brush(meshes.brush.len() - 1)
        } else if m.name.ends_with(".bsp") {
            match files.get(m.name).map(Bsp::parse) {
                Some(Ok(b)) => {
                    let skins = add_skins(&b, palette, &mut meshes);
                    meshes.brush.push(brush_mesh(&b, 0, None, skins));
                    MeshRef::Brush(meshes.brush.len() - 1)
                }
                _ => MeshRef::None,
            }
        } else if m.name.ends_with(".mdl") {
            // id's model with id's skins (row 39)
            match files.get(m.name).and_then(|b| Mdl::parse(b).ok()) {
                Some(mdl) => {
                    let mut places = Vec::new();
                    for skin in &mdl.skins {
                        meshes.pictures.push(native(mdl.skin_width, mdl.skin_height, &skin.pictures[0], palette, Mark::Glow));
                        places.push(meshes.pictures.len() as u32 - 1);
                    }
                    let (mesh, f) = alias_mesh(&mdl, places);
                    meshes.alias.push(mesh);
                    frames.push(f);
                    MeshRef::Alias(frames.len() - 1)
                }
                None => MeshRef::None,
            }
        } else if m.name.ends_with(".spr") {
            // id's sprite, each frame's first picture (row 39)
            match files.get(m.name).map(data::spr::Spr::parse) {
                Some(Ok(spr)) => {
                    let mut list = Vec::new();
                    for f in &spr.frames {
                        let Some(pic) = f.pictures.first() else { continue };
                        meshes.pictures.push(native(pic.width, pic.height, &pic.pixels, palette, Mark::Shown));
                        let (left, up) = (pic.origin[0] as f32, pic.origin[1] as f32);
                        list.push((meshes.pictures.len() as u32 - 1, [left, left + pic.width as f32, up - pic.height as f32, up]));
                    }
                    MeshRef::Sprite { frames: list }
                }
                _ => MeshRef::None,
            }
        } else {
            MeshRef::None
        };
        refs.push(r);
    }
    Built { meshes, refs, frames, by_name }
}

/// Where a model is between two of its poses: `t` of the way from `from` to `to`.
#[derive(Clone, Copy)]
pub struct Pose {
    pub from: usize,
    pub to: usize,
    pub t: f32,
}

impl Pose {
    pub fn still(p: usize) -> Pose {
        Pose { from: p, to: p, t: 0.0 }
    }
}

impl Built {
    /// Model `a` (`MeshRef::Alias`) drawn between two poses, wearing its skin `skin`.
    pub fn model(&self, a: usize, transform: glam::Mat4, pose: Pose, skin: usize) -> render::Draw {
        let Pose { from, to, t } = pose;
        render::Draw::Alias { mesh: a, transform, from, to, t, skin }
    }
}
