use crate::{include_wgsl_shader, include_wgsl_shader_vertex_fragment};
mod shader_macros;

include_wgsl_shader!(r#"include/basic_vertex.wgsl"#, basic_vertex);
include_wgsl_shader!(r#"include/camera_h.wgsl"#, gpu_camera);
include_wgsl_shader!(r#"include/model_h.wgsl"#, gpu_model);

include_wgsl_shader_vertex_fragment!(r#"unlit_diffuse.wgsl"#, unlit_diffuse);
