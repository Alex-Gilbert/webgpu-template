@group(#TEXTURE_GROUP) @binding(#TEXTURE_BINDING * 2)
var texture: texture_2d<f32>;
@group(#TEXTURE_GROUP) @binding(#TEXTURE_BINDING * 2 + 1)
var texture_sampler: sampler;

fn sample_2D(uv: vec2<f32>) -> vec4<f32> {
    return textureSample(texture, texture_sampler, uv);
}
