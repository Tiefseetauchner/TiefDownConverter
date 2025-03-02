use std::path::PathBuf;

use color_eyre::eyre::{eyre, Result};

use crate::converters;

pub fn get_converter(template: &str) -> Result<fn(&PathBuf, &str) -> Result<PathBuf>> {
    if template.ends_with(".tex") {
        Ok(converters::convert_latex)
    } else if template.trim_matches('/').ends_with("_epub") {
        Ok(converters::convert_epub)
    } else {
        Err(eyre!(
            "Unsupported template type for template name '{}'.",
            template
        ))
    }
}
