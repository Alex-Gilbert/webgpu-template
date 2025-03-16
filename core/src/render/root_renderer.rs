use bevy_ecs::{
    system::{Query, Res, SystemState},
    world::World,
};

use wgpu::{CommandBuffer, TextureView};

use crate::{
    ecs::components::gpu_bindings::camera_bindings::CameraBindings,
    gpu_resources::render_resources::RenderResources,
    utils::texture::{Texture, TextureBuilder},
};

use super::unlit_diffuse_sub_renderer::UnlitDiffuseSubRenderer;

type RootRendererSystemState = SystemState<(
    Res<'static, RenderResources>,
    Query<'static, 'static, (&'static CameraBindings,)>,
)>;

pub struct RootRenderer {
    system_state: RootRendererSystemState,

    unlit_diffuse_sub_renderer: UnlitDiffuseSubRenderer,

    depth_texture: Texture,
}

impl std::fmt::Debug for RootRenderer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RootRenderer").finish()
    }
}

impl RootRenderer {
    pub fn new(world: &mut World, width: u32, height: u32) -> Self {
        let unlit_diffuse_sub_renderer = UnlitDiffuseSubRenderer::new(world);
        let system_state: RootRendererSystemState = SystemState::new(world);

        let render_resources = world.get_resource::<RenderResources>().unwrap();
        let device = &render_resources.device;

        let mut renderer = Self {
            system_state,
            unlit_diffuse_sub_renderer,
            depth_texture: TextureBuilder::new(device)
                .size(width, height)
                .depth_texture()
                .label("Depth Texture")
                .build()
                .expect("Failed to create depth texture"),
        };

        renderer.set_size(device, width, height);
        renderer
    }

    pub fn set_size(&mut self, device: &wgpu::Device, width: u32, height: u32) {
        self.depth_texture = TextureBuilder::new(device)
            .size(width, height)
            .depth_texture()
            .label("Depth Texture")
            .build()
            .expect("Failed to create depth texture");
    }

    pub fn render(&mut self, world: &World, output_view: &TextureView) -> CommandBuffer {
        let (render_resources, camera_query) = self.system_state.get(world);
        let device = &render_resources.device;

        // TODO: Support multiple cameras
        let main_camera = camera_query.single().0;

        // set up command encoder for render pass
        let mut encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
            label: Some("Render Encoder"),
        });

        {
            let pass_descriptor = wgpu::RenderPassDescriptor {
                label: Some("Background Pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: output_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color::BLACK),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(wgpu::RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            };
            let mut render_pass = encoder.begin_render_pass(&pass_descriptor);
            render_pass.set_bind_group(0, &main_camera.bind_group, &[]);

            self.unlit_diffuse_sub_renderer
                .render(world, &mut render_pass);
        }

        encoder.finish()
    }
}
