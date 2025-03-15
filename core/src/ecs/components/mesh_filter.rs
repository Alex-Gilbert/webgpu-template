use bevy_ecs::component::Component;
use bytemuck::Pod;
use wgpu::util::DeviceExt;

#[derive(Component)]
pub struct MeshFilter {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub index_count: u32,
    pub index_format: wgpu::IndexFormat,
}

impl MeshFilter {
    pub fn new<V: Pod>(device: &wgpu::Device, vertices: &[V], indices: &[u32]) -> Self {
        Self {
            vertex_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Vertex Buffer"),
                contents: bytemuck::cast_slice(vertices),
                usage: wgpu::BufferUsages::VERTEX,
            }),
            index_buffer: device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("Index Buffer"),
                contents: bytemuck::cast_slice(indices),
                usage: wgpu::BufferUsages::INDEX,
            }),
            index_count: indices.len() as u32,
            index_format: wgpu::IndexFormat::Uint32,
        }
    }
}
