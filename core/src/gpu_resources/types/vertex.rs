pub trait Vertex: bytemuck::Pod + bytemuck::Zeroable + 'static {
    fn get_layout() -> wgpu::VertexBufferLayout<'static>;
}
