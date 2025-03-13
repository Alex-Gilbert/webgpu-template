use glam::{Mat4, Vec3};

use crate::{
    define_gpu_data_type,
    ecs::components::{camera::Camera, transform::Transform},
};

define_gpu_data_type!(super::super::shaders::gpu_camera::naga::types::CameraUniform as GpuCamera);

impl GpuCamera {
    pub fn from_camera_and_transform(camera: &mut Camera, transform: &mut Transform) -> Self {
        let view = transform.get_trs_matrix();
        let proj = OPENGL_TO_WGPU_MATRIX * camera.get_projection_matrix();
        Self {
            view,
            proj,
            view_proj: proj * view,
        }
    }

    pub fn update_view_proj(&mut self, camera: &mut Camera, transform: &mut Transform) -> bool {
        if camera.needs_update() || transform.needs_update() {
            self.view = transform.get_trs_matrix();
            self.proj = OPENGL_TO_WGPU_MATRIX * camera.get_projection_matrix();
            self.view_proj = self.proj * self.view;
            true
        } else {
            false
        }
    }
}

#[rustfmt::skip]
const OPENGL_TO_WGPU_MATRIX: Mat4 = Mat4::from_cols_array_2d(&[
    [1.0, 0.0, 0.0, 0.0],
    [0.0, 1.0, 0.0, 0.0],
    [0.0, 0.0, 0.5, 0.5],
    [0.0, 0.0, 0.0, 1.0],
]);
