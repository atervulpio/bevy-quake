//! The light, computed: every surface of a map lit by its lamps, its sky and its lava, by the
//! laws light keeps, and lit again by what the other surfaces throw back.
//!
//! **The units.** Irradiance is kept so that a surface is drawn as its own colour times it:
//! 1 is the light under which id's original showed a texture as painted (lightmap value 128).
//!
//! **A lamp** (`light`, `light_fluoro`, the flames and torches) is a small sphere, 8 units across,
//! whose light falls with the square of the distance (`E = I cos θ / d²`) and is stopped by the
//! world's solids. id's original made light fall in a straight line to nothing at the lamp's
//! value (`light.exe`, `SingleLightFace`: `add = light - dist`, then halved, `rangescale`); the
//! lamp's intensity here is set so the two agree half way out, `I = L³ / 2048`, and so a lamp is
//! brighter near and reaches further, softly, as light does.
//!
//! **A spotlight** (a lamp with a `target`) lights a cone of half-angle `angle / 2` (20° when
//! unset, as id's), its edge softened over 10% of the angle rather than cut.
//!
//! **The sky and lava** are lights of their own, the whole of their surface: what a ray from a
//! surface meets on them is light arriving.
//!
//! **Bounced light**: each surface throws back its colour's share of what reaches it, gathered
//! by rays from every surface, three times over.

pub mod atlas;
pub mod bvh;

use bvh::{Bvh, Tri};
use data::bsp::{Bsp, TEX_SPECIAL};
use glam::{Mat3, Vec3};
use rayon::prelude::*;

/// What a surface is, as far as light cares: the share of light it throws back (its albedo,
/// linear), what it gives off by itself, and whether light passes through it.
#[derive(Clone, Copy)]
pub struct Surface {
    pub albedo: Vec3,
    pub emission: Vec3,
    pub sky: bool,
    pub liquid: bool,
}

#[derive(Clone, Copy)]
pub struct Settings {
    /// World units between two samples of light on a surface (id's original: 16).
    pub luxel: f32,
    /// Rays to each lamp's sphere, for soft shadows.
    pub shadow_rays: u32,
    /// Rays gathering the sky and the bounced light, per sample.
    pub gather_rays: u32,
    pub bounces: u32,
}

impl Default for Settings {
    fn default() -> Self {
        Settings { luxel: 4.0, shadow_rays: 16, gather_rays: 96, bounces: 3 }
    }
}

/// A lamp of the map.
#[derive(Clone, Copy, Debug)]
pub struct Lamp {
    pub origin: Vec3,
    /// Radiant intensity in this crate's units: irradiance at one unit's distance, facing.
    pub intensity: f32,
    pub style: u8,
    /// A spotlight's axis and the cosines of its cone's soft edge (outer, inner).
    pub spot: Option<(Vec3, f32, f32)>,
}

/// The light of one face: its grid of samples in texture space and, for each light style that
/// reaches it (at most four, as the original), where its samples are.
#[derive(Clone, Debug)]
pub struct FaceLight {
    pub width: u32,
    pub height: u32,
    /// The texture-space place of the first sample.
    pub s0: f32,
    pub t0: f32,
    /// The styles reaching the face, 255 for none (as `dface_t.styles`).
    pub styles: [u8; 4],
    /// Where each style's samples begin in `samples`.
    pub offsets: [u32; 4],
}

pub struct Lightmaps {
    pub luxel: f32,
    pub faces: Vec<FaceLight>,
    pub samples: Vec<Vec3>,
    /// For each sample, where its light comes from: the brightness-weighted mean direction
    /// (`xyz`, one long) and how much of the light comes that way (`w`: 1 all from one side,
    /// nought evenly from all round). It lets a surface's relief be lit by baked light.
    pub directions: Vec<glam::Vec4>,
    pub probes: Probes,
}

/// A colour's brightness as the eye weighs it (Rec. 709).
fn luminance(c: Vec3) -> f32 {
    c.dot(Vec3::new(0.2126, 0.7152, 0.0722))
}

/// A sum of directions weighted by brightness, and the brightness, as a mean direction and its
/// share.
fn direction_of(sum: Vec3, light: Vec3) -> glam::Vec4 {
    let l = luminance(light);
    if l <= 0.0 || sum.length_squared() == 0.0 {
        return glam::Vec4::new(0.0, 0.0, 1.0, 0.0);
    }
    sum.normalize().extend((sum.length() / l).clamp(0.0, 1.0))
}

