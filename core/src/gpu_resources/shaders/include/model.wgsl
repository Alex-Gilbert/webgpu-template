#import model_h.wgsl

// Bind the model uniform buffer to the specified group and binding 0
@group(#MODEL_GROUP) @binding(0)
var<uniform> model: model_h::ModelUniform;

// Function to transform a position from model space to world space
fn to_world(position: vec3<f32>) -> vec3<f32> {
    // Apply the model matrix to transform the position
    let world_pos = model.model * vec4<f32>(position, 1.0);
    return world_pos.xyz;
}

// Function to transform a normal from model space to world space
// Normals require the normal_matrix (inverse transpose of model matrix)
// to ensure they remain perpendicular to surfaces after transformation
fn transform_normal(normal: vec3<f32>) -> vec3<f32> {
    // Use the normal matrix to transform the normal vector
    // This preserves perpendicularity even with non-uniform scaling
    return normalize(model.normal_matrix * normal);
}
