use std::path::PathBuf;

use color_eyre::eyre::Result;

use crate::{
    converters,
    manifest_model::{TemplateMapping, TemplateType},
    template_management::get_template_type_from_path,
};

pub fn get_converter(
    template: &str,
) -> Result<
    fn(
        compiled_markdown_path: &PathBuf,
        compiled_directory_path: &PathBuf,
        template: &TemplateMapping,
    ) -> Result<PathBuf>,
> {
    let template_type = get_template_type_from_path(template)?;

    return match template_type {
        TemplateType::Tex => Ok(converters::convert_latex),
        TemplateType::Typst => Ok(converters::convert_typst),
        TemplateType::Epub => Ok(converters::convert_epub),
    };
}