/// The light in the open, for what moves through it (monsters, weapons, pick-ups): a grid of
/// points, each holding the irradiance a surface facing each of the six axes would get there
/// (an ambient cube: +x, −x, +y, −y, +z, −z), from the lamps of style nought, the sky and every
/// surface's bounced light; and, apart, for each other style, its lamps' own light at the points
/// it reaches, to be added as bright as the style is now, as a face's slots are.
#[derive(Clone, Default)]
pub struct Probes {
    pub origin: Vec3,
    pub spacing: f32,
    pub dims: [u32; 3],
    pub cube: Vec<[Vec3; 6]>,
    pub styled: Vec<StyledProbes>,
}

/// One style's lamps' light in the open: the points it reaches (their places in the grid) and its
/// ambient cube at each.
#[derive(Clone, Default)]
pub struct StyledProbes {
    pub style: u8,
    pub points: Vec<u32>,
    pub cube: Vec<[Vec3; 6]>,
}

impl Probes {
    pub fn index(&self, x: u32, y: u32, z: u32) -> usize {
        ((z * self.dims[1] + y) * self.dims[0] + x) as usize
    }
}

/// The six axes of an ambient cube, in its order.
pub const AXES: [Vec3; 6] = [Vec3::X, Vec3::NEG_X, Vec3::Y, Vec3::NEG_Y, Vec3::Z, Vec3::NEG_Z];
/// World units between two probes.
pub const PROBE_SPACING: f32 = 32.0;

/// The lamps of a map's entities, as `light.exe` read them (`ENTITIES.C`): any `light*` class,
/// its `light` value 300 when unset, its `style`, a `target` making it a spotlight.
pub fn lamps(entities: &str) -> Vec<Lamp> {
    let ents = data::entities::parse(entities);
    let get = |e: &data::entities::Entity, k: &str| data::entities::get(e, k).map(str::to_string);
    let vec = |s: Option<String>| {
        let v: Vec<f32> = s.unwrap_or_default().split_whitespace().filter_map(|x| x.parse().ok()).collect();
        if v.len() == 3 { Vec3::new(v[0], v[1], v[2]) } else { Vec3::ZERO }
    };
    let mut out = Vec::new();
    for e in &ents {
        let class = get(e, "classname").unwrap_or_default();
        if !class.starts_with("light") {
            continue;
        }
        let origin = vec(get(e, "origin"));
        let level = e.iter().find(|(k, _)| k.starts_with("light") || k == "_light").and_then(|(_, v)| v.parse::<f32>().ok()).filter(|&v| v != 0.0).unwrap_or(300.0);
        let style: u8 = get(e, "style").and_then(|s| s.parse().ok()).unwrap_or(0);
        let spot = get(e, "target").and_then(|t| ents.iter().find(|o| data::entities::get(o, "targetname") == Some(t.as_str()))).map(|t| {
            let axis = (vec(get(t, "origin")) - origin).normalize_or_zero();
            let angle: f32 = get(e, "angle").and_then(|a| a.parse().ok()).filter(|&a: &f32| a != 0.0).map(|a| a / 2.0).unwrap_or(20.0);
            let outer = (angle * 1.05).to_radians().cos();
            let inner = (angle * 0.95).to_radians().cos();
            (axis, outer, inner)
        });
        out.push(Lamp { origin, intensity: level.abs().powi(3) / 2048.0 * level.signum(), style, spot });
    }
    out
}

/// A face's frame between texture space and the world: from (s, t) to the point on the plane.
struct FaceFrame {
    to_world: Mat3,
    s_off: f32,
    t_off: f32,
    dist: f32,
    normal: Vec3,
    points: Vec<Vec3>,
}

impl FaceFrame {
    fn point(&self, s: f32, t: f32) -> Vec3 {
        self.to_world * Vec3::new(s - self.s_off, t - self.t_off, self.dist)
    }
}

/// The sample points of a face, each pulled inside the polygon if its grid square lies past an
/// edge, so no sample sits in the wall beside it.
fn inside(p: Vec3, f: &FaceFrame) -> Vec3 {
    let n = f.points.len();
    // A face's corners wind either way round its normal, so "into the face" is found from its
    // centre, not assumed from the order.
    let centre = f.points.iter().fold(Vec3::ZERO, |a, &b| a + b) / n as f32;
    let mut q = p;
    for _ in 0..2 {
        let mut moved = false;
        for i in 0..n {
            let a = f.points[i];
            let b = f.points[(i + 1) % n];
            let mut edge_in = f.normal.cross(b - a).normalize_or_zero();
            if edge_in.dot(centre - a) < 0.0 {
                edge_in = -edge_in;
            }
            let d = (q - a).dot(edge_in);
            if d < 0.25 {
                q += edge_in * (0.25 - d);
                moved = true;
            }
        }
        if !moved {
            break;
        }
    }
    q
}

