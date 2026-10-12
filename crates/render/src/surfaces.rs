//! The sky and the liquids as Bevy's materials: Bevy's own vertex program and pass, and our
//! programs at every pixel (`sky_fs`, `liquid_fs`), each reading where the eye is and the time,
//! how bright it is drawn, and its picture.

use crate::SHADERS;
use bevy::mesh::MeshVertexBufferLayoutRef;
use bevy::pbr::{Material, MaterialPipeline, MaterialPipelineKey, MaterialPlugin};
use bevy::prelude::*;
use bevy::render::render_resource::{AsBindGroup, RenderPipelineDescriptor, SpecializedMeshPipelineError};
use bevy::shader::ShaderRef;

/// The program at every pixel of a material, by its name in our module.
fn our_fragment(descriptor: &mut RenderPipelineDescriptor, name: &'static str) -> Result<(), SpecializedMeshPipelineError> {
    if let Some(f) = descriptor.fragment.as_mut() {
        f.entry_point = Some(name.into());
    }
    Ok(())
}

/// The sky (`R_DrawSkyChain`, row 39): its two layers drifting over a low dome.
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct SkyMaterial {
    /// Where the eye stands in the game's own terms, and the time (`w`).
    #[uniform(0)]
    pub eye: Vec4,
    /// `x`: the sky's own glow times the camera's exposure.
    #[uniform(1)]
    pub light: Vec4,
    #[texture(2)]
    #[sampler(3)]
    pub picture: Handle<Image>,
}

impl Material for SkyMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(SHADERS)
    }

    fn specialize(_: &MaterialPipeline, descriptor: &mut RenderPipelineDescriptor, _: &MeshVertexBufferLayoutRef, _: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        our_fragment(descriptor, "sky_fs")
    }
}

/// A liquid (`EmitWaterPolys`, row 39): its picture warped, giving off its own light.
#[derive(Asset, TypePath, AsBindGroup, Clone)]
pub struct LiquidMaterial {
    /// The time is `w`; the rest is the eye's, read by the sky alone.
    #[uniform(0)]
    pub eye: Vec4,
    #[uniform(1)]
    pub light: Vec4,
    #[texture(2)]
    #[sampler(3)]
    pub picture: Handle<Image>,
}

impl Material for LiquidMaterial {
    fn fragment_shader() -> ShaderRef {
        ShaderRef::Handle(SHADERS)
    }

    fn specialize(_: &MaterialPipeline, descriptor: &mut RenderPipelineDescriptor, _: &MeshVertexBufferLayoutRef, _: MaterialPipelineKey<Self>) -> Result<(), SpecializedMeshPipelineError> {
        our_fragment(descriptor, "liquid_fs")
    }
}

pub(crate) struct Surfaces;

impl Plugin for Surfaces {
    fn build(&self, app: &mut App) {
        app.add_plugins((MaterialPlugin::<SkyMaterial>::default(), MaterialPlugin::<LiquidMaterial>::default()));
    }
}
