use std::path::{Path, PathBuf};

use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::Table;

use crate::{
    converters::common::{
        get_sorted_files, retrieve_combined_output, retrieve_injections, retrieve_output_extension,
        retrieve_preprocessors, run_preprocessors_on_inputs, write_combined_output,
        write_single_file_outputs,
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
            "Custom Preprocessor templates cannot have a processor. Use preprocessors instead.",
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
            "Output Path is required for Custom Preprocessor conversions."
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

    debug!("Collecting input files for preprocessing...");

    let injections = retrieve_injections(template, injections, conversion_input_dir)?;

    let input_files = get_sorted_files(
        conversion_input_dir,
        project_directory_path,
        compiled_directory_path,
        injections,
    )?;

    debug!("Found {} input files.", input_files.len());

    debug!("Running preprocessors on inputs...");
    let results = run_preprocessors_on_inputs(
        template,
        project_directory_path,
        compiled_directory_path,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        &input_files,
    )?;

    let combined_output = retrieve_combined_output(template, &None)?;

    if template.multi_file_output.unwrap_or(false) {
        let output_extension = retrieve_output_extension(template, &None)?;

        write_single_file_outputs(
            project_directory_path,
            compiled_directory_path,
            conversion_input_dir,
            &output_path,
            output_extension,
            &input_files,
            &results,
        )?;
    } else if let Some(combined_output) = combined_output {
        debug!("Combined output file: {}", combined_output.display());
        write_combined_output(compiled_directory_path, &combined_output, &results)?;
    } else {
        return Err(eyre!(
            "Either multi-file output must be enabled or a combined output be set."
        ));
    }

    debug!("Preprocessing complete.");

    let output_path = compiled_directory_path.join(&output_path);
    debug!("CustomPreprocessor result path: {}", output_path.display());

    Ok(output_path)
}
