//! The picture, drawn by Bevy (row 41): Bevy draws, the programs for the card stay Rust.
//! Bevy takes the window and draws the world with its own renderer, its own lights and shadows; the
//! light our physics computed is handed to it as it is, the surfaces' light as Bevy's lightmaps
//! and the light in the open as Bevy's irradiance volume; what Bevy does not draw itself, the sky,
//! the liquids, the colour over the screen, the letters and the old monitor, are programs of ours in
//! Rust (`crates/shaders`). The view and its text (`view.rs`), what a level is made into
//! (`level.rs`), one frame shown (`frame.rs`), the sky and the liquids (`surfaces.rs`), the passes
//! after Bevy's tone curve (`screen.rs`); here what the game gives and the plugin that takes it.
//!
//! **Bevy's world is the game's turned.** The game's axes are Quake's, z up; Bevy's are y up. Every
//! thing of the level stands under one entity, `Quake`, turned a quarter about x, so the meshes,
//! the lamps and the probes keep the game's own numbers.

mod frame;
mod level;
mod screen;
mod surfaces;
mod view;

use bevy::prelude::*;
pub use view::{Text, View, angle_vectors, text_width};

/// The programs of `crates/shaders`, compiled to SPIR-V by rust-gpu when this crate is built.
pub(crate) const SHADERS_SPV: &[u8] = include_bytes!(env!("SHADERS_SPV"));
/// Their module among Bevy's shaders, under a name of its own.
pub(crate) const SHADERS: Handle<Shader> = bevy::asset::uuid_handle!("6a1f3c52-8a0e-4b8e-9f7d-2c1b0e5a7d31");

/// A surface's reflectance against its palette colour. id's colours are what a surface looks like
/// under the original's standard light, which its renderer brightened up to twice ("overbright"):
/// taken as reflectances they are coal-black (the episode's textures 2% at the median, 16% at the
/// brightest, the marble floor), and a surface that dark shows little but the 4% gloss every
/// non-metal has, grey. So a palette colour is a quarter of the surface's reflectance: the marble
/// 65%, as pale marble is, the median 8.5%, dark stone and painted iron; and the frame's exposure
/// is a quarter, so what direct light shows is unchanged and only the light thrown back from
/// surface to surface is a true reflectance's. Every colour of the game's data meets the light
/// through this.
pub const REFLECTANCE: f32 = 4.0;

/// A reflectance at most: nothing real throws back all the light.
pub(crate) const MOST: f32 = 0.95;

/// How bright a thing's own light is against the light computed for a surface: the palette's
/// bright colours, drawn by the original at full brightness whatever the light, given off here at
/// one and a half times a painted surface's brightness.
pub(crate) const GLOW: f32 = 1.5 * REFLECTANCE;

/// The most moving lights at once (`MAX_DLIGHTS`).
pub const MAX_LIGHTS: usize = 32;

/// What a texture is drawn as: a surface lit by the light, the sky, or a liquid of a kind.
pub const SURFACE: u32 = 0;
pub const SKY: u32 = 1;
pub const WATER: u32 = 2;
pub const SLIME: u32 = 3;
pub const LAVA: u32 = 4;
pub const TELEPORT: u32 = 5;

/// A corner of a brush model's faces: its place and normal, where on its texture (in the texture's
/// own repeats), where its light is in the level's light atlas (in samples) and which face's light
/// it is (`u32::MAX` for a face with none: the sky, a liquid, a pick-up's box of its own file), and
/// which of the level's texture slots it wears.
#[derive(Clone, Copy)]
pub struct BrushVertex {
    pub position: Vec3,
    pub normal: Vec3,
    pub uv: Vec2,
    pub light: Vec2,
    pub face: u32,
    pub slot: u32,
}

/// A model made of the map's brushes: the world, a door, a lift, a box of shells.
pub struct BrushMesh {
    pub vertices: Vec<BrushVertex>,
    pub indices: Vec<u32>,
}

/// A model's vertex in one pose: its place and normal in the model's own space.
#[derive(Clone, Copy)]
pub struct PoseVertex {
    pub position: Vec3,
    pub normal: Vec3,
}

/// A model of the game's own (`.mdl`): each corner's place on the skin (0–1), its skins' places
/// in the level's pictures, its triangles, and every pose it can take.
pub struct AliasMesh {
    pub uvs: Vec<Vec2>,
    pub skins: Vec<u32>,
    pub indices: Vec<u32>,
    pub poses: Vec<Vec<PoseVertex>>,
}

/// One face's light in the atlas: where its samples' block stands (inside a border of one), its
/// size, and the light styles of its four slots (255 for none).
#[derive(Clone, Copy)]
pub struct FaceLight {
    pub place: (u32, u32),
    pub width: u32,
    pub height: u32,
    pub styles: [u8; 4],
}

/// The level's computed light: the atlas, four layers of `Rgb9e5` samples, one for each of a
/// face's style slots; each face's place in it; and the probe grid (its first point, spacing,
/// size, and for each of the six axes of the game, +x, −x, +y, −y, +z, −z, its `Rgb9e5`
/// irradiance at every point, x fastest), with each other style's lamps' own light apart.
pub struct LightData {
    pub width: u32,
    pub height: u32,
    pub layers: [Vec<u32>; 4],
    pub faces: Vec<Option<FaceLight>>,
    pub probe_origin: Vec3,
    pub probe_spacing: f32,
    pub probe_dims: [u32; 3],
    pub probes: [Vec<u32>; 6],
    pub styled_probes: Vec<light::StyledProbes>,
}