fn hash(mut x: u32) -> u32 {
    x ^= x >> 16;
    x = x.wrapping_mul(0x7feb_352d);
    x ^= x >> 15;
    x = x.wrapping_mul(0x846c_a68b);
    x ^ (x >> 16)
}

/// A low-discrepancy pair for sample `i` of `n` (a jittered grid's cell with a hashed offset).
fn pair(i: u32, n: u32, seed: u32) -> (f32, f32) {
    let side = (n as f32).sqrt().ceil() as u32;
    let (cx, cy) = (i % side, i / side);
    let h = hash(seed ^ i.wrapping_mul(0x9e37_79b9));
    let jx = (h & 0xffff) as f32 / 65536.0;
    let jy = (h >> 16) as f32 / 65536.0;
    ((cx as f32 + jx) / side as f32, (cy as f32 + jy) / side as f32)
}

/// A direction around `n` weighted by the cosine, from a pair in the unit square.
fn cosine_dir(n: Vec3, u: f32, v: f32) -> Vec3 {
    let r = u.sqrt();
    let phi = 2.0 * std::f32::consts::PI * v;
    let (t, b) = n.any_orthonormal_pair();
    (t * (r * phi.cos()) + b * (r * phi.sin()) + n * (1.0 - u).max(0.0).sqrt()).normalize()
}

/// A point on a sphere of radius `r` about `c`, from a pair in the unit square.
fn sphere_point(c: Vec3, r: f32, u: f32, v: f32) -> Vec3 {
    let z = 1.0 - 2.0 * u;
    let s = (1.0 - z * z).max(0.0).sqrt();
    let phi = 2.0 * std::f32::consts::PI * v;
    c + r * Vec3::new(s * phi.cos(), s * phi.sin(), z)
}

/// The lamp's own radius, 4 units: the flame or bulb is not a point, so shadows have a penumbra.
const LAMP_RADIUS: f32 = 4.0;
/// Light below this irradiance is not followed (a lamp's reach ends where it adds under 1/500
/// of a texture's painted brightness).
const DIM: f32 = 0.002;

