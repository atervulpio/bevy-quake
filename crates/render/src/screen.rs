//! After Bevy's pass, before its picture goes to the window: the game's tone curve and the colour
//! laid over the screen, the screen's letters over the picture (`overlay_fs`, `text_vs`,
//! `text_fs`), then, when it is on, the old monitor (`glow_down_fs`, `glow_across_fs`,
//! `glow_downward_fs`, `crt_fs`). Each a pass of our programs over the whole picture, in Bevy's own
//! place for such passes (`Core3dSystems::PostProcess`, after `tonemapping`, which lays nothing: the
//! camera asks for no curve of Bevy's).

use crate::SHADERS;
use bevy::core_pipeline::schedule::Core3d;
use bevy::core_pipeline::tonemapping::tonemapping;
use bevy::core_pipeline::Core3dSystems;
use bevy::prelude::*;
use bevy::render::extract_resource::{ExtractResource, ExtractResourcePlugin};
use bevy::render::render_resource::binding_types::{sampler, storage_buffer_read_only_sized, texture_2d, uniform_buffer_sized};
use bevy::render::render_resource::*;
use bevy::render::renderer::{RenderContext, RenderDevice, ViewQuery};
use bevy::render::texture::{CachedTexture, TextureCache};
use bevy::render::view::ViewTarget;
use bevy::render::{Render, RenderApp, RenderStartup, RenderSystems};
use shaders::{Frame, GLOW_STEP, Letter};

/// What the passes after Bevy's are given each frame by the game's side: the colour over
/// the screen, whether the monitor is on, and the letters.
#[derive(Resource, Clone, Default, ExtractResource)]
#[extract_app(RenderApp)]
pub(crate) struct Overlay {
    pub blend: Vec4,
    pub crt: bool,
    pub letters: Vec<Letter>,
}

/// The pipelines, made once.
#[derive(Resource)]
struct Passes {
    picture_layout: BindGroupLayoutDescriptor,
    text_layout: BindGroupLayoutDescriptor,
    sampler: Sampler,
    strokes: Buffer,
    overlay: CachedRenderPipelineId,
    text: CachedRenderPipelineId,
    glow: [CachedRenderPipelineId; 3],
    crt: CachedRenderPipelineId,
}

/// The format the passes draw into: the camera draws in light's own range (`Hdr`), in Bevy's
/// format for it (`Rgba16Float`).
const PICTURE: TextureFormat = TextureFormat::Rgba16Float;

fn make_passes(mut commands: Commands, device: Res<RenderDevice>, cache: Res<PipelineCache>) {
    let picture_layout = BindGroupLayoutDescriptor::new(
        "a picture read",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::FRAGMENT,
            (
                uniform_buffer_sized(false, Some(std::num::NonZero::new(size_of::<Frame>() as u64).unwrap())),
                texture_2d(TextureSampleType::Float { filterable: true }),
                sampler(SamplerBindingType::Filtering),
                texture_2d(TextureSampleType::Float { filterable: true }),
            ),
        ),
    );
    let text_layout = BindGroupLayoutDescriptor::new(
        "letters",
        &BindGroupLayoutEntries::sequential(
            ShaderStages::VERTEX_FRAGMENT,
            (uniform_buffer_sized(false, None), storage_buffer_read_only_sized(false, None), storage_buffer_read_only_sized(false, None)),
        ),
    );
    let (segments, _) = letters::table();
    let strokes = device.create_buffer_with_data(&BufferInitDescriptor { label: Some("strokes"), contents: bytemuck::cast_slice(&segments), usage: BufferUsages::STORAGE });
    let sampler = device.create_sampler(&SamplerDescriptor { mag_filter: FilterMode::Linear, min_filter: FilterMode::Linear, ..default() });
    let pass = |label: &'static str, layout: &BindGroupLayoutDescriptor, vs: &'static str, fs: &'static str, format: TextureFormat, blend: Option<BlendState>| {
        cache.queue_render_pipeline(RenderPipelineDescriptor {
            label: Some(label.into()),
            layout: vec![layout.clone()],
            vertex: VertexState { shader: SHADERS, entry_point: Some(vs.into()), ..default() },
            fragment: Some(FragmentState { shader: SHADERS, entry_point: Some(fs.into()), targets: vec![Some(ColorTargetState { format, blend, write_mask: ColorWrites::ALL })], ..default() }),
            ..default()
        })
    };
    commands.insert_resource(Passes {
        overlay: pass("the colour over the screen", &picture_layout, "screen_vs", "overlay_fs", PICTURE, None),
        text: pass("letters", &text_layout, "text_vs", "text_fs", PICTURE, Some(BlendState::PREMULTIPLIED_ALPHA_BLENDING)),
        glow: [
            pass("the glass's glow brought down", &picture_layout, "screen_vs", "glow_down_fs", GLOW, None),
            pass("the glow blurred across", &picture_layout, "screen_vs", "glow_across_fs", GLOW, None),
            pass("the glow blurred down", &picture_layout, "screen_vs", "glow_downward_fs", GLOW, None),
        ],
        crt: pass("the monitor", &picture_layout, "screen_vs", "crt_fs", PICTURE, None),
        picture_layout,
        text_layout,
        sampler,
        strokes,
    });
}

