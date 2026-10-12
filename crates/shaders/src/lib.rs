//! The programs of ours the graphics card runs, written in Rust and compiled to SPIR-V by rust-gpu
//! (row 41: Bevy draws, and these are what it does not draw itself). Two surfaces of id's that are
//! no material's, the sky and the liquids, worked out at every pixel inside Bevy's own pass; and
//! after Bevy's pass, the game's tone curve and the colour laid over the screen, the screen's
//! letters and the old monitor. The structures the host fills are defined here once, so both sides read one layout.
#![no_std]
// a program's every input and binding is an argument of its function
#![allow(clippy::too_many_arguments)]

use spirv_std::glam::{IVec2, UVec4, Vec2, Vec3, Vec4, Vec4Swizzles, vec2, vec3, vec4};
use spirv_std::{Image, Sampler, spirv};
#[cfg(target_arch = "spirv")]
use spirv_std::num_traits::Float;

pub type Picture = Image!(2D, type = f32, sampled);

/// A place in Bevy's world (y up) in the game's own (z up): Bevy's world is the game's turned a
/// quarter about x, so the game's `y` is Bevy's `−z` and its `z` Bevy's `y`.
fn game_place(bevy: Vec3) -> Vec3 {
    vec3(bevy.x, -bevy.z, bevy.y)
}

/// Where the sky's two layers are seen along `dir` at `time` (`R_DrawSkyChain`, row 39): the
/// direction flattened toward a low dome (`z` three times), brought to six times 63 units long, the
/// back layer drifting eight units a second and the front sixteen, a layer 128 units across; each
/// place in its half of id's sky texture (the front the left half, the back the right).
fn sky_places(dir: Vec3, time: f32) -> (Vec2, Vec2) {
    let d = vec3(dir.x, dir.y, dir.z * 3.0);
    let k = 6.0 * 63.0 / d.length().max(1.0e-6);
    let (dx, dy) = (d.x * k, d.y * k);
    let layer = |speed: f32| {
        let shift = time * speed;
        let s = (shift + dx) / 128.0;
        let t = (shift + dy) / 128.0;
        vec2(s - s.floor(), t - t.floor())
    };
    let front = layer(16.0);
    let back = layer(8.0);
    (vec2(front.x * 0.5, front.y), vec2(0.5 + back.x * 0.5, back.y))
}

/// A liquid's place on its texture warped as id warps it (`EmitWaterPolys`, row 39): each way
/// moved by eight texels of a sine of the other way's place an eighth over and the time, in a
/// texture of 64 texels.
fn liquid_place(uv: Vec2, time: f32) -> Vec2 {
    vec2(uv.x + 0.125 * (uv.y * 8.0 + time).sin(), uv.y + 0.125 * (uv.x * 8.0 + time).sin())
}

/// The sky, in Bevy's pass, its bindings in the group Bevy gives a material's own (3,
/// `MATERIAL_BIND_GROUP_INDEX`): its inputs are the first of what Bevy's mesh vertex program
/// gives (`VertexOutput`: the place in the world at 0, the normal at 1). `eye` is where the eye stands
/// in the game's own terms and the time (`w`); `light.x` how bright the picture is drawn, the
/// sky's own glow and the camera's exposure together, since what a material writes is the light
/// reaching the camera after its exposure, as Bevy's own materials write it.
#[spirv(fragment)]
pub fn sky_fs(
    world: Vec4,
    _normal: Vec3,
    #[spirv(uniform, descriptor_set = 3, binding = 0)] eye: &Vec4,
    #[spirv(uniform, descriptor_set = 3, binding = 1)] light: &Vec4,
    #[spirv(descriptor_set = 3, binding = 2)] picture: &Picture,
    #[spirv(descriptor_set = 3, binding = 3)] repeat: &Sampler,
    out: &mut Vec4,
) {
    // only the direction counts, and `sky_places` brings it to its own length; glam's
    // `normalize_or_zero` would bring in an infinite constant wgpu's translator refuses
    let dir = game_place(world.xyz()) - eye.xyz();
    let (front, back) = sky_places(dir, eye.w);
    let f: Vec4 = picture.sample_by_lod(*repeat, front, 0.0);
    let b: Vec4 = picture.sample_by_lod(*repeat, back, 0.0);
    // the front layer's colour nought is where the back shows through (`a` nought there)
    let c = if f.w > 0.5 { f.xyz() } else { b.xyz() };
    *out = (c * light.x).extend(1.0);
}

