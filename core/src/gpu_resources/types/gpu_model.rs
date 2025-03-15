use glam::Mat3;

use crate::{define_gpu_data_type, ecs::components::transform::Transform};

define_gpu_data_type!(super::super::shaders::gpu_model::naga::types::ModelUniform as GpuModel);

impl GpuModel {
    pub fn from_transform(transform: &mut Transform) -> Self {
        Self {
            model: transform.get_trs_matrix(),
            normal_matrix: Mat3::from_mat4(transform.get_trs_matrix().inverse().transpose()),
        }
    }

    pub fn update_model(&mut self, transform: &mut Transform) -> bool {
        if transform.needs_update() {
            self.model = transform.get_trs_matrix();
            self.normal_matrix = Mat3::from_mat4(transform.get_trs_matrix().inverse().transpose());
            true
        } else {
            false
        }
    }
}
