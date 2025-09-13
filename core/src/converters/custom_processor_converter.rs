use std::{
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::Table;

use crate::{
    converters::common::{
        combine_pandoc_native, merge_preprocessors, preprocess_cli_args, retrieve_combined_output,
        retrieve_preprocessors, run_preprocessors_on_inputs, run_with_logging,
        write_combined_output,
    },
    manifest_model::{
        DEFAULT_CUSTOM_PROCESSOR_PREPROCESSORS, MetadataSettings, Processors, TemplateMapping,
    },
};

pub(crate) fn convert_custom_processor(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &TemplateMapping,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    debug!(
        "Starting Processor conversion for template '{}'...",
        template.name
    );

    let Some(output_path) = template.output.clone() else {
        return Err(eyre!(
            "Output Path is required for Custom Pandoc conversions."
        ));
    };

    debug!("Retrieving preprocessors...");
    let default_preprocessors = retrieve_preprocessors(
        &Some(DEFAULT_CUSTOM_PROCESSOR_PREPROCESSORS.0.clone()),
        &DEFAULT_CUSTOM_PROCESSOR_PREPROCESSORS.1,
    );
    let preprocessors =
        retrieve_preprocessors(&template.preprocessors, &custom_processors.preprocessors);
    let preprocessors = merge_preprocessors(vec![preprocessors, default_preprocessors]);
    debug!(
        "Selected preprocessors: {:?}",
        preprocessors
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
    );

    let combined_output: PathBuf = retrieve_combined_output(
        template,
        &Some(DEFAULT_CUSTOM_PROCESSOR_PREPROCESSORS.0.clone()),
    )?;
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
    )?;

    let pandoc_native = combine_pandoc_native(results);

    write_combined_output(
        compiled_directory_path,
        &combined_output,
        &vec![pandoc_native],
    )?;

    debug!("Preprocessing complete.");

    let Some(processor) = &template.processor else {
        return Err(eyre!(
            "Processor is required for Custom Processor conversions.",
        ));
    };
    let processor_args = preprocess_cli_args(
        &custom_processors
            .processors
            .iter()
            .find(|p| p.name == *processor)
            .ok_or_else(|| eyre!("Processor {} not found in custom processors.", processor))?
            .processor_args,
        metadata_fields,
    );

    let mut pandoc_command = Command::new("pandoc");

    pandoc_command
        .current_dir(compiled_directory_path)
        .args(vec!["-f", "native"])
        .arg("-o")
        .arg(&output_path)
        .arg(&combined_output)
        .args(processor_args);

    run_with_logging(pandoc_command, "pandoc", false)?;

    let output_path = compiled_directory_path.join(&output_path);

    Ok(output_path)
}