/// The glass's glow pictures' format.
const GLOW: TextureFormat = TextureFormat::Rgba16Float;

/// A view's buffers and glow pictures for this frame.
#[derive(Component)]
struct ViewOverlay {
    frame: Buffer,
    screen: Buffer,
    letters: Buffer,
    count: u32,
    glow: [CachedTexture; 2],
}

fn prepare(mut commands: Commands, views: Query<(Entity, &ViewTarget)>, overlay: Res<Overlay>, device: Res<RenderDevice>, mut textures: ResMut<TextureCache>) {
    for (entity, target) in &views {
        let size = target.main_texture().size();
        let glow_size = (size.width.div_ceil(GLOW_STEP), size.height.div_ceil(GLOW_STEP));
        let frame = Frame { screen: Vec4::new(size.width as f32, size.height as f32, glow_size.0 as f32, glow_size.1 as f32), blend: overlay.blend };
        let buffer = |label: &'static str, bytes: &[u8], usage: BufferUsages| device.create_buffer_with_data(&BufferInitDescriptor { label: Some(label), contents: bytes, usage });
        // a buffer may not be empty: no letters is one letter of nought, drawn by no one
        let letters: &[Letter] = if overlay.letters.is_empty() { &[Letter { place: Vec4::ZERO, colour: Vec4::ZERO, strokes: UVec4::ZERO }] } else { &overlay.letters };
        let mut glow = || {
            textures.get(
                &device,
                TextureDescriptor {
                    label: Some("the glass's glow"),
                    size: Extent3d { width: glow_size.0, height: glow_size.1, depth_or_array_layers: 1 },
                    mip_level_count: 1,
                    sample_count: 1,
                    dimension: TextureDimension::D2,
                    format: GLOW,
                    usage: TextureUsages::RENDER_ATTACHMENT | TextureUsages::TEXTURE_BINDING,
                    view_formats: &[],
                },
            )
        };
        let glow = [glow(), glow()];
        commands.entity(entity).insert(ViewOverlay {
            frame: buffer("frame", bytemuck::bytes_of(&frame), BufferUsages::UNIFORM),
            screen: buffer("screen", bytemuck::bytes_of(&shaders::Screen { size: frame.screen }), BufferUsages::UNIFORM),
            letters: buffer("letters", bytemuck::cast_slice(letters), BufferUsages::STORAGE),
            count: overlay.letters.len() as u32,
            glow,
        });
    }
}

