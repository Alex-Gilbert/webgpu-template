use bevy_ecs::component::Component;

use crate::text_engine::text_object::TextObject;

#[derive(Component, Default)]
pub struct TextComponent {
    pub text_object: TextObject,
    pub styles: Vec<Handle<FontStyle>>,
}

impl TextComponent {
    pub fn new(text_object: TextObject) -> Self {
        Self {
            text_object,
            styles: Vec::new(),
        }
    }

    /// Add a new style to the text object
    pub fn add_style(&mut self, style: Handle<FontStyle>) {
        self.styles.push(style);
    }
}
