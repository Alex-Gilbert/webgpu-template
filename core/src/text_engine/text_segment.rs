use std::collections::HashMap;

use crate::asset_management::Handle;

use super::{
    font_style::FontStyle,
    interpolation_value::InterpolationValue,
    variable_enum::{VariableEnum, VariableStorage},
};

#[derive(Debug, Clone)]
pub struct TextSegment {
    pub template: String, // Text with {variable} placeholders
    pub style_id: usize,  // Font style index
}

impl TextSegment {
    pub fn new(template: String, style_id: usize) -> Self {
        Self { template, style_id }
    }

    pub fn get_text(&self, variables: &dyn VariableStorage) -> String {
        let mut result = self.template.clone();
        let mut pos = 0;

        // Keep looking for placeholders until none are left
        while pos < result.len() {
            if let Some(start) = result[pos..].find('{') {
                if let Some(end) = result[start..].find('}') {
                    let end = start + end;

                    // Extract the variable name (between { and })
                    let var_name = &result[start + 1..end];

                    let replacement = variables
                        .get_value(var_name)
                        .map(|v| v.as_string())
                        .unwrap_or_else(|| {
                            // Variable exists but no value set
                            String::new() // Replace with empty
                        });

                    // Replace the {variable} with the actual value
                    result.replace_range(start..=end, &replacement);
                    pos = start + replacement.len(); // Skip to next placeholder
                } else {
                    pos += 1; // Skip to next character
                }
            }
        }

        result
    }
}