pub fn bake(bsp: &Bsp, surfaces: &[Surface], settings: &Settings) -> Lightmaps {
    let step = settings.luxel;
    let surface_of = |face: usize| -> Surface {
        let ti = &bsp.texinfo[bsp.faces[face].texinfo as usize];
        surfaces.get(ti.miptex as usize).copied().unwrap_or(Surface { albedo: Vec3::splat(0.3), emission: Vec3::ZERO, sky: false, liquid: false })
    };

    // The world's triangles: what light meets. Only the world itself casts shadows, as in
    // `light.exe`; doors and lifts are lit but throw no shadow, since they move.
    let world = &bsp.models[0];
    let mut tris = Vec::new();
    for fi in world.first_face as usize..(world.first_face + world.num_faces) as usize {
        let pts = bsp.face_points(&bsp.faces[fi]);
        let s = surface_of(fi);
        for i in 1..pts.len() - 1 {
            tris.push(Tri { a: pts[0], e1: pts[i] - pts[0], e2: pts[i + 1] - pts[0], face: fi as u32, blocks: !s.liquid });
        }
    }
    let bvh = Bvh::build(tris);
    let lamps = lamps(&bsp.entities);

    // Every face's grid of samples.
    let frames: Vec<FaceFrame> = bsp
        .faces
        .iter()
        .map(|f| {
            let plane = &bsp.planes[f.plane as usize];
            let (normal, dist) = if f.side != 0 { (-plane.normal, -plane.dist) } else { (plane.normal, plane.dist) };
            let ti = &bsp.texinfo[f.texinfo as usize];
            let sv = Vec3::new(ti.s[0], ti.s[1], ti.s[2]);
            let tv = Vec3::new(ti.t[0], ti.t[1], ti.t[2]);
            let m = Mat3::from_cols(sv, tv, normal).transpose();
            FaceFrame { to_world: m.inverse(), s_off: ti.s[3], t_off: ti.t[3], dist, normal, points: bsp.face_points(f) }
        })
        .collect();
    let mut faces = Vec::with_capacity(bsp.faces.len());
    let mut points: Vec<(u32, Vec3)> = Vec::new(); // (face, sample point) for every sample
    for (fi, f) in bsp.faces.iter().enumerate() {
        let ti = &bsp.texinfo[f.texinfo as usize];
        let fr = &frames[fi];
        let special = ti.flags & TEX_SPECIAL != 0;
        if special {
            faces.push(FaceLight { width: 0, height: 0, s0: 0.0, t0: 0.0, styles: [255; 4], offsets: [0; 4] });
            continue;
        }
        let (mut smin, mut smax, mut tmin, mut tmax) = (f32::MAX, f32::MIN, f32::MAX, f32::MIN);
        for p in &fr.points {
            let s = p.dot(Vec3::new(ti.s[0], ti.s[1], ti.s[2])) + ti.s[3];
            let t = p.dot(Vec3::new(ti.t[0], ti.t[1], ti.t[2])) + ti.t[3];
            smin = smin.min(s);
            smax = smax.max(s);
            tmin = tmin.min(t);
            tmax = tmax.max(t);
        }
        let s0 = (smin / step).floor() * step;
        let t0 = (tmin / step).floor() * step;
        let width = ((smax / step).ceil() - (smin / step).floor()) as u32 + 1;
        let height = ((tmax / step).ceil() - (tmin / step).floor()) as u32 + 1;
        let first = points.len() as u32;
        for j in 0..height {
            for i in 0..width {
                let p = inside(fr.point(s0 + i as f32 * step, t0 + j as f32 * step), fr);
                points.push((fi as u32, p));
            }
        }
        faces.push(FaceLight { width, height, s0, t0, styles: [255; 4], offsets: [first, 0, 0, 0] });
    }

    // Direct light, per style: each lamp's share at each sample.
    let mut styles_present: Vec<u8> = lamps.iter().map(|l| l.style).collect();
    styles_present.sort_unstable();
    styles_present.dedup();
    let direct_both: Vec<Vec<(Vec3, Vec3)>> = styles_present
        .iter()
        .map(|&style| {
            let mine: Vec<&Lamp> = lamps.iter().filter(|l| l.style == style).collect();
            points
                .par_iter()
                .enumerate()
                .map(|(k, &(fi, p))| {
                    let n = frames[fi as usize].normal;
                    let o = p + n * 0.5;
                    let mut e = 0.0f32;
                    let mut d = Vec3::ZERO;
                    for (li, l) in mine.iter().enumerate() {
                        let to = l.origin - o;
                        let d2 = to.length_squared();
                        let cos = to.dot(n) / d2.sqrt();
                        if cos <= 0.0 || l.intensity * cos / d2 < DIM * 0.25 {
                            continue;
                        }
                        let mut seen = 0;
                        for r in 0..settings.shadow_rays {
                            let (u, v) = pair(r, settings.shadow_rays, (k as u32).wrapping_mul(7919) ^ li as u32);
                            let target = sphere_point(l.origin, LAMP_RADIUS, u, v);
                            let dir = target - o;
                            let len = dir.length();
                            if !bvh.occluded(o, dir / len, len - LAMP_RADIUS * 0.5) {
                                seen += 1;
                            }
                        }
                        if seen == 0 {
                            continue;
                        }
                        let mut cone = 1.0;
                        if let Some((axis, outer, inner)) = l.spot {
                            let c = (-to / d2.sqrt()).dot(axis);
                            cone = ((c - outer) / (inner - outer)).clamp(0.0, 1.0);
                            cone = cone * cone * (3.0 - 2.0 * cone);
                        }
                        let add = l.intensity * cos / d2 * cone * seen as f32 / settings.shadow_rays as f32;
                        e += add;
                        d += to / d2.sqrt() * add;
                    }
                    (Vec3::splat(e), d)
                })
                .collect()
        })
        .collect();
    let direct: Vec<Vec<Vec3>> = direct_both.iter().map(|v| v.iter().map(|x| x.0).collect()).collect();
    let direct_dirs: Vec<Vec<Vec3>> = direct_both.iter().map(|v| v.iter().map(|x| x.1).collect()).collect();
    drop(direct_both);

    // The sky and lava, and the bounces, for style 0's light: gathered by rays from each sample.
    // A ray that meets the sky or lava brings its glow; one that meets another surface brings that
    // surface's colour times the light it had in the pass before.
    let base = styles_present.iter().position(|&s| s == 0);
    let zero = vec![Vec3::ZERO; points.len()];
    let direct0 = base.map(|i| &direct[i]).unwrap_or(&zero);
    let face_sample = |fi: usize, p: Vec3| -> usize {
        let f = &faces[fi];
        let ti = &bsp.texinfo[bsp.faces[fi].texinfo as usize];
        let s = p.dot(Vec3::new(ti.s[0], ti.s[1], ti.s[2])) + ti.s[3];
        let t = p.dot(Vec3::new(ti.t[0], ti.t[1], ti.t[2])) + ti.t[3];
        let i = (((s - f.s0) / step).round() as i64).clamp(0, f.width as i64 - 1) as u32;
        let j = (((t - f.t0) / step).round() as i64).clamp(0, f.height as i64 - 1) as u32;
        (f.offsets[0] + j * f.width + i) as usize
    };
    let mut total = direct0.clone();
    let mut total_dir = base.map(|i| direct_dirs[i].clone()).unwrap_or_else(|| vec![Vec3::ZERO; points.len()]);
    let mut previous = direct0.clone();
    for bounce in 0..settings.bounces.max(1) {
        let gathered_both: Vec<(Vec3, Vec3)> = points
            .par_iter()
            .enumerate()
            .map(|(k, &(fi, p))| {
                let n = frames[fi as usize].normal;
                let o = p + n * 0.5;
                let mut sum = Vec3::ZERO;
                let mut dsum = Vec3::ZERO;
                for r in 0..settings.gather_rays {
                    let (u, v) = pair(r, settings.gather_rays, (k as u32).wrapping_mul(2654435761) ^ (bounce << 24));
                    let dir = cosine_dir(n, u, v);
                    let Some(h) = bvh.closest(o, dir, 16384.0) else { continue };
                    let hf = bvh.tris[h.tri as usize].face as usize;
                    let s = surface_of(hf);
                    let came = if s.sky || s.emission != Vec3::ZERO {
                        if bounce == 0 { s.emission } else { Vec3::ZERO }
                    } else if frames[hf].normal.dot(dir) >= 0.0 || faces[hf].width == 0 {
                        Vec3::ZERO // the back of a face: inside a wall
                    } else {
                        s.albedo * previous[face_sample(hf, o + dir * h.t)]
                    };
                    sum += came;
                    // cosine-weighted rays: each ray's light counts once, so its direction is
                    // weighted by its brightness alone
                    dsum += dir * luminance(came);
                }
                (sum / settings.gather_rays as f32, dsum / settings.gather_rays as f32)
            })
            .collect();
        let gathered: Vec<Vec3> = gathered_both.iter().map(|x| x.0).collect();
        for (i, (t, g)) in total.iter_mut().zip(&gathered).enumerate() {
            *t += *g;
            total_dir[i] += gathered_both[i].1;
        }
        // the next bounce gathers only what this one brought
        previous = gathered;
    }

    // Lay the styles out: style 0 (direct, sky and bounces) first, then each other style that
    // reaches the face, at most four, as the original allows.
    let mut samples = Vec::new();
    let mut directions = Vec::new();
    for f in faces.iter_mut() {
        if f.width == 0 {
            continue;
        }
        let n = (f.width * f.height) as usize;
        let first = f.offsets[0] as usize;
        let mut slot = 0;
        for (si, &style) in styles_present.iter().enumerate() {
            if slot == 4 {
                break;
            }
            let src: &[Vec3] = if style == 0 { &total[first..first + n] } else { &direct[si][first..first + n] };
            if style != 0 && src.iter().all(|v| v.max_element() < DIM) {
                continue;
            }
            let dirs: &[Vec3] = if style == 0 { &total_dir[first..first + n] } else { &direct_dirs[si][first..first + n] };
            f.styles[slot] = style;
            f.offsets[slot] = samples.len() as u32;
            samples.extend_from_slice(src);
            directions.extend(src.iter().zip(dirs).map(|(&l, &d)| direction_of(d, l)));
            slot += 1;
        }
        if style_count(f) == 0 {
            // no style at all reached it (no lamps in the map): it keeps a dark style 0
            f.styles[0] = 0;
            f.offsets[0] = samples.len() as u32;
            samples.extend(std::iter::repeat_n(Vec3::ZERO, n));
            directions.extend(std::iter::repeat_n(glam::Vec4::new(0.0, 0.0, 1.0, 0.0), n));
        }
    }
    let probes = bake_probes(bsp, &bvh, &lamps, &surface_of, &faces, &samples, step, settings);
    Lightmaps { luxel: step, faces, samples, directions, probes }
}