/// A liquid (water, slime, lava, a teleporter), in Bevy's pass: its texture warped, giving off its
/// own light as the sky does. Its inputs: the place in the world, the normal, the place on the
/// texture (2).
#[spirv(fragment)]
pub fn liquid_fs(
    _world: Vec4,
    _normal: Vec3,
    uv: Vec2,
    #[spirv(uniform, descriptor_set = 3, binding = 0)] eye: &Vec4,
    #[spirv(uniform, descriptor_set = 3, binding = 1)] light: &Vec4,
    #[spirv(descriptor_set = 3, binding = 2)] picture: &Picture,
    #[spirv(descriptor_set = 3, binding = 3)] repeat: &Sampler,
    out: &mut Vec4,
) {
    let c: Vec4 = picture.sample(*repeat, liquid_place(uv, eye.w));
    *out = (c.xyz() * light.x).extend(1.0);
}

/// What the passes after Bevy's own read: the screen's size in pixels (`x`, `y`) and the
/// glass's glow picture's (`z`, `w`), and the colour laid over the screen and how much of it.
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Frame {
    pub screen: Vec4,
    pub blend: Vec4,
}

/// The whole screen covered by one triangle, for passes that work on every pixel.
#[spirv(vertex)]
pub fn screen_vs(#[spirv(vertex_index)] index: i32, #[spirv(position)] out: &mut Vec4) {
    let u = ((index << 1) & 2) as f32;
    let v = (index & 2) as f32;
    *out = vec4(u * 2.0 - 1.0, 1.0 - v * 2.0, 0.0, 1.0);
}

/// The ACES filmic curve as Narkowicz fitted it: light of any brightness brought to the screen's
/// range with a film's soft shoulder, so a lamp's glare rolls off instead of clipping. The game's
/// own curve since the light was first computed, kept under Bevy (row 41): Bevy's own ACES fit
/// (Hill's) is two and a half to three times darker at the brightness most of the game is seen at.
fn aces(x: Vec3) -> Vec3 {
    let a = x * (x * 2.51 + Vec3::splat(0.03));
    let b = x * (x * 2.43 + Vec3::splat(0.59)) + Vec3::splat(0.14);
    (a / b).clamp(Vec3::ZERO, Vec3::ONE)
}

/// The light Bevy drew, already exposed, brought to the screen through the game's tone curve, and
/// the screen's colour (a hit's red, water's brown, a pick-up's gold) laid over it as the original
/// laid it over its palette.
#[spirv(fragment)]
pub fn overlay_fs(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] frame: &Frame,
    #[spirv(descriptor_set = 0, binding = 1)] picture: &Picture,
    out: &mut Vec4,
) {
    let c: Vec4 = picture.fetch(IVec2::new(coord.x as i32, coord.y as i32));
    let a = frame.blend.w;
    *out = (aces(c.xyz()) * (1.0 - a) + frame.blend.xyz() * a).extend(1.0);
}

/// How many of the screen's pixels each way one pixel of the glass's glow picture covers.
pub const GLOW_STEP: u32 = 8;

