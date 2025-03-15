use std::sync::Arc;

use bevy_ecs::world::World;

pub mod layouts;
pub mod pipelines;
pub mod render_resources;
mod shaders;
pub mod types;

pub fn initialize_gpu_resources(
    world: &mut World,
    device: Arc<wgpu::Device>,
    queue: Arc<wgpu::Queue>,
    surface_format: wgpu::TextureFormat,
) {
    let render_resources =
        render_resources::RenderResources::new(device.clone(), queue.clone(), surface_format);
    world.insert_resource(render_resources);

    layouts::initialize_bind_group_layouts(world, &device);
    pipelines::initialize_pipelines(world);
}