/// The probe grid over the map's box. Each probe in the open gathers, for each axis, rays over
/// that axis's half of the sphere weighted by the cosine (what a surface facing it would get):
/// a ray meeting a surface brings its colour times its computed light, one meeting the sky or
/// lava its glow; and each lamp of style 0 it sees adds `I cos θ / d²`. A probe inside a solid
/// takes the mean of its open neighbours, so blending between probes never reaches into a wall's
/// darkness.
#[allow(clippy::too_many_arguments)]
fn bake_probes(bsp: &Bsp, bvh: &Bvh, lamps: &[Lamp], surface_of: &(dyn Fn(usize) -> Surface + Sync), faces: &[FaceLight], samples: &[Vec3], step: f32, settings: &Settings) -> Probes {
    let m = &bsp.models[0];
    let origin = (m.mins / PROBE_SPACING).floor() * PROBE_SPACING;
    let dims = (((m.maxs - origin) / PROBE_SPACING).ceil() + Vec3::ONE).as_uvec3().to_array();
    let total = (dims[0] * dims[1] * dims[2]) as usize;
    let world = data_point_contents(bsp);
    let sample_at = |fi: usize, p: Vec3| -> Vec3 {
        let f = &faces[fi];
        if f.width == 0 || f.styles[0] != 0 {
            return Vec3::ZERO;
        }
        let ti = &bsp.texinfo[bsp.faces[fi].texinfo as usize];
        let s = p.dot(Vec3::new(ti.s[0], ti.s[1], ti.s[2])) + ti.s[3];
        let t = p.dot(Vec3::new(ti.t[0], ti.t[1], ti.t[2])) + ti.t[3];
        let i = (((s - f.s0) / step).round() as i64).clamp(0, f.width as i64 - 1) as u32;
        let j = (((t - f.t0) / step).round() as i64).clamp(0, f.height as i64 - 1) as u32;
        samples[(f.offsets[0] + j * f.width + i) as usize]
    };
    let rays = (settings.gather_rays / 3).max(8);
    let mut cube: Vec<Option<[Vec3; 6]>> = (0..total)
        .into_par_iter()
        .map(|k| {
            let k = k as u32;
            let (x, y, z) = (k % dims[0], (k / dims[0]) % dims[1], k / (dims[0] * dims[1]));
            let p = origin + Vec3::new(x as f32, y as f32, z as f32) * PROBE_SPACING;
            if world(p) == data::bsp::CONTENTS_SOLID {
                return None;
            }
            let mut c = [Vec3::ZERO; 6];
            for (a, axis) in AXES.iter().enumerate() {
                let mut sum = Vec3::ZERO;
                for r in 0..rays {
                    let (u, v) = pair(r, rays, k.wrapping_mul(2246822519) ^ (a as u32) << 28);
                    let dir = cosine_dir(*axis, u, v);
                    let Some(h) = bvh.closest(p, dir, 16384.0) else { continue };
                    let hf = bvh.tris[h.tri as usize].face as usize;
                    let s = surface_of(hf);
                    if s.sky || s.emission != Vec3::ZERO {
                        sum += s.emission;
                    } else {
                        sum += s.albedo * sample_at(hf, p + dir * h.t);
                    }
                }
                c[a] = sum / rays as f32;
            }
            lamps_at(p, lamps.iter().filter(|l| l.style == 0), bvh, &mut c);
            Some(c)
        })
        .collect();
    // a probe in a solid takes its open neighbours' mean, twice over
    for _ in 0..2 {
        let before = cube.clone();
        for k in 0..total {
            if before[k].is_some() {
                continue;
            }
            let k = k as u32;
            let (x, y, z) = (k % dims[0], (k / dims[0]) % dims[1], k / (dims[0] * dims[1]));
            let mut acc = [Vec3::ZERO; 6];
            let mut n = 0;
            for (dx, dy, dz) in [(-1i32, 0i32, 0i32), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)] {
                let (nx, ny, nz) = (x as i32 + dx, y as i32 + dy, z as i32 + dz);
                if nx < 0 || ny < 0 || nz < 0 || nx >= dims[0] as i32 || ny >= dims[1] as i32 || nz >= dims[2] as i32 {
                    continue;
                }
                if let Some(c) = before[((nz as u32 * dims[1] + ny as u32) * dims[0] + nx as u32) as usize] {
                    for a in 0..6 {
                        acc[a] += c[a];
                    }
                    n += 1;
                }
            }
            if n > 0 {
                cube[k as usize] = Some(acc.map(|v| v / n as f32));
            }
        }
    }
    // each other style's lamps at every open point they reach (a probe in a solid takes none: its
    // neighbours' light, filled in above, is style nought's alone)
    let mut styles: Vec<u8> = lamps.iter().map(|l| l.style).filter(|&s| s != 0).collect();
    styles.sort_unstable();
    styles.dedup();
    let open: Vec<u32> = (0..total as u32).filter(|&k| world(probe_place(origin, dims, k)) != data::bsp::CONTENTS_SOLID).collect();
    let styled = styles
        .into_iter()
        .map(|style| {
            let reached: Vec<(u32, [Vec3; 6])> = open
                .par_iter()
                .filter_map(|&k| {
                    let mut c = [Vec3::ZERO; 6];
                    lamps_at(probe_place(origin, dims, k), lamps.iter().filter(|l| l.style == style), bvh, &mut c);
                    c.iter().any(|v| v.max_element() > 0.0).then_some((k, c))
                })
                .collect();
            let (points, cube) = reached.into_iter().unzip();
            StyledProbes { style, points, cube }
        })
        .collect();
    Probes { origin, spacing: PROBE_SPACING, dims, cube: cube.into_iter().map(|c| c.unwrap_or([Vec3::ZERO; 6])).collect(), styled }
}

