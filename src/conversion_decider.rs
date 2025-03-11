use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;

use crate::{
    converters,
    manifest_model::{TemplateMapping, TemplateType},
    template_management::get_template_type_from_path,
};

type Converter = fn(
    project_directory_path: &Path,
    compiled_markdown_path: &Path,
    compiled_directory_path: &Path,
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
