use bevy_ecs::system::Query;

use crate::{
    ecs::components::{rotate_component::RotateComponent, transform::Transform},
    utils::degrees_and_radians::Deg,
};

pub fn rotate_transform_system(mut query: Query<(&mut Transform, &RotateComponent)>) {
    for (mut transform, rotate_component) in query.iter_mut() {
        let rotate_amount = Deg::new(rotate_component.rotate_speed);
        transform.rotate_around(rotate_component.rotate_axis, rotate_amount.to_rad());
    }
}
