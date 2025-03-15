use bevy_ecs::prelude::*;
use glam::Mat4;

use super::transform::Transform;

/// Enum defining the projection type
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum ProjectionType {
    Perspective,
    Orthographic,
}

/// A camera component supporting both perspective and orthographic projections
#[derive(Component)]
pub struct Camera {
    /// Type of projection to use
    pub projection_type: ProjectionType,

    // Common parameters
    /// Aspect ratio (width / height)
    pub aspect_ratio: f32,
    /// Near clipping plane distance
    pub near: f32,
    /// Far clipping plane distance
    pub far: f32,

    // Perspective parameters
    /// Field of view in radians (vertical) - used for perspective only
    pub fov: f32,
    /// Whether to use infinite projection - perspective only
    pub infinite_projection: bool,
    /// Whether to use reversed depth (better precision) - affects both projections
    pub reversed_depth: bool,

    // Orthographic parameters
    /// The size (height) of the orthographic view
    pub ortho_size: f32,

    // Cached projection matrix
    projection_matrix: Option<Mat4>,
}

impl Default for Camera {
    fn default() -> Self {
        Self {
            projection_type: ProjectionType::Perspective,
            aspect_ratio: 16.0 / 9.0,
            near: 0.1,
            far: 1000.0,
            fov: std::f32::consts::PI / 4.0, // 45 degrees
            infinite_projection: false,
            reversed_depth: false,
            ortho_size: 10.0,
            projection_matrix: None,
        }
    }
}

impl Camera {
    /// Creates a new perspective camera
    pub fn new_perspective(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            projection_type: ProjectionType::Perspective,
            aspect_ratio,
            near,
            far,
            fov,
            infinite_projection: false,
            reversed_depth: false,
            ortho_size: 10.0, // Default, not used in perspective
            projection_matrix: None,
        }
    }

    /// Creates a new orthographic camera
    pub fn new_orthographic(size: f32, aspect_ratio: f32, near: f32, far: f32) -> Self {
        Self {
            projection_type: ProjectionType::Orthographic,
            aspect_ratio,
            near,
            far,
            fov: std::f32::consts::PI / 4.0, // Default, not used in orthographic
            infinite_projection: false,
            reversed_depth: false,
            ortho_size: size,
            projection_matrix: None,
        }
    }

    /// Sets the aspect ratio and marks the projection matrix as dirty
    pub fn set_aspect_ratio(&mut self, aspect_ratio: f32) {
        self.aspect_ratio = aspect_ratio;
        self.projection_matrix = None;
    }

    /// Sets the field of view (perspective only) and marks the projection matrix as dirty
    pub fn set_fov(&mut self, fov: f32) {
        self.fov = fov;
        self.projection_matrix = None;
    }

    /// Sets the orthographic size (orthographic only) and marks the projection matrix as dirty
    pub fn set_ortho_size(&mut self, size: f32) {
        self.ortho_size = size;
        self.projection_matrix = None;
    }

    /// Sets the near clip plane and marks the projection matrix as dirty
    pub fn set_near(&mut self, near: f32) {
        self.near = near;
        self.projection_matrix = None;
    }

    /// Sets the far clip plane and marks the projection matrix as dirty
    pub fn set_far(&mut self, far: f32) {
        self.far = far;
        self.projection_matrix = None;
    }

    /// Changes the projection type and marks the projection matrix as dirty
    pub fn set_projection_type(&mut self, projection_type: ProjectionType) {
        self.projection_type = projection_type;
        self.projection_matrix = None;
    }

    /// Updates the projection matrix based on the current parameters
    pub fn get_projection_matrix(&mut self) -> Mat4 {
        if let Some(projection_matrix) = self.projection_matrix {
            projection_matrix
        } else {
            let projection_matrix = match self.projection_type {
                ProjectionType::Perspective => {
                    match (self.infinite_projection, self.reversed_depth) {
                        (true, false) => {
                            Mat4::perspective_infinite_rh(self.fov, self.aspect_ratio, self.near)
                        }

                        (true, true) => Mat4::perspective_infinite_reverse_rh(
                            self.fov,
                            self.aspect_ratio,
                            self.near,
                        ),
                        (false, false) => {
                            Mat4::perspective_rh(self.fov, self.aspect_ratio, self.near, self.far)
                        }
                        (false, true) => {
                            Mat4::perspective_rh(self.fov, self.aspect_ratio, self.far, self.near)
                        }
                    }
                }
                ProjectionType::Orthographic => {
                    // Calculate orthographic dimensions
                    let height = self.ortho_size;
                    let width = height * self.aspect_ratio;

                    // Create orthographic projection matrix
                    if self.reversed_depth {
                        // Reversed depth for orthographic
                        Mat4::orthographic_rh(
                            -width / 2.0,
                            width / 2.0,
                            -height / 2.0,
                            height / 2.0,
                            self.far,
                            self.near, // Swap near and far for reversed depth
                        )
                    } else {
                        Mat4::orthographic_rh(
                            -width / 2.0,
                            width / 2.0,
                            -height / 2.0,
                            height / 2.0,
                            self.near,
                            self.far,
                        )
                    }
                }
            };
            self.projection_matrix = Some(projection_matrix);
            projection_matrix
        }
    }

    /// Creates a combined view-projection matrix
    pub fn view_projection_matrix(&mut self, transform: &mut Transform) -> Mat4 {
        let view = transform.get_trs_matrix();
        self.get_projection_matrix() * view
    }

    pub fn needs_update(&self) -> bool {
        self.projection_matrix.is_none()
    }
}
