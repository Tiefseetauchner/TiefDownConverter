use std::{error::Error, path::PathBuf};

use crate::converters;

pub fn get_converter(
    template: &str,
) -> Result<fn(&PathBuf, &String) -> Result<PathBuf, Box<dyn Error>>, Box<dyn Error>> {
    if template.ends_with(".tex") {
        Ok(converters::convert_latex)
    } else if template.trim_matches('/').ends_with("_epub") {
        Ok(converters::convert_epub)
    } else {
        Err(format!(
            "Unsupported template type for template name '{}'.",
            template
        )
        .into())
    }
}
