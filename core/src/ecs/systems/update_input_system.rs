use bevy_ecs::system::ResMut;

use crate::ecs::resources::input::Input;

pub fn update_input_system(mut input: ResMut<Input>) {
    input.as_mut().update();
}