/// Probe `k`'s place in the world.
fn probe_place(origin: Vec3, dims: [u32; 3], k: u32) -> Vec3 {
    let (x, y, z) = (k % dims[0], (k / dims[0]) % dims[1], k / (dims[0] * dims[1]));
    origin + Vec3::new(x as f32, y as f32, z as f32) * PROBE_SPACING
}

/// The lamps' direct light at `p` added to its ambient cube: each lamp it sees past the world's
/// solids gives each axis facing it `I cos θ / d²`, in its cone.
fn lamps_at<'a>(p: Vec3, lamps: impl Iterator<Item = &'a Lamp>, bvh: &Bvh, c: &mut [Vec3; 6]) {
    for l in lamps {
        let to = l.origin - p;
        let d2 = to.length_squared().max(LAMP_RADIUS * LAMP_RADIUS);
        let d = d2.sqrt();
        if l.intensity / d2 < DIM * 0.25 || bvh.occluded(p, to / d, d - LAMP_RADIUS) {
            continue;
        }
        let mut cone = 1.0;
        if let Some((axis, outer, inner)) = l.spot {
            let cc = (-to / d).dot(axis);
            cone = ((cc - outer) / (inner - outer)).clamp(0.0, 1.0);
            cone = cone * cone * (3.0 - 2.0 * cone);
        }
        for (a, axis) in AXES.iter().enumerate() {
            let cos = (to / d).dot(*axis);
            if cos > 0.0 {
                c[a] += Vec3::splat(l.intensity * cos / d2 * cone);
            }
        }
    }
}

