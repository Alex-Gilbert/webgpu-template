use crate::{
    define_gpu_data_type,
    ecs::components::{camera::Camera, transform::Transform},
};

define_gpu_data_type!(super::super::shaders::gpu_camera::naga::types::CameraUniform as GpuCamera);

impl GpuCamera {
    pub fn from_camera_and_transform(camera: &mut Camera, transform: &mut Transform) -> Self {
        let view = transform.get_trs_matrix().inverse();
        let proj = camera.get_projection_matrix();
        Self {
            view,
            proj,
            view_proj: proj * view,
        }
    }

    pub fn update_view_proj(&mut self, camera: &mut Camera, transform: &mut Transform) -> bool {
        if camera.needs_update() || transform.needs_update() {
            self.view = transform.get_trs_matrix().inverse();
            self.proj = camera.get_projection_matrix();
            self.view_proj = self.proj * self.view;
            true
        } else {
            false
        }
    }
}
