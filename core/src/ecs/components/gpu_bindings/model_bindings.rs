use bevy_ecs::component::Component;
use bevy_ecs::world::World;
use wgpu::Queue;
use wgpu::util::DeviceExt;

use crate::ecs::components::transform::Transform;

use crate::gpu_resources::{
    layouts::model_uniform_layout::ModelUniformLayout, types::gpu_model::GpuModel,
};

use crate::gpu_resources::types::gpu_type_macros::GpuUniformType;

#[derive(Component, Debug)]
pub struct ModelBindings {
    pub bind_group: wgpu::BindGroup,
    buffer: wgpu::Buffer,
    gpu_model: GpuModel,
}

impl ModelBindings {
    pub fn new(world: &World, device: &wgpu::Device, transform: &mut Transform) -> Self {
        let model_bind_group_layout = world.get_resource::<ModelUniformLayout>().unwrap();
        let gpu_model = GpuModel::from_transform(transform);

        let model_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Model Buffer"),
            contents: &gpu_model.as_buffer(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let model_bind_group = model_bind_group_layout.create_bind_group(device, &model_buffer);

        Self {
            bind_group: model_bind_group,
            buffer: model_buffer,
            gpu_model,
        }
    }

    pub fn update(&mut self, queue: &Queue, transform: &mut Transform) {
        if self.gpu_model.update_model(transform) {
            queue.write_buffer(&self.buffer, 0, &self.gpu_model.as_buffer());
        }
    }
}
