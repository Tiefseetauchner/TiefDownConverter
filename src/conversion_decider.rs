use std::path::PathBuf;

use color_eyre::eyre::Result;

use crate::{
    converters,
    manifest_model::{TemplateMapping, TemplateType},
    template_management::get_template_type_from_path,
};

type Converter = fn(
    project_directory_path: &PathBuf,
    compiled_markdown_path: &PathBuf,
    compiled_directory_path: &PathBuf,
    template: &TemplateMapping,
) -> Result<PathBuf>;

pub fn get_converter(template: &str) -> Result<Converter> {
    let template_type = get_template_type_from_path(template)?;

    Ok(match template_type {
        TemplateType::Tex => converters::convert_latex,
        TemplateType::Typst => converters::convert_typst,
        TemplateType::Epub => converters::convert_epub,
    })
}
