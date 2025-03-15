use bevy_ecs::system::Resource;

#[derive(Debug, Resource)]
pub struct Time {
    pub delta_time: f32,
    pub total_time: f32,
    pub frame_count: u64,
}

impl Default for Time {
    fn default() -> Self {
        Self::new()
    }
}

impl Time {
    pub fn new() -> Self {
        Time {
            delta_time: 0.0,
            total_time: 0.0,
            frame_count: 0,
        }
    }

    pub fn new_frame(&mut self, delta_time: f32) {
        self.delta_time = delta_time;
        self.total_time += self.delta_time;
        self.frame_count += 1;
    }
}
