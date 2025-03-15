use crate::define_gpu_data_type;

use super::vertex::Vertex;

define_gpu_data_type!(super::super::shaders::basic_vertex::naga::types::BasicVertex as BasicVertex);

impl Vertex for BasicVertex {
    fn get_layout() -> wgpu::VertexBufferLayout<'static> {
        BasicVertex::vertex_layout()
    }
}
