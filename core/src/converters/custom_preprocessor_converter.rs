use std::path::{Path, PathBuf};

use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::Table;

use crate::{
    converters::common::{
        retrieve_combined_output, retrieve_preprocessors, run_preprocessors_on_inputs,
        write_combined_output,
    },
    manifest_model::{Injection, MetadataSettings, Processors, Template},
    template_type::TemplateType,
};

pub(crate) fn convert_custom_preprocessors(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &Template,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
    injections: &Vec<Injection>,
) -> Result<PathBuf> {
    debug!(
        "Starting CustomPandoc conversion for template '{}'...",
        template.name
    );
    if template.processor != None {
        return Err(eyre!(
            "Custom Pandoc templates cannot have a processor. Use preprocessors instead.",
        ));
    }

    if template.preprocessors.is_none() {
        return Err(eyre!(
            "Template type {} has to define a preprocessor.",
            TemplateType::CustomPreprocessors
        ));
    }

    let output_path: Option<PathBuf> = template.output.clone();

    let Some(output_path) = output_path else {
        return Err(eyre!(
            "Output Path is required for Custom Pandoc conversions."
        ));
    };

    debug!("Retrieving preprocessors...");
    let preprocessors =
        retrieve_preprocessors(&template.preprocessors, &custom_processors.preprocessors);
    debug!(
        "Selected preprocessors: {:?}",
        preprocessors
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
    );

    let combined_output: PathBuf = retrieve_combined_output(template, &None)?;
    debug!("Combined output file: {}", combined_output.display());

    debug!("Running preprocessors on inputs...");
    let results = run_preprocessors_on_inputs(
        template,
        project_directory_path,
        compiled_directory_path,
        conversion_input_dir,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        injections,
    )?;

    write_combined_output(compiled_directory_path, &combined_output, &results)?;

    debug!("Preprocessing complete.");

    let output_path = compiled_directory_path.join(&output_path);
    debug!("CustomPandoc result path: {}", output_path.display());

    Ok(output_path)
}
