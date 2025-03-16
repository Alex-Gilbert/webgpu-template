use crate::{
    gpu_resources::types::basic_vertex::BasicVertex,
    utils::buffer::{Buffer, BufferBuilder},
};
use bevy_ecs::component::Component;
use bytemuck::{Pod, Zeroable};

/// Trait to associate index types with their corresponding wgpu::IndexFormat
pub trait IndexType: Pod + Zeroable {
    const INDEX_FORMAT: wgpu::IndexFormat;
}

impl IndexType for u16 {
    const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint16;
}

impl IndexType for u32 {
    const INDEX_FORMAT: wgpu::IndexFormat = wgpu::IndexFormat::Uint32;
}

#[derive(Component)]
pub struct BasicMeshFilter {
    pub filter: MeshFilter<BasicVertex, u32>,
}

pub struct MeshFilter<V: Pod + Zeroable, I: IndexType> {
    pub vertex_buffer: Buffer<V>,
    pub index_buffer: Buffer<I>,
    pub index_count: u32,
    pub index_format: wgpu::IndexFormat,
}

impl<V: Pod + Zeroable, I: IndexType> MeshFilter<V, I> {
    pub fn new(device: &wgpu::Device, vertices: &[V], indices: &[I]) -> Self {
        Self {
            vertex_buffer: BufferBuilder::new(device)
                .contents(vertices)
                .usage(wgpu::BufferUsages::VERTEX)
                .label("Vertex Buffer")
                .build()
                .expect("Failed to create vertex buffer"),
            index_buffer: BufferBuilder::new(device)
                .contents(indices)
                .usage(wgpu::BufferUsages::INDEX)
                .label("Index Buffer")
                .build()
                .expect("Failed to create index buffer"),
            index_count: indices.len() as u32,
            index_format: I::INDEX_FORMAT,
        }
    }

    pub fn draw<'w, 'a>(&'w self, render_pass: &mut wgpu::RenderPass<'a>)
    where
        'w: 'a,
    {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice());
        render_pass.set_index_buffer(self.index_buffer.slice(), self.index_format);
        render_pass.draw_indexed(0..self.index_count, 0, 0..1);
    }

    pub fn draw_instanced<'w, 'a>(
        &'w self,
        render_pass: &mut wgpu::RenderPass<'a>,
        instance_count: u32,
    ) where
        'w: 'a,
    {
        render_pass.set_vertex_buffer(0, self.vertex_buffer.slice());
        render_pass.set_index_buffer(self.index_buffer.slice(), self.index_format);
        render_pass.draw_indexed(0..self.index_count, 0, 0..instance_count);
    }
}