/// The glow picture: each of its pixels the mean of the `GLOW_STEP × GLOW_STEP` screen pixels it
/// covers, sixteen filtered samples each the mean of four.
#[spirv(fragment)]
pub fn glow_down_fs(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] frame: &Frame,
    #[spirv(descriptor_set = 0, binding = 1)] picture: &Picture,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    out: &mut Vec4,
) {
    let step = GLOW_STEP as f32;
    let corner = (vec2(coord.x, coord.y) - Vec2::splat(0.5)) * step;
    let size = vec2(frame.screen.x, frame.screen.y);
    let mut sum = Vec3::ZERO;
    let mut j = 0;
    while j < 4 {
        let mut i = 0;
        while i < 4 {
            let p = corner + vec2(1.0 + 2.0 * i as f32, 1.0 + 2.0 * j as f32);
            let c: Vec4 = picture.sample_by_lod(*sampler, p / size, 0.0);
            sum += c.xyz();
            i += 1;
        }
        j += 1;
    }
    *out = (sum / 16.0).extend(1.0);
}

/// How far the glass spreads the light it scatters, as a share of the screen's height: the
/// standard deviation of the halo. A faceplate about 12 mm thick sends light caught inside it
/// out again within about twice its thickness of where it entered; on a tube some 300 mm high
/// that is a standard deviation near 1.5% of the height. The model's own estimate from the glass's
/// geometry, not a measurement.
const HALATION_SPREAD: f32 = 0.015;

/// One direction of the glow's blur: a Gaussian of `HALATION_SPREAD`, across (`dir` 1, 0) or down
/// (0, 1) the glow picture.
fn glow_blur(coord: Vec4, frame: &Frame, picture: &Picture, dir: IVec2) -> Vec4 {
    let sigma = (HALATION_SPREAD * frame.screen.y / GLOW_STEP as f32).max(0.5);
    let reach = (3.0 * sigma).ceil() as i32;
    let size = IVec2::new(frame.screen.z as i32, frame.screen.w as i32);
    let at = IVec2::new(coord.x as i32, coord.y as i32);
    let mut sum = Vec3::ZERO;
    let mut weight = 0.0;
    let mut i = -reach;
    while i <= reach {
        let p = (at + dir * i).clamp(IVec2::ZERO, size - IVec2::ONE);
        let w = (-((i * i) as f32) / (2.0 * sigma * sigma)).exp();
        let c: Vec4 = picture.fetch(p);
        sum += c.xyz() * w;
        weight += w;
        i += 1;
    }
    (sum / weight).extend(1.0)
}

/// The glow's blur across.
#[spirv(fragment)]
pub fn glow_across_fs(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] frame: &Frame,
    #[spirv(descriptor_set = 0, binding = 1)] picture: &Picture,
    out: &mut Vec4,
) {
    *out = glow_blur(coord, frame, picture, IVec2::new(1, 0));
}

/// The glow's blur down.
#[spirv(fragment)]
pub fn glow_downward_fs(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] frame: &Frame,
    #[spirv(descriptor_set = 0, binding = 1)] picture: &Picture,
    out: &mut Vec4,
) {
    *out = glow_blur(coord, frame, picture, IVec2::new(0, 1));
}

/// The error function (Abramowitz and Stegun 7.1.26, within 1.5e-7).
fn erf(x: f32) -> f32 {
    let s = if x < 0.0 { -1.0 } else { 1.0 };
    let x = x.abs();
    let t = 1.0 / (1.0 + 0.327_591_1 * x);
    let poly = t * (0.254_829_6 + t * (-0.284_496_74 + t * (1.421_413_7 + t * (-1.453_152 + t * 1.061_405_4))));
    s * (1.0 - poly * (-x * x).exp())
}

/// How many of the screen's rows one line of the monitor's scan takes: two, the fewest in which
/// a line and the gap beside it can both be shown, so the scan is the finest the screen can hold
/// whatever its size (row 24).
const ROWS_PER_LINE: i32 = 2;

/// The beam's spot, its standard deviation in screen rows, dark and at full brightness: a beam
/// carrying more current spreads wider, so dark lines stand apart and bright ones close the gap
/// between them. The model's own pair, set so the sharpest tube shows its lines in the shadows
/// and none in full light (at 1.0 a line's light reaches the rows between evenly), not measured
/// from a tube.
const SPOT_DARK: f32 = 0.6;
const SPOT_BRIGHT: f32 = 1.0;