/// The world's contents at a point, from the drawing tree (`Mod_PointInLeaf`).
fn data_point_contents(bsp: &Bsp) -> impl Fn(Vec3) -> i32 + Sync + '_ {
    move |p: Vec3| {
        let mut n = 0i32;
        loop {
            let node = &bsp.nodes[n as usize];
            let pl = &bsp.planes[node.plane as usize];
            let d = pl.normal.dot(p) - pl.dist;
            n = if d > 0.0 { node.children[0] } else { node.children[1] };
            if n < 0 {
                return bsp.leafs[(-1 - n) as usize].contents;
            }
        }
    }
}

fn style_count(f: &FaceLight) -> usize {
    f.styles.iter().filter(|&&s| s != 255).count()
}

/// A number that changes with anything a bake depends on (FNV-1a), to name its kept copy.
pub fn fingerprint(bsp_bytes: &[u8], surfaces: &[Surface], settings: &Settings) -> u64 {
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    let mut eat = |b: &[u8]| {
        for &x in b {
            h = (h ^ x as u64).wrapping_mul(0x0000_0100_0000_01b3);
        }
    };
    eat(bsp_bytes);
    for s in surfaces {
        for v in [s.albedo, s.emission] {
            for x in v.to_array() {
                eat(&x.to_le_bytes());
            }
        }
        eat(&[s.sky as u8, s.liquid as u8]);
    }
    for x in [settings.luxel, settings.shadow_rays as f32, settings.gather_rays as f32, settings.bounces as f32] {
        eat(&x.to_le_bytes());
    }
    // the bake's own version: raised whenever how light is computed changes (5: each style's
    // lamps in the probes, row 41)
    eat(&5u32.to_le_bytes());
    h
}

