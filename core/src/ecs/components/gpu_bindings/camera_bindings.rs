use bevy_ecs::{system::Resource, world::World};
use glam::Mat4;
use wgpu::Queue;
use wgpu::util::DeviceExt;

use crate::{
    components::camera::Camera,
    gpu_resources::{
        layouts::camera_uniform_layout::CameraUniformLayout, types::gpu_camera::GpuCamera,
    },
};

#[derive(Resource, Debug)]
pub struct CameraBindings {
    buffer: wgpu::Buffer,
    gpu_camera: GpuCamera,
    pub bind_group: wgpu::BindGroup,
}

impl CameraBindings {
    pub fn new(
        world: &World,
        device: &wgpu::Device,
        camera: &mut Camera,
        transform: &mut Transform,
    ) -> Self {
        let camera_bind_group_layout = world.get_resource::<CameraUniformLayout>().unwrap();
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

    pub fn update(&self, queue: &Queue, camera: &mut Camera, transform: &mut Transform) {
        if self.gpu_camera.update_view_proj(camera, transform) {
            queue.write_buffer(&self.buffer, 0, &self.gpu_camera.as_buffer());
        }
    }
}
