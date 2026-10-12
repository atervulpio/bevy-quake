//! Where the eye is and what is laid over the picture: the view in the game's own terms, and lines
//! of text.

use crate::{Billboard, Dot};
use glam::{Mat3, Vec3};

/// A line of text on the screen: its left end on the base line, in pixels from the top left; its
/// capitals' height in pixels; its colour and how opaque.
#[derive(Clone)]
pub struct Text {
    pub x: f32,
    pub y: f32,
    pub size: f32,
    pub colour: glam::Vec4,
    pub text: String,
}

/// A letter's place to the next, in grid units: four wide, two between.
pub(crate) const ADVANCE: f32 = 6.0;

/// How wide a line is drawn at a size, for centring it.
pub fn text_width(text: &str, size: f32) -> f32 {
    let n = text.chars().count() as f32;
    if n == 0.0 { 0.0 } else { (n * ADVANCE - 2.0) * size / 6.0 }
}

/// Where the eye stands and looks, in the game's own terms: Quake's axes (x forward at yaw 0,
/// y left, z up) and its angles in degrees (pitch down positive, yaw, roll).
#[derive(Clone, Default)]
pub struct View {
    pub origin: Vec3,
    pub angles: Vec3,
    /// The horizontal field of view at a 4:3 screen, the game's `fov` (90 by default); a wider
    /// screen sees more at the sides, never less above and below.
    pub fov: f32,
    pub time: f32,
    /// The 64 light styles' brightness now (`d_lightstylevalue / 256`).
    pub styles: Vec<f32>,
    /// The colour laid over the screen and how much of it (`v_blend`).
    pub blend: glam::Vec4,
    /// A multiplier on all light before the tone curve.
    pub exposure: f32,
    /// Moving lights: place, intensity (`r³ / 2048` from the original's radius) and colour.
    pub lights: Vec<(Vec3, f32, Vec3)>,
    pub dots: Vec<Dot>,
    /// id's sprites (row 39).
    pub sprites: Vec<Billboard>,
    /// Text over the picture: the status, messages, the menu.
    pub texts: Vec<Text>,
    /// Shown on the sharpest monitor the screen can show (`crt_fs`), or as it is.
    pub crt: bool,
    /// The draws of the weapon in hand, which stands in no lamp's way (it is not in the world).
    pub in_hand: std::ops::Range<usize>,
}

/// `AngleVectors` (`mathlib.c`): forward, right and up from pitch, yaw and roll.
pub fn angle_vectors(a: Vec3) -> (Vec3, Vec3, Vec3) {
    let (sp, cp) = a.x.to_radians().sin_cos();
    let (sy, cy) = a.y.to_radians().sin_cos();
    let (sr, cr) = a.z.to_radians().sin_cos();
    let forward = Vec3::new(cp * cy, cp * sy, -sp);
    let right = Vec3::new(-sr * sp * cy + cr * sy, -sr * sp * sy - cr * cy, -sr * cp);
    let up = Vec3::new(cr * sp * cy + sr * sy, cr * sp * sy - sr * cy, cr * cp);
    (forward, right, up)
}

impl View {
    /// The eye's turn as Bevy holds a camera's, in the game's space: its right along x, its up
    /// along y, and looking down its own −z.
    pub(crate) fn turn(&self) -> Mat3 {
        let (f, r, u) = angle_vectors(self.angles);
        Mat3::from_cols(r, u, -f)
    }

    /// The vertical field of view, from the horizontal one at a 4:3 screen.
    pub(crate) fn fov_y(&self) -> f32 {
        2.0 * ((self.fov.to_radians() * 0.5).tan() * 3.0 / 4.0).atan()
    }
}