/// The aperture grille: each of the screen's columns in turn carries the red, green or blue
/// phosphor stripe, three columns a triad, the finest the screen can show. A stripe passes its
/// own colour whole and `GRILLE_OTHER` of the other two (the model's own share: a sharp tube's
/// light spreading a little past its stripe), the whole brought back to the same mean.
const GRILLE_OTHER: f32 = 0.7;

/// The share of the light that reaches the eye by way of the glass's halo, the rest passing
/// straight out. The sharpest tubes had dark-tinted faceplates, about half the light lost on each
/// pass through the glass, so the scattered light, which crosses the glass at least twice more,
/// comes out several times weaker than through clear glass: about 2%. The model's own estimate,
/// not measured. (Six per cent, a clear faceplate's, lifted the darkest of a dark corridor almost
/// twice and hazed the picture.)
const HALATION_SHARE: f32 = 0.02;

/// Light a stripe cannot show past the screen's full brightness spills to the pixel's other
/// colours, as a saturated phosphor stripe blooms onto its neighbours: white stays white and the
/// mean is kept wherever the screen can hold it.
fn spill(v: Vec3) -> Vec3 {
    let over = (v - Vec3::ONE).max(Vec3::ZERO);
    let held = v.min(Vec3::ONE);
    let room = Vec3::ONE - held;
    let share = ((over.x + over.y + over.z) / (room.x + room.y + room.z).max(1.0e-6)).min(1.0);
    held + room * share
}

/// One line's light along a column: the picture's rows its band covers, a box two rows high
/// centred on the line's row (`k` the line's number), the rows at its edges given half.
fn line_signal(picture: &Picture, x: i32, k: i32, rows: i32) -> Vec3 {
    let row = |r: i32| -> Vec3 {
        let c: Vec4 = picture.fetch(IVec2::new(x, r.clamp(0, rows - 1)));
        c.xyz()
    };
    let centre = k * ROWS_PER_LINE;
    (row(centre) + (row(centre - 1) + row(centre + 1)) * 0.5) / ROWS_PER_LINE as f32
}

/// The light one line's beam leaves on a row of the screen `distance` rows from the line: the
/// beam's Gaussian spot, its width by the brightness it carries (each colour's gun its own),
/// integrated over the row's height, so a line's light over all the rows it reaches is its
/// signal times the rows it takes.
fn beam_on_row(signal: Vec3, distance: f32) -> Vec3 {
    let spread = |s: f32| -> f32 {
        let sigma = SPOT_DARK + (SPOT_BRIGHT - SPOT_DARK) * s.clamp(0.0, 1.0).sqrt();
        let k = 1.0 / (sigma * core::f32::consts::SQRT_2);
        0.5 * (erf((distance + 0.5) * k) - erf((distance - 0.5) * k)) * ROWS_PER_LINE as f32
    };
    vec3(signal.x * spread(signal.x), signal.y * spread(signal.y), signal.z * spread(signal.z))
}

