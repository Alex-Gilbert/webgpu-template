use bevy_ecs::component::Component;

#[derive(Component, Default)]
pub struct RotateComponent {
    pub rotate_axis: glam::Vec3,
    pub rotate_speed: f32,
}
