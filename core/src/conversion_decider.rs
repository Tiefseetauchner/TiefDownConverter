use crate::{
    converters::{
        custom_preprocessor_converter::convert_custom_preprocessors,
        custom_processor_converter::convert_custom_processor, epub_converter::convert_epub,
        tex_converter::convert_latex, typst_converter::convert_typst,
    },
    manifest_model::{MetadataSettings, Processors, TemplateMapping},
    template_type::TemplateType,
};
use color_eyre::eyre::Result;
use log::debug;
use std::path::{Path, PathBuf};
use toml::Table;

type Converter = fn(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &TemplateMapping,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
) -> Result<PathBuf>;

pub(crate) fn get_converter(template_type: &TemplateType) -> Result<Converter> {
    debug!("Selecting converter for template type: {:?}", template_type);
    let converter = match template_type {
        TemplateType::Tex => convert_latex,
        TemplateType::Typst => convert_typst,
        TemplateType::Epub => convert_epub,
        TemplateType::CustomPreprocessors => convert_custom_preprocessors,
        TemplateType::CustomProcessor => convert_custom_processor,
    };
    debug!("Converter selected.");
    Ok(converter)
}
