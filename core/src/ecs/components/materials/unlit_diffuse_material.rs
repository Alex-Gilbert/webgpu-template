use bevy_ecs::{component::Component, world::World};

use crate::{
    gpu_resources::{
        layouts::texture_uniform_layout::TextureUniformLayout, render_resources::RenderResources,
    },
    utils::texture::Texture,
};

#[derive(Component)]
pub struct UnlitDiffuseMaterial {
    pub bind_group: wgpu::BindGroup,
}

impl UnlitDiffuseMaterial {
    pub fn new(world: &World, texture: &Texture) -> Self {
        let render_resources: &RenderResources = world.get_resource::<RenderResources>().unwrap();

        let texture_uniform_layout: &TextureUniformLayout<1> =
            world.get_resource::<TextureUniformLayout<1>>().unwrap();

        let device = &render_resources.device;

        let bind_group = texture_uniform_layout.create_complete_bind_group(device, &[texture]);

        Self { bind_group }
    }
}
