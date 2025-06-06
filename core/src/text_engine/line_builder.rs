use std::{collections::HashMap, ops::Index, ptr};

use super::{
    font_style::FontStyle, interpolation_value::InterpolationValue, text_segment::TextSegment,
    variable_enum::VariableStorage,
};

pub struct StyleRange {
    pub start: usize,
    pub end: usize,
    pub style_index: usize,
}

pub struct Line {
    pub glyphs: Vec<char>,
    pub style_ranges: Vec<StyleRange>,
    pub width: f32,
    pub height: f32,
}

impl Line {
    fn new() -> Self {
        Self {
            glyphs: Vec::new(),
            style_ranges: Vec::new(),
            width: 0.0,
            height: 0.0,
        }
    }

    /// Support a new style by updating the height and baseline
    fn adjust_height(&mut self, line_height: f32) {
        let height = line_height;
        if self.height < height {
            self.height = height;
        }
    }

    fn with_height(mut self, height: f32) -> Self {
        self.height = height;
        self
    }

    fn push_glyphs(&mut self, glyphs: &[char], width: f32, style_index: usize, line_height: f32) {
        self.glyphs.extend_from_slice(glyphs);
        self.width += width;
        self.adjust_height(line_height);

        // If the last style range uses the same style... simply update the end of the previous
        // style range
        if let Some(last_style_range) = self.style_ranges.last_mut() {
            if last_style_range.style_index == style_index {
                last_style_range.end = self.glyphs.len();
                return;
            }
        }

        self.style_ranges.push(StyleRange {
            start: self.glyphs.len() - glyphs.len(),
            end: self.glyphs.len(),
            style_index,
        });
    }
}

fn get_words_and_spaces(text: &str) -> Vec<&str> {
    let mut start = 0;
    let mut words_and_spaces = Vec::new();

    for (i, ch) in text.char_indices() {
        if ch.is_ascii_whitespace() {
            if !start.eq(&i) {
                words_and_spaces.push(&text[start..i]);
            }
            words_and_spaces.push(&text[i..i + 1]);
            start = i + 1;
        }
    }

    // Add the last word
    if !start.eq(&text.len()) {
        words_and_spaces.push(&text[start..]);
    }

    words_and_spaces
}

pub fn build_lines(
    text_segments: &[TextSegment],
    styles: &[&FontStyle],
    vars: &dyn VariableStorage,
    max_width: f32,
) -> Vec<Line> {
    let mut lines = Vec::new();
    lines.push(Line::new());

    for text_segment in text_segments {
        let text = text_segment.get_text(vars);
        let words = get_words_and_spaces(&text);
        let style = styles[text_segment.style_id];
        let line_height = style.line_height();

        for word in words {
            if word.len() == 1 && word.as_bytes()[0] == b'\n' {
                // when we see a line break no glyphs are added to the line.
                // However we need to make sure the line has the correct height and baseline of the current style
                lines.last_mut().unwrap().adjust_height(line_height);
                continue;
            }

            let font_style = styles[text_segment.style_id];
            let width = font_style.calculate_width(word);

            // If the line cannot fit the next word, create a new one
            if lines.last().is_none_or(|l| l.width + width > max_width) {
                lines.push(Line::new().with_height(font_style.line_height()));
            }

            lines.last_mut().unwrap().push_glyphs(
                word.chars().collect::<Vec<char>>().as_slice(),
                width,
                text_segment.style_id,
                line_height,
            );
        }
    }

    lines
}
