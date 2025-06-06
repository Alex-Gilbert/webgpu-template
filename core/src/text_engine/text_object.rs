use std::{collections::HashMap, marker::PhantomData};

use crate::{
    asset_management::Handle, ecs::components::mesh_filter::MeshFilter,
    gpu_resources::types::font_types::FontVertex, utils::Bounds,
};

use super::{
    font_style::FontStyle,
    interpolation_value::InterpolationValue,
    line_builder::build_lines,
    text_segment::TextSegment,
    variable_enum::{EmptyVariableStorage, EnumVariableStorage, VariableEnum, VariableStorage},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum HorizontalAlign {
    Left,
    Center,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum VerticalAlign {
    Top,
    Middle,
    Bottom,
}

pub struct TextObject {
    pub text_segments: Vec<TextSegment>,

    pub variables: Box<dyn VariableStorage>,

    pub auto_line_break: bool,
    pub size_to_fit: bool,
    pub h_align: HorizontalAlign,
    pub v_align: VerticalAlign,
    pub bounds: Bounds,

    dirty: bool,
    max_style_id: usize,
}

impl TextObject {
    /// Create static text object with no variable interpolation
    pub fn new(text: String) -> Self {
        Self {
            text_segments: vec![TextSegment::new(text, 0)],
            variables: Box::new(EmptyVariableStorage::new()),
            auto_line_break: false,
            size_to_fit: false,
            h_align: HorizontalAlign::Center,
            v_align: VerticalAlign::Middle,
            bounds: Bounds::default(),
            dirty: true,
            max_style_id: 0,
        }
    }

    /// Create with a specific enum type's variable storage
    pub fn new_with_variables<T: VariableEnum>(text: String) -> Self {
        Self {
            text_segments: vec![TextSegment::new(text, 0)],
            variables: Box::new(EnumVariableStorage::<T>::new()),
            auto_line_break: false,
            size_to_fit: false,
            h_align: HorizontalAlign::Center,
            v_align: VerticalAlign::Middle,
            bounds: Bounds::default(),
            dirty: true,
            max_style_id: 0,
        }
    }

    /// Create with pre-configured variable storage
    pub fn new_with_storage(text: String, variables: Box<dyn VariableStorage>) -> Self {
        Self {
            text_segments: vec![TextSegment::new(text, 0)],
            variables,
            auto_line_break: false,
            size_to_fit: false,
            h_align: HorizontalAlign::Center,
            v_align: VerticalAlign::Middle,
            bounds: Bounds::default(),
            dirty: true,
            max_style_id: 0,
        }
    }

    pub fn with_bounds(mut self, bounds: Bounds) -> Self {
        self.bounds = bounds;
        self.dirty = true;
        self
    }

    pub fn with_auto_line_break(mut self, auto_line_break: bool) -> Self {
        self.auto_line_break = auto_line_break;
        self.dirty = true;
        self
    }

    pub fn sized_to_fit(mut self, auto_size: bool) -> Self {
        self.size_to_fit = auto_size;
        self.dirty = true;
        self
    }

    pub fn with_h_align(mut self, h_align: HorizontalAlign) -> Self {
        self.h_align = h_align;
        self.dirty = true;
        self
    }

    pub fn with_v_align(mut self, v_align: VerticalAlign) -> Self {
        self.v_align = v_align;
        self.dirty = true;
        self
    }

    pub fn set_variable(&mut self, name: &str, value: impl Into<InterpolationValue>) {
        if self.variables.set_value_by_name(name, value.into()) {
            self.dirty = true;
        }
    }

    pub fn get_variable(&self, name: &str) -> Option<&InterpolationValue> {
        self.variables.get_value(name)
    }

    pub fn with_variable(mut self, name: &str, value: impl Into<InterpolationValue>) -> Self {
        self.set_variable(name, value);
        self.dirty = true;
        self
    }

    pub fn with_text_segment(mut self, text_segment: TextSegment) -> Self {
        self.max_style_id = text_segment.style_id.max(self.max_style_id);
        self.text_segments.push(text_segment);
        self.dirty = true;
        self
    }

    pub fn set_clean(&mut self) {
        self.dirty = false;
    }

    pub fn needs_update(&self) -> bool {
        self.dirty
    }

    pub fn tesselate(&self, styles: &[&FontStyle]) -> Vec<(Vec<FontVertex>, Vec<u32>)> {
        let mut lines = build_lines(
            &self.text_segments,
            styles,
            self.variables.as_ref(),
            self.bounds.width(),
        );

        let total_line_height: f32 = lines.iter().map(|l| l.height).sum();

        // calculate the total glyphs per style
        let mut total_glyphs_per_style: Vec<usize> = Vec::with_capacity(styles.len());
        for line in lines.iter() {
            for style_range in line.style_ranges.iter() {
                for glyph in line.glyphs[style_range.start..style_range.end].iter() {
                    if !glyph.is_ascii_whitespace() {
                        total_glyphs_per_style[style_range.style_index] += 1;
                    }
                }
            }
        }

        // create a vertex and index buffer using the total number of glyphs per style
        let mut vert_index_buffers: Vec<(Vec<FontVertex>, Vec<u32>)> = total_glyphs_per_style
            .iter()
            .map(|&c| {
                (
                    Vec::<FontVertex>::with_capacity(c * 4),
                    Vec::<u32>::with_capacity(c * 6),
                )
            })
            .collect();

        let mut cursor_y = match self.v_align {
            VerticalAlign::Top => self.bounds.top(),
            VerticalAlign::Middle => (self.bounds.height() - total_line_height) / 2.0,
            VerticalAlign::Bottom => self.bounds.bottom() - total_line_height,
        };

        let mut vertex_index = 0;
        let mut index_index = 0;

        for line in lines.iter_mut() {
            let line_bottom = cursor_y - line.height;
            let mut cursor_x = match self.h_align {
                HorizontalAlign::Left => 0.0,
                HorizontalAlign::Center => (self.bounds.width() - line.width) / 2.0,
                HorizontalAlign::Right => self.bounds.width() - line.width,
            };

            for style_range in line.style_ranges.iter() {
                let style = styles[style_range.style_index];
                let baseline_y = line_bottom + style.get_descender();

                for glyph in &line.glyphs[style_range.start..style_range.end] {
                    if let Some((vertex_buffer, index_buffer)) =
                        vert_index_buffers.get_mut(style_range.style_index)
                    {
                        let glyph_data = style.font.glyphs[*glyph as u8 as usize];
                        if let Some(plane_bounds) = glyph_data.plane_bounds {
                            if let Some(atlas_bounds) = glyph_data.atlas_bounds {
                                let translated_plane_bounds = plane_bounds
                                    .transformed(cursor_x, baseline_y, style.size, style.size);
                                let normalized_plane_bounds =
                                    plane_bounds.normalized_within(self.bounds);

                                // 1 ------ 2
                                // |       |
                                // |       |
                                // 0 ------ 3
                                vertex_buffer.push(FontVertex {
                                    position: translated_plane_bounds.get_bottom_left().into(),
                                    color: style.color.into(),
                                    altas_coords: atlas_bounds.get_bottom_left(),
                                    glyph_coords: Vec2::new(0.0, 0.0),
                                    bounds_coords: normalized_plane_bounds.get_bottom_left(),
                                });

                                vertex_buffer.push(FontVertex {
                                    position: translated_plane_bounds.get_top_left().into(),
                                    color: style.color.into(),
                                    altas_coords: atlas_bounds.get_top_left(),
                                    glyph_coords: Vec2::new(0.0, 1.0),
                                    bounds_coords: normalized_plane_bounds.get_top_left(),
                                });

                                vertex_buffer.push(FontVertex {
                                    position: translated_plane_bounds.get_top_right().into(),
                                    color: style.color.into(),
                                    altas_coords: atlas_bounds.get_top_right(),
                                    glyph_coords: Vec2::new(1.0, 1.0),
                                    bounds_coords: normalized_plane_bounds.get_top_right(),
                                });

                                vertex_buffer.push(FontVertex {
                                    position: translated_plane_bounds.get_bottom_right().into(),
                                    color: style.color.into(),
                                    altas_coords: atlas_bounds.get_bottom_right(),
                                    glyph_coords: Vec2::new(1.0, 0.0),
                                    bounds_coords: normalized_plane_bounds.get_bottom_right(),
                                });

                                index_buffer.push(0);
                                index_buffer.push(1);
                                index_buffer.push(2);
                                index_buffer.push(0);
                                index_buffer.push(2);
                                index_buffer.push(3);
                            }
                        }
                        // Advance cursor horizonally
                        // TODO: Handle kerning pairs and different kerning styles
                        cursor_x += glyph_data.advance * style.size;
                    }
                }
                // Advance cursor vertically
                cursor_y -= line.height;
            }
        }

        vert_index_buffers
    }
}
