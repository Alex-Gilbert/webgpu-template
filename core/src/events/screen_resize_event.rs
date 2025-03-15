use bevy_ecs::{
    event::{Event, Events},
    system::Resource,
};

#[derive(Event)]
pub struct ScreenResizeEvent {
    pub width: u32,
    pub height: u32,
}

#[derive(Resource)]
pub struct ScreenResizeEvents {
    pub events: Events<ScreenResizeEvent>,
}
