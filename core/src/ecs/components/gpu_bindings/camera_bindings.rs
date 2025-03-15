use bevy_ecs::{component::Component, world::World};
use wgpu::Queue;
use wgpu::util::DeviceExt;

use crate::{
    ecs::components::{camera::Camera, transform::Transform},
    gpu_resources::{
        layouts::camera_uniform_layout::CameraUniformLayout,
        render_resources::RenderResources,
        types::{gpu_camera::GpuCamera, gpu_type_macros::GpuUniformType},
    },
};

#[derive(Component, Debug)]
pub struct CameraBindings {
    buffer: wgpu::Buffer,
    gpu_camera: GpuCamera,
    pub bind_group: wgpu::BindGroup,
}

impl CameraBindings {
    pub fn new(world: &World, camera: &mut Camera, transform: &mut Transform) -> Self {
        let camera_bind_group_layout = world.get_resource::<CameraUniformLayout>().unwrap();
        let device = &world.get_resource::<RenderResources>().unwrap().device;

        let gpu_camera = GpuCamera::from_camera_and_transform(camera, transform);

        let camera_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Camera Buffer"),
            contents: &gpu_camera.as_buffer(),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
        });

        let camera_bind_group = camera_bind_group_layout.create_bind_group(device, &camera_buffer);

        Self {
            buffer: camera_buffer,
            bind_group: camera_bind_group,
            gpu_camera,
        }
    }

    pub fn update(&mut self, queue: &Queue, camera: &mut Camera, transform: &mut Transform) {
        if self.gpu_camera.update_view_proj(camera, transform) {
            queue.write_buffer(&self.buffer, 0, &self.gpu_camera.as_buffer());
        }
    }
}
