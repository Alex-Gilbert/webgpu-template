use std::collections::HashMap;

use serde::{Deserialize, Deserializer};

use crate::utils::Bounds;

use super::utils::unicode_codepoint_to_ascii_decimal;

#[derive(Deserialize, Debug)]
pub enum FontAtlasType {
    #[serde(alias = "hardmask")]
    Hardmask,

    #[serde(alias = "softmask")]
    Softmask,

    #[serde(alias = "sdf")]
    Sdf,

    #[serde(alias = "psdf")]
    Psdf,

    #[serde(alias = "msdf")]
    Msdf,

    #[serde(alias = "mtsdf")]
    Mtsdf,
}

#[derive(Deserialize, Debug)]
pub enum YOrigin {
    #[serde(alias = "top")]
    Top,
    #[serde(alias = "bottom")]
    Bottom,
}

#[derive(Deserialize, Debug)]
pub struct FontAtlas {
    #[serde(alias = "type")]
    pub atlas_type: FontAtlasType,

    #[serde(alias = "distanceRange")]
    pub distance_range: f32,

    #[serde(alias = "distanceRangeMiddle")]
    pub distance_range_middle: f32,

    pub size: f32,

    pub width: u32,
    pub height: u32,

    #[serde(alias = "yOrigin")]
    pub y_origin: YOrigin,
}

#[derive(Deserialize, Debug)]
pub struct FontMetrics {
    #[serde(alias = "emSize")]
    pub em_size: f32,

    #[serde(alias = "lineHeight")]
    pub line_height: f32,

    pub ascender: f32,
    pub descender: f32,

    #[serde(alias = "underlineY")]
    pub underline_y: f32,

    #[serde(alias = "underlineThickness")]
    pub underline_thickness: f32,
}

#[derive(Deserialize, Debug)]
pub struct GlyphWithUnicode {
    pub unicode: u32,
    pub advance: f32,

    #[serde(alias = "planeBounds")]
    pub plane_bounds: Option<Bounds>,

    #[serde(alias = "atlasBounds")]
    pub atlas_bounds: Option<Bounds>,
}

#[derive(Debug, Clone, Copy)]
pub struct Glyph {
    pub advance: f32,

    pub plane_bounds: Option<Bounds>,
    pub atlas_bounds: Option<Bounds>,
}

// helper to turn Vec<Glyph> → Ascii array of Glyphs
fn deserialize_ascii_glyphs<'de, D>(deserializer: D) -> Result<[Glyph; 128], D::Error>
where
    D: Deserializer<'de>,
{
    let v = Vec::<GlyphWithUnicode>::deserialize(deserializer)?;
    let mut glyphs: [Option<Glyph>; 128] = [const { None }; 128];

    for glyph_data in v {
        // Only include ASCII characters (0-127)
        if glyph_data.unicode < 128 {
            let glyph = Glyph {
                advance: glyph_data.advance,
                plane_bounds: glyph_data.plane_bounds,
                atlas_bounds: glyph_data.atlas_bounds,
            };
            glyphs[glyph_data.unicode as usize] = Some(glyph);
        }
    }

    // Now, replace any missing glyphs with '?'
    // We will panic if we don't have a '?' glyph (which is fine, for now...)
    // TODO: handle this better
    let qmark = glyphs['?' as usize].unwrap().clone();

    Ok(glyphs.map(|g| g.unwrap_or(qmark)))
}

#[derive(Deserialize, Debug)]
pub struct FontData {
    pub atlas: FontAtlas,
    pub metrics: FontMetrics,

    #[serde(deserialize_with = "deserialize_ascii_glyphs")]
    pub glyphs: [Glyph; 128],
}
