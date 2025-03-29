use std::path::{Path, PathBuf};

use color_eyre::eyre::Result;

use crate::{
    converters,
    manifest_model::{Processors, TemplateMapping, TemplateType},
};

type Converter = fn(
    project_directory_path: &Path,
    compiled_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    custom_processors: &Processors,
) -> Result<PathBuf>;

pub fn get_converter(template_type: &TemplateType) -> Result<Converter> {
    Ok(match template_type {
        TemplateType::Tex => converters::convert_latex,
        TemplateType::Typst => converters::convert_typst,
        TemplateType::Epub => converters::convert_epub,
        TemplateType::CustomPandoc => converters::convert_custom_pandoc,
    })
}