/// One of id's pictures (row 39): its colours, sRGB, with `a` marking what the picture's kind
/// asks (what glows by itself on a texture or a skin; the sky's front layer; where a sprite
/// shows), every level of detail, the first `width × height`.
pub struct PictureData {
    pub width: u32,
    pub height: u32,
    pub levels: Vec<Vec<[u8; 4]>>,
}

/// What a texture is drawn as now: which picture, and what kind (`SURFACE`, `SKY`…).
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Skin {
    pub picture: u32,
    pub kind: u32,
}

/// What one of the level's textures shows: its animation's frames in turn, five a second
/// (`R_TextureAnimation`), and the other animation a brush model shows at a frame other than its
/// first (a pressed button). With no other animation, the first serves both.
pub struct Slot {
    pub frames: Vec<Skin>,
    pub alternate: Vec<Skin>,
}

impl Slot {
    /// The frame shown at `time`: each frame two tenths of a second (`ANIM_CYCLE`).
    pub(crate) fn at(&self, alternate: bool, time: f32) -> Skin {
        let frames = if alternate && !self.alternate.is_empty() { &self.alternate } else { &self.frames };
        frames[((time * 5.0).max(0.0) as usize) % frames.len()]
    }
}

/// Everything a level draws, made once when it begins.
#[derive(Default)]
pub struct LevelMeshes {
    pub brush: Vec<BrushMesh>,
    pub alias: Vec<AliasMesh>,
    pub light: Option<LightData>,
    pub pictures: Vec<PictureData>,
    pub slots: Vec<Slot>,
}

/// One thing to draw this frame, its transform in the game's own terms.
#[derive(Clone, Copy)]
pub enum Draw {
    /// A brush model; `alternate` when its frame is not its first (`currententity->frame`).
    Brush { mesh: usize, transform: Mat4, alternate: bool },
    /// A model between two poses: `t` of the way from `from` to `to`, wearing its skin `skin`.
    Alias { mesh: usize, transform: Mat4, from: usize, to: usize, t: f32, skin: usize },
}

/// A sprite (row 39), id's: where it stands, which picture of the level's, and where its picture's
/// edges stand from there, left, right, down and up (`R_DrawSprite`).
#[derive(Clone, Copy)]
pub struct Billboard {
    pub origin: Vec3,
    pub picture: u32,
    pub edges: [f32; 4],
}

/// A particle: its place and size, its colour (linear), and whether it gives off light of its own.
#[derive(Clone, Copy)]
pub struct Dot {
    pub place: Vec3,
    pub size: f32,
    pub colour: Vec3,
    pub glows: bool,
}

/// What the game gives Bevy each frame: the view and what to draw.
#[derive(Resource, Default)]
pub struct Shown {
    pub view: View,
    pub draws: Vec<Draw>,
}

/// A picture drawn into a texture of this size rather than into the window, as a picture kept or
/// timed is (`--shot`): what the frame costs is then the drawing's alone, not the window's showing
/// it, whose pace the desktop sets. Its image, once made.
#[derive(Resource, Default)]
pub struct Offscreen {
    pub size: Option<(u32, u32)>,
    pub image: Option<Handle<Image>>,
}

/// A level the game has made, for Bevy to take in place of the one shown.
#[derive(Resource, Default)]
pub struct NewLevel(pub Option<LevelMeshes>);

/// Whether the card has every program the frame asks for: Bevy makes its pipelines over the frames
/// after they are first asked for, and a picture taken before they are made is missing what they
/// draw. Set by the drawing's side each frame; read by whoever waits for a whole picture (`--shot`).
#[derive(Resource, Clone, Default)]
pub struct Ready(pub std::sync::Arc<std::sync::atomic::AtomicBool>);

fn tell_ready(cache: Res<bevy::render::render_resource::PipelineCache>, ready: Res<Ready>) {
    ready.0.store(cache.waiting_pipelines().next().is_none(), std::sync::atomic::Ordering::Relaxed);
}

/// The picture: everything above, given to a Bevy app.
pub struct Picture;

impl Plugin for Picture {
    fn build(&self, app: &mut App) {
        app.world_mut().resource_mut::<Assets<Shader>>().insert(SHADERS.id(), Shader::from_spirv(SHADERS_SPV, "shaders.spv")).expect("the shaders' place among Bevy's");
        app.init_resource::<Shown>()
            .init_resource::<NewLevel>()
            .init_resource::<Offscreen>()
            // the light is ours, all of it: no light from nowhere (Bevy's default is 80 cd/m²)
            .insert_resource(GlobalAmbientLight { brightness: 0.0, ..default() })
            .add_plugins((surfaces::Surfaces, screen::Screen))
            .add_systems(Startup, frame::begin)
            .add_systems(PostUpdate, (level::take, frame::show).chain().before(bevy::transform::TransformSystems::Propagate));
        let ready = Ready::default();
        app.insert_resource(ready.clone());
        if let Some(render_app) = app.get_sub_app_mut(bevy::render::RenderApp) {
            render_app.insert_resource(ready).add_systems(bevy::render::Render, tell_ready.in_set(bevy::render::RenderSystems::Cleanup));
        }
    }
}
