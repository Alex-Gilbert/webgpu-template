use bevy_ecs::{bundle::Bundle, world::World};
use glam::Vec3;

use crate::ecs::components::{
    camera::Camera, gpu_bindings::camera_bindings::CameraBindings, transform::Transform,
};

#[derive(Bundle)]
pub struct CameraBundle {
    camera: Camera,
    transform: Transform,
    camera_bindings: CameraBindings,
}

impl CameraBundle {
    pub fn new(world: &World, eye: Vec3, target: Vec3, up: Vec3) -> Self {
        let mut transform = Transform::from_translation(eye);
        transform.look_at(target, up);
        let mut camera = Camera::default();

        let camera_bindings = CameraBindings::new(world, &mut camera, &mut transform);

        Self {
            camera,
            transform,
            camera_bindings,
        }
    }
}
