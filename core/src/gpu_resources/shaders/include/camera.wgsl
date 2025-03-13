#import camera_h.wgsl

// Bind the camera uniform buffer
@group(#CAMERA_GROUP) @binding(0)
var<uniform> camera: camera_h::CameraUniform;

fn aspect() -> f32 {
    return camera.aspect;
}

fn to_clip(pos: vec3<f32>) -> vec4<f32> {
    return camera.view_proj * vec4<f32>(pos, 1.0);
}
