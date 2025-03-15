use bevy_ecs::system::Resource;

#[derive(Resource)]
pub struct ScreenParameters {
    pub width: u32,
    pub height: u32,
}

impl ScreenParameters {
    pub fn new(width: u32, height: u32) -> Self {
        Self { width, height }
    }

    pub fn set_size(&mut self, width: u32, height: u32) {
        self.width = width;
        self.height = height;
    }
}