/// The finished picture seen on the sharpest monitor a screen can show (row 24): scanned in lines
/// two rows apart, each colour's beam a spot that widens as it brightens, through an aperture
/// grille of one column a stripe, out through glass that scatters a little of its light into a
/// halo. Flat, as the sharpest tubes were.
#[spirv(fragment)]
pub fn crt_fs(
    #[spirv(frag_coord)] coord: Vec4,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] frame: &Frame,
    #[spirv(descriptor_set = 0, binding = 1)] picture: &Picture,
    #[spirv(descriptor_set = 0, binding = 2)] sampler: &Sampler,
    #[spirv(descriptor_set = 0, binding = 3)] glow: &Picture,
    out: &mut Vec4,
) {
    let x = coord.x as i32;
    let y = coord.y as i32;
    let rows = frame.screen.y as i32;
    // the lines whose spots reach this row: the nearest at or above, one above it, two below
    let nearest = y / ROWS_PER_LINE;
    let mut light = Vec3::ZERO;
    let mut k = nearest - 1;
    while k <= nearest + 2 {
        let distance = (y - k * ROWS_PER_LINE) as f32;
        light += beam_on_row(line_signal(picture, x, k, rows), distance);
        k += 1;
    }
    // the grille's stripe under this column, the same mean light as no grille
    let own = 3.0 / (1.0 + 2.0 * GRILLE_OTHER);
    let other = GRILLE_OTHER * own;
    let stripe = x.rem_euclid(3);
    let grille = vec3(if stripe == 0 { own } else { other }, if stripe == 1 { own } else { other }, if stripe == 2 { own } else { other });
    let halo: Vec4 = glow.sample_by_lod(*sampler, vec2(coord.x, coord.y) / vec2(frame.screen.x, frame.screen.y), 0.0);
    let shown = spill(light * grille) * (1.0 - HALATION_SHARE) + halo.xyz() * HALATION_SHARE;
    *out = shown.extend(1.0);
}

/// A letter on the screen: where its grid's corner on the base line is, in pixels from the top
/// left, pixels a grid unit, and the stroke's width in grid units; its colour; which letter.
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Letter {
    pub place: Vec4,
    pub colour: Vec4,
    /// `x`: the letter's first stroke in the stroke list; `y`: how many strokes.
    pub strokes: UVec4,
}

/// The screen's size in pixels, for the letters.
#[derive(Clone, Copy)]
#[cfg_attr(not(target_arch = "spirv"), derive(bytemuck::Pod, bytemuck::Zeroable))]
#[repr(C)]
pub struct Screen {
    pub size: Vec4,
}

/// A letter's square: its grid from −1 to 5 across and −2 to 7 up, room for the strokes' width.
#[spirv(vertex)]
pub fn text_vs(
    #[spirv(vertex_index)] vertex: i32,
    #[spirv(instance_index)] instance: u32,
    #[spirv(uniform, descriptor_set = 0, binding = 0)] screen: &Screen,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] letters: &[Letter],
    #[spirv(position)] out_position: &mut Vec4,
    out_grid: &mut Vec2,
    #[spirv(flat)] out_index: &mut u32,
) {
    let l = letters[instance as usize];
    let corners = [vec2(-1.0, -2.0), vec2(5.0, -2.0), vec2(5.0, 7.0), vec2(-1.0, -2.0), vec2(5.0, 7.0), vec2(-1.0, 7.0)];
    let g = corners[vertex as usize % 6];
    let px = vec2(l.place.x + g.x * l.place.z, l.place.y - g.y * l.place.z);
    *out_position = vec4(px.x / screen.size.x * 2.0 - 1.0, 1.0 - px.y / screen.size.y * 2.0, 0.0, 1.0);
    *out_grid = g;
    *out_index = instance;
}

/// A letter drawn by the distance to its nearest stroke, its edge a pixel soft.
#[spirv(fragment)]
pub fn text_fs(
    grid: Vec2,
    #[spirv(flat)] index: u32,
    #[spirv(storage_buffer, descriptor_set = 0, binding = 1)] letters: &[Letter],
    #[spirv(storage_buffer, descriptor_set = 0, binding = 2)] strokes: &[Vec4],
    out: &mut Vec4,
) {
    let l = letters[index as usize];
    let mut d = 1.0e9f32;
    let mut i = 0;
    while i < l.strokes.y {
        let s = strokes[(l.strokes.x + i) as usize];
        let a = s.xy();
        let b = s.zw();
        let ab = b - a;
        let t = ((grid - a).dot(ab) / ab.dot(ab).max(1.0e-6)).clamp(0.0, 1.0);
        d = d.min((grid - (a + ab * t)).length());
        i += 1;
    }
    let edge = (d - l.place.w * 0.5) * l.place.z;
    let alpha = (0.5 - edge).clamp(0.0, 1.0) * l.colour.w;
    *out = (l.colour.xyz() * alpha).extend(alpha);
}
