use bevy_ecs::component::Component;
use glam::{Quat, Vec3};

use crate::utils::degrees_and_radians::{Deg, Rad};

/// Simple transform component
/// This is meant to represent a local transform
#[derive(Component, Default, Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: Quat,
    pub scale: Vec3,
}

impl Transform {
    /// Creates an identity transform
    pub fn identity() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
        }
    }

    pub fn from_trs(translation: Vec3, rotation: Quat, scale: Vec3) -> Self {
        Self {
            translation,
            rotation,
            scale,
        }
    }

    pub fn from_translation(translation: Vec3) -> Self {
        Self {
            translation,
            rotation: Quat::IDENTITY,
            scale: Vec3::ONE,
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
    }

    pub fn translate(&mut self, delta: Vec3) {
        self.translation += delta;
    }

    pub fn rotate(&mut self, delta: Quat) {
        self.rotation *= delta;
    }

    pub fn scale(&mut self, delta: Vec3) {
        self.scale *= delta;
    }

    pub fn set_position(&mut self, position: Vec3) {
        self.translation = position;
    }

    pub fn set_rotation(&mut self, rotation: Quat) {
        self.rotation = rotation;
    }

    pub fn set_scale(&mut self, scale: Vec3) {
        self.scale = scale;
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
    }

    pub fn add_pitch(&mut self, angle: Deg<f32>) {
        self.rotation = self.rotation * Quat::from_axis_angle(Vec3::X, angle.to_rad().0);
    }

    pub fn add_roll(&mut self, angle: Deg<f32>) {
        self.rotation = self.rotation * Quat::from_axis_angle(Vec3::Z, angle.to_rad().0);
    }

    pub fn get_matrix(&self) -> glam::Mat4 {
        glam::Mat4::from_scale_rotation_translation(self.scale, self.rotation, self.translation)
    }

    /// Multiplies this transform by another transform (self * other)
    /// This applies the other transform after this transform
    pub fn mul_transform(&self, other: &Transform) -> Transform {
        // For accurate transform multiplication, we need to decompose properly
        // This is equivalent to: self.matrix * other.matrix, but maintains TRS components

        // Apply this transform's scale to the other's translation and scale
        let new_translation = self.translation + self.rotation * (self.scale * other.translation);
        let new_rotation = self.rotation * other.rotation;
        let new_scale = self.scale * other.scale;

        Transform::from_trs(new_translation, new_rotation, new_scale)
    }

    /// Multiplies this transform by another transform in-place (self = self * other)
    pub fn mul_assign(&mut self, other: &Transform) {
        let result = self.mul_transform(other);
        self.translation = result.translation;
        self.rotation = result.rotation;
        self.scale = result.scale;
    }

    /// Returns the inverse of this transform
    pub fn inverse(&self) -> Transform {
        let inv_scale = Vec3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
        let inv_rotation = self.rotation.inverse();
        let inv_translation = inv_rotation * (-self.translation * inv_scale);

        Transform::from_trs(inv_translation, inv_rotation, inv_scale)
    }

    /// Transforms a point by this transform
    pub fn transform_point(&self, point: Vec3) -> Vec3 {
        self.translation + self.rotation * (self.scale * point)
    }

    /// Transforms a vector by this transform (ignores translation)
    pub fn transform_vector(&self, vector: Vec3) -> Vec3 {
        self.rotation * (self.scale * vector)
    }

    /// Transforms a direction by this transform (ignores translation and scale)
    pub fn transform_direction(&self, direction: Vec3) -> Vec3 {
        self.rotation * direction
    }

    /// Inverse transforms a point by this transform
    pub fn inverse_transform_point(&self, point: Vec3) -> Vec3 {
        let inv_scale = Vec3::new(1.0 / self.scale.x, 1.0 / self.scale.y, 1.0 / self.scale.z);
        let inv_rotation = self.rotation.inverse();
        inv_rotation * ((point - self.translation) * inv_scale)
    }

    /// Linearly interpolates between this transform and another
    pub fn lerp(&self, other: &Transform, t: f32) -> Transform {
        Transform::from_trs(
            self.translation.lerp(other.translation, t),
            self.rotation.slerp(other.rotation, t),
            self.scale.lerp(other.scale, t),
        )
    }

    /// Creates a transform from a matrix (decomposes TRS components)
    pub fn from_matrix(matrix: glam::Mat4) -> Transform {
        let (scale, rotation, translation) = matrix.to_scale_rotation_translation();
        Transform::from_trs(translation, rotation, scale)
    }
}

// Implement standard operators for convenient transform multiplication
impl std::ops::Mul<Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        self.mul_transform(&other)
    }
}

impl std::ops::Mul<&Transform> for Transform {
    type Output = Transform;

    fn mul(self, other: &Transform) -> Transform {
        self.mul_transform(other)
    }
}

impl std::ops::Mul<Transform> for &Transform {
    type Output = Transform;

    fn mul(self, other: Transform) -> Transform {
        self.mul_transform(&other)
    }
}

impl std::ops::Mul<&Transform> for &Transform {
    type Output = Transform;

    fn mul(self, other: &Transform) -> Transform {
        self.mul_transform(other)
    }
}

impl std::ops::MulAssign<Transform> for Transform {
    fn mul_assign(&mut self, other: Transform) {
        self.mul_assign(&other);
    }
}

impl std::ops::MulAssign<&Transform> for Transform {
    fn mul_assign(&mut self, other: &Transform) {
        self.mul_assign(other);
    }
}