/// A pass over every pixel of `target`, reading what `group` holds.
fn whole(ctx: &mut RenderContext, label: &str, target: &TextureView, pipeline: &RenderPipeline, group: &BindGroup) {
    let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
        label: Some(label),
        color_attachments: &[Some(RenderPassColorAttachment { view: target, depth_slice: None, resolve_target: None, ops: Operations::default() })],
        depth_stencil_attachment: None,
        timestamp_writes: None,
        occlusion_query_set: None,
        multiview_mask: None,
    });
    pass.set_pipeline(pipeline);
    pass.set_bind_group(0, group, &[]);
    pass.draw(0..3, 0..1);
}

fn after_bevys_pass(view: ViewQuery<(&ViewTarget, &ViewOverlay)>, passes: Option<Res<Passes>>, cache: Res<PipelineCache>, device: Res<RenderDevice>, overlay: Res<Overlay>, mut ctx: RenderContext) {
    let Some(passes) = passes else { return };
    let (target, mine) = view.into_inner();
    assert_eq!(target.main_texture_format(), PICTURE, "the passes after Bevy's are made for its picture in light's own range");
    let get = |id| cache.get_render_pipeline(id);
    let (Some(overlay_pipeline), Some(text), Some(down), Some(across), Some(downward), Some(crt)) =
        (get(passes.overlay), get(passes.text), get(passes.glow[0]), get(passes.glow[1]), get(passes.glow[2]), get(passes.crt))
    else {
        return;
    };
    let picture_layout = cache.get_bind_group_layout(&passes.picture_layout);
    let reading = |picture: &TextureView, other: &TextureView| {
        device.create_bind_group("a picture read", &picture_layout, &BindGroupEntries::sequential((mine.frame.as_entire_binding(), picture, &passes.sampler, other)))
    };
    // the tone curve and the colour over the screen, then the letters over the picture
    let write = target.post_process_write();
    {
        let group = reading(write.source, write.source);
        let text_group = device.create_bind_group(
            "letters",
            &cache.get_bind_group_layout(&passes.text_layout),
            &BindGroupEntries::sequential((mine.screen.as_entire_binding(), mine.letters.as_entire_binding(), passes.strokes.as_entire_binding())),
        );
        let mut pass = ctx.command_encoder().begin_render_pass(&RenderPassDescriptor {
            label: Some("the colour over the screen, and the letters"),
            color_attachments: &[Some(RenderPassColorAttachment { view: write.destination, depth_slice: None, resolve_target: None, ops: Operations::default() })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
            multiview_mask: None,
        });
        pass.set_pipeline(overlay_pipeline);
        pass.set_bind_group(0, &group, &[]);
        pass.draw(0..3, 0..1);
        if mine.count > 0 {
            pass.set_pipeline(text);
            pass.set_bind_group(0, &text_group, &[]);
            pass.draw(0..6, 0..mine.count);
        }
    }
    if !overlay.crt {
        return;
    }
    // the monitor: the glass's glow brought down and blurred both ways, then the scan, the grille
    // and the glow together
    let picture = target.main_texture_view();
    let [a, b] = &mine.glow;
    whole(&mut ctx, "the glass's glow brought down", &a.default_view, down, &reading(picture, picture));
    whole(&mut ctx, "the glow blurred across", &b.default_view, across, &reading(&a.default_view, &a.default_view));
    whole(&mut ctx, "the glow blurred down", &a.default_view, downward, &reading(&b.default_view, &b.default_view));
    let write = target.post_process_write();
    whole(&mut ctx, "the monitor", write.destination, crt, &reading(write.source, &a.default_view));
}

pub(crate) struct Screen;

impl Plugin for Screen {
    fn build(&self, app: &mut App) {
        app.init_resource::<Overlay>().add_plugins(ExtractResourcePlugin::<Overlay>::default());
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else { return };
        render_app
            .add_systems(RenderStartup, make_passes)
            .add_systems(Render, prepare.in_set(RenderSystems::PrepareResources))
            .add_systems(Core3d, after_bevys_pass.in_set(Core3dSystems::PostProcess).after(tonemapping));
    }
}
