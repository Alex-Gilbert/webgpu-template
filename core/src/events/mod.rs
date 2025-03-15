pub mod screen_resize_event;

use bevy_ecs::{event::Events, system::ResMut, world::World};
use screen_resize_event::{ScreenResizeEvent, ScreenResizeEvents};

pub fn init_events(world: &mut World) {
    let screen_resize_events = ScreenResizeEvents {
        events: Events<ScreenResizeEvent>::default(),
    };

    world.insert_resource(screen_resize_events);
}

/// The update system for events... run after late update
pub fn update_events_system(mut screen_resize_events: ResMut<ScreenResizeEvents>) {
    screen_resize_events.events.update();
}
