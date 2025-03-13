#define TEXTURE_GROUP 0
#define TEXTURE_BINDING 0
#import include/texture_sampler.wgsl as diffuse

#define MODEL_GROUP 1
#import include/model.wgsl

#define CAMERA_GROUP 2
#import include/camera.wgsl

#import include/basic_vertex.wgsl

struct VertexOutput {
    @builtin(position) clip_position: vec4<f32>,
    @location(0) tex_coords: vec2<f32>,
};

@vertex
fn vs_main(in: basic_vertex::BasicVertex) -> VertexOutput {
    var out: VertexOutput;
    out.tex_coords = in.tex_coords;
    out.clip_position = camera::to_clip(model::to_world(in.position));
    return out;
}


@fragment
fn fs_main(in: VertexOutput) -> @location(0) vec4<f32> {
    return diffuse::sample_2D(in.tex_coords.xy);
}
