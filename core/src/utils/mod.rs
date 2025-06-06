use serde::Deserialize;

pub mod buffer;
pub mod colors;
pub mod degrees_and_radians;
pub mod primitives;
pub mod texture;

#[derive(Deserialize, Debug, Default, Clone, Copy, PartialEq)]
pub struct Bounds {
    left: f32,
    top: f32,
    right: f32,
    bottom: f32,
}

impl Bounds {
    pub fn new(left: f32, top: f32, right: f32, bottom: f32) -> Self {
        Self {
            left,
            top,
            right,
            bottom,
        }
    }

    pub fn new_with_center(center: Vec2, width: f32, height: f32) -> Self {
        Self {
            left: center.x - width / 2.0,
            top: center.y - height / 2.0,
            right: center.x + width / 2.0,
            bottom: center.y + height / 2.0,
        }
    }

    pub fn new_with_size(size: Vec2) -> Self {
        Self {
            left: 0.0,
            top: 0.0,
            right: size.x,
            bottom: size.y,
        }
    }

    pub fn width(&self) -> f32 {
        self.right - self.left
    }

    pub fn height(&self) -> f32 {
        self.bottom - self.top
    }

    pub fn left(&self) -> f32 {
        self.left
    }

    pub fn top(&self) -> f32 {
        self.top
    }

    pub fn right(&self) -> f32 {
        self.right
    }

    pub fn bottom(&self) -> f32 {
        self.bottom
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.left + self.width() / 2.0,
            self.top + self.height() / 2.0,
        )
    }

    pub fn centered_at(&self, center: Vec2) -> Self {
        Self {
            left: center.x - self.width() / 2.0,
            top: center.y + self.height() / 2.0,
            right: center.x + self.width() / 2.0,
            bottom: center.y - self.height() / 2.0,
        }
    }

    pub fn centered_within(&self, other_bounds: Bounds) -> Self {
        Self {
            left: other_bounds.left + (other_bounds.width() - self.width()) / 2.0,
            top: other_bounds.top + (other_bounds.height() - self.height()) / 2.0,
            right: other_bounds.left + (other_bounds.width() - self.width()) / 2.0,
            bottom: other_bounds.top + (other_bounds.height() - self.height()) / 2.0,
        }
    }

    pub fn translated(&self, pos_x: f32, pos_y: f32) -> Self {
        Self {
            left: self.left + pos_x,
            right: self.right + pos_x,
            top: self.top + pos_y,
            bottom: self.bottom + pos_y,
        }
    }

    pub fn scaled(&self, scale_x: f32, scale_y: f32) -> Self {
        Self {
            left: self.left * scale_x,
            right: self.right * scale_x,
            top: self.top * scale_y,
            bottom: self.bottom * scale_y,
        }
    }

    pub fn transformed(&self, pos_x: f32, pos_y: f32, scale_x: f32, scale_y: f32) -> Self {
        Self {
            left: self.left * scale_x + pos_x,
            right: self.right * scale_x + pos_x,
            top: self.top * scale_y + pos_y,
            bottom: self.bottom * scale_y + pos_y,
        }
    }

    pub fn normalized_within(&self, other_bounds: Bounds) -> Bounds {
        Bounds {
            left: (self.left - other_bounds.left) / other_bounds.width(),
            top: (self.top - other_bounds.bottom) / other_bounds.height(),
            right: (self.right - other_bounds.left) / other_bounds.width(),
            bottom: (self.bottom - other_bounds.bottom) / other_bounds.height(),
        }
    }

    pub fn get_center(&self) -> Vec2 {
        Vec2::new(
            self.left + self.width() / 2.0,
            self.top + self.height() / 2.0,
        )
    }

    pub fn get_bottom_center(&self) -> Vec2 {
        Vec2::new(self.left + self.width() / 2.0, self.bottom)
    }

    pub fn get_top_center(&self) -> Vec2 {
        Vec2::new(self.left + self.width() / 2.0, self.top)
    }

    pub fn get_left_center(&self) -> Vec2 {
        Vec2::new(self.left, self.top + self.height() / 2.0)
    }

    pub fn get_right_center(&self) -> Vec2 {
        Vec2::new(self.right, self.top + self.height() / 2.0)
    }

    pub fn get_top_left(&self) -> Vec2 {
        Vec2::new(self.left, self.top)
    }

    pub fn get_top_right(&self) -> Vec2 {
        Vec2::new(self.right, self.top)
    }

    pub fn get_bottom_left(&self) -> Vec2 {
        Vec2::new(self.left, self.bottom)
    }

    pub fn get_bottom_right(&self) -> Vec2 {
        Vec2::new(self.right, self.bottom)
    }
}
