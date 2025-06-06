use crate::utils::colors::Color;

use super::font_data::FontData;

#[derive(Debug, Clone)]
pub struct FontStyle<'a> {
    pub font: &'a FontData,
    pub size: f32,
    pub color: Color,
}

impl<'a> FontStyle<'a> {
    pub fn new(font: &'a FontData) -> Self {
        Self {
            font,
            size: 1.0,
            color: Color::WHITE,
        }
    }

    pub fn line_height(&self) -> f32 {
        self.font.metrics.line_height * self.size
    }

    pub fn get_descender(&self) -> f32 {
        self.font.metrics.descender * self.size
    }

    pub fn calculate_width(&self, text: &str) -> f32 {
        text.chars()
            .map(|ch| self.font.glyphs[ch as usize].advance)
            .sum()
    }
}
