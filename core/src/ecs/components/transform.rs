use bevy_ecs::component::Component;
use glam::{Quat, Vec3};

use crate::utils::degrees_and_radians::{Deg, Rad};

/// Simple transform component to pair with the camera
#[derive(Component, Default)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
    matrix: Option<glam::Mat4>,
}

impl Transform {
    pub fn from_trs(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
            matrix: None,
        }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
            matrix: None,
        }
    }

    pub fn look_at(&mut self, target: Vec3, up: Vec3) {
        // Calculate forard direction (z-axis)
        let forward = (target - self.translation).normalize();

        // Calculate right direction (x-axis) as cross product of forward and up
        let right = up.cross(forward).normalize();

        // Recalculate up to ensure orthogonality (y-axis)
        let corrected_up = forward.cross(right).normalize();

        // Create rotation from the orthonormal basis
        let mat3 = glam::Mat3::from_cols(right, corrected_up, forward);
        self.rotation = Quat::from_mat3(&mat3);
        self.matrix = None;
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.translation += delta;
        self.matrix = None;
    }

    pub fn rotate(&mut self, delta: Quat) {
        self.rotation *= delta;
        self.matrix = None;
    }

    pub fn scale(&mut self, delta: Vec3) {
        self.scale *= delta;
        self.matrix = None;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.translation = position;
        self.matrix = None;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
        self.matrix = None;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
        self.matrix = None;
    }

    pub fn forward(&self) -> Vec3 {
        self.rotation * Vec3::Z
    }

    pub fn up(&self) -> Vec3 {
        self.rotation * Vec3::Y
    }

    pub fn right(&self) -> Vec3 {
        self.rotation * Vec3::X
    }

    pub fn rotate_around(&mut self, axis: Vec3, angle: Rad<f32>) {
        self.rotation *= Quat::from_axis_angle(axis.normalize(), angle.0);
        self.matrix = None;
    }

    pub fn yaw(&self) -> f32 {
        self.rotation.to_euler(glam::EulerRot::YXZ).2
    }

    pub fn pitch(&self) -> f32 {
        self.rotation.to_euler(glam::EulerRot::YXZ).1
    }

    pub fn roll(&self) -> f32 {
        self.rotation.to_euler(glam::EulerRot::YXZ).0
    }

    pub fn add_yaw(&mut self, angle: Deg<f32>) {
        self.rotation = self.rotation * Quat::from_axis_angle(Vec3::Y, angle.to_rad().0);
        self.matrix = None;
    }

    pub fn add_pitch(&mut self, angle: Deg<f32>) {
        self.rotation = self.rotation * Quat::from_axis_angle(Vec3::X, angle.to_rad().0);
        self.matrix = None;
    }

    pub fn add_roll(&mut self, angle: Deg<f32>) {
        self.rotation = self.rotation * Quat::from_axis_angle(Vec3::Z, angle.to_rad().0);
        self.matrix = None;
    }

    pub fn get_trs_matrix(&mut self) -> glam::Mat4 {
        if let Some(mat) = self.matrix {
            mat
        } else {
            let mat = glam::Mat4::from_scale_rotation_translation(
                self.scale,
                self.rotation,
                self.translation,
            );
            self.matrix = Some(mat);
            mat
        }
    }

    pub fn needs_update(&self) -> bool {
        self.matrix.is_none()
    }
}
