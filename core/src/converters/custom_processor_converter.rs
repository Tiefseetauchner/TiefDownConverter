use crate::{
    converters::common::{
        add_lua_filters, combine_pandoc_native, get_sorted_files, merge_preprocessors,
        preprocess_cli_args, retrieve_combined_output, retrieve_injections, retrieve_preprocessors,
        run_preprocessors_on_inputs, run_with_logging, write_combined_output,
    },
    manifest_model::{
        DEFAULT_CUSTOM_PROCESSOR_PREPROCESSORS, Injection, MetadataSettings, Processors, Template,
    },
};
use color_eyre::eyre::{Result, eyre};
use log::debug;
use std::{
    path::{Path, PathBuf},
    process::Command,
};
use toml::Table;

pub(crate) fn convert_custom_processor(
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

    let combined_output = retrieve_combined_output(
        template,
        &Some(DEFAULT_CUSTOM_PROCESSOR_PREPROCESSORS.0.clone()),
    )?;

    if template.multi_file_output.unwrap_or(false) || combined_output.is_none() {
        return Err(eyre!(
            "Multi-file outputs are currently not supported for templatetype '{}'.",
            template.template_type
        ));
    }

    let combined_output = combined_output.unwrap();

    debug!("Combined output file: {}", combined_output.display());

    debug!("Collecting input files for preprocessing...");

    let injections = retrieve_injections(
        template,
        project_directory_path,
        compiled_directory_path,
        injections,
    )?;

    let input_files = get_sorted_files(
        conversion_input_dir,
        project_directory_path,
        compiled_directory_path,
        &injections,
        template.multi_file_output.unwrap_or(false),
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
        &injections,
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

    add_lua_filters(
        template,
        project_directory_path,
        compiled_directory_path,
        &mut pandoc_command,
    )?;

    pandoc_command
        .current_dir(compiled_directory_path)
        .args(vec!["-f", "native"])
        .arg("-o")
        .arg(&output_path)
        .args(processor_args)
        .arg(&combined_output);

    run_with_logging(pandoc_command, "pandoc", false)?;

    let output_path = compiled_directory_path.join(&output_path);

    Ok(output_path)
}