impl Lightmaps {
    /// The bake written as it is: its step, then each face, then the samples, little-endian.
    pub fn to_bytes(&self) -> Vec<u8> {
        let mut out = Vec::new();
        out.extend_from_slice(b"RQLM");
        out.extend_from_slice(&self.luxel.to_le_bytes());
        out.extend_from_slice(&(self.faces.len() as u32).to_le_bytes());
        for f in &self.faces {
            for x in [f.width, f.height] {
                out.extend_from_slice(&x.to_le_bytes());
            }
            for x in [f.s0, f.t0] {
                out.extend_from_slice(&x.to_le_bytes());
            }
            out.extend_from_slice(&f.styles);
            for x in f.offsets {
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
        out.extend_from_slice(&(self.samples.len() as u32).to_le_bytes());
        for s in &self.samples {
            for x in s.to_array() {
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
        for d in &self.directions {
            for x in d.to_array() {
                out.extend_from_slice(&x.to_le_bytes());
            }
        }
        let p = &self.probes;
        for x in p.origin.to_array() {
            out.extend_from_slice(&x.to_le_bytes());
        }
        out.extend_from_slice(&p.spacing.to_le_bytes());
        for d in p.dims {
            out.extend_from_slice(&d.to_le_bytes());
        }
        let cube = |out: &mut Vec<u8>, c: &[Vec3; 6]| {
            for v in c {
                for x in v.to_array() {
                    out.extend_from_slice(&x.to_le_bytes());
                }
            }
        };
        for c in &p.cube {
            cube(&mut out, c);
        }
        out.extend_from_slice(&(p.styled.len() as u32).to_le_bytes());
        for s in &p.styled {
            out.push(s.style);
            out.extend_from_slice(&(s.points.len() as u32).to_le_bytes());
            for (k, c) in s.points.iter().zip(&s.cube) {
                out.extend_from_slice(&k.to_le_bytes());
                cube(&mut out, c);
            }
        }
        out
    }

    pub fn from_bytes(b: &[u8]) -> Option<Lightmaps> {
        let mut at = 0;
        let mut take = |n: usize| -> Option<&[u8]> {
            let s = b.get(at..at + n)?;
            at += n;
            Some(s)
        };
        if take(4)? != b"RQLM" {
            return None;
        }
        let f32_ = |s: &[u8]| f32::from_le_bytes(s.try_into().unwrap());
        let u32_ = |s: &[u8]| u32::from_le_bytes(s.try_into().unwrap());
        let luxel = f32_(take(4)?);
        let n = u32_(take(4)?) as usize;
        let mut faces = Vec::with_capacity(n);
        for _ in 0..n {
            let width = u32_(take(4)?);
            let height = u32_(take(4)?);
            let s0 = f32_(take(4)?);
            let t0 = f32_(take(4)?);
            let styles: [u8; 4] = take(4)?.try_into().unwrap();
            let mut offsets = [0u32; 4];
            for o in &mut offsets {
                *o = u32_(take(4)?);
            }
            faces.push(FaceLight { width, height, s0, t0, styles, offsets });
        }
        let m = u32_(take(4)?) as usize;
        let mut samples = Vec::with_capacity(m);
        for _ in 0..m {
            samples.push(Vec3::new(f32_(take(4)?), f32_(take(4)?), f32_(take(4)?)));
        }
        let mut directions = Vec::with_capacity(m);
        for _ in 0..m {
            directions.push(glam::Vec4::new(f32_(take(4)?), f32_(take(4)?), f32_(take(4)?), f32_(take(4)?)));
        }
        let origin = Vec3::new(f32_(take(4)?), f32_(take(4)?), f32_(take(4)?));
        let spacing = f32_(take(4)?);
        let dims = [u32_(take(4)?), u32_(take(4)?), u32_(take(4)?)];
        let count = (dims[0] * dims[1] * dims[2]) as usize;
        // an ambient cube: six colours, each three floats
        macro_rules! read_cube {
            () => {{
                let mut c = [Vec3::ZERO; 6];
                for v in &mut c {
                    *v = Vec3::new(f32_(take(4)?), f32_(take(4)?), f32_(take(4)?));
                }
                c
            }};
        }
        let mut cube = Vec::with_capacity(count);
        for _ in 0..count {
            cube.push(read_cube!());
        }
        let mut styled = Vec::new();
        for _ in 0..u32_(take(4)?) {
            let style = take(1)?[0];
            let n = u32_(take(4)?) as usize;
            let (mut points, mut cubes) = (Vec::with_capacity(n), Vec::with_capacity(n));
            for _ in 0..n {
                points.push(u32_(take(4)?));
                cubes.push(read_cube!());
            }
            styled.push(StyledProbes { style, points, cube: cubes });
        }
        Some(Lightmaps { luxel, faces, samples, directions, probes: Probes { origin, spacing, dims, cube, styled } })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn a_lamp_agrees_with_id_half_way_out() {
        // id: (L - d) * 0.5 lightmap units at d, 128 being 1 here
        let l = 300.0f32;
        let lamp = lamps("{\n\"classname\" \"light\"\n\"origin\" \"0 0 0\"\n}\n")[0];
        assert_eq!(lamp.intensity, l.powi(3) / 2048.0);
        let d = l / 2.0;
        let ours = lamp.intensity / (d * d);
        let ids = (l - d) * 0.5 / 128.0;
        assert!((ours - ids).abs() < 1e-4, "{ours} {ids}");
    }
}
