use std::{
    fs,
    io::Write,
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::Table;

use crate::{
    converters::common::{
        merge_preprocessors, retrieve_combined_output, retrieve_preprocessors,
        run_preprocessors_on_injections, run_preprocessors_on_inputs, run_with_logging,
        write_combined_output,
    },
    file_retrieval::get_sorted_files,
    injections::retrieve_injections,
    manifest_model::{
        DEFAULT_TYPST_PREPROCESSORS, Injection, MetadataSettings, Processors, Template,
    },
    template_management::{get_output_path, get_template_path},
};

pub(crate) fn convert_typst(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &Template,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
    injections: &Vec<Injection>,
) -> Result<PathBuf> {
    debug!("Starting Typst conversion...");

    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = get_output_path(
        template.output.clone(),
        &template.name,
        template.template_type.clone(),
    )?;
    debug!(
        "Template path: {} | Output path: {}",
        compiled_directory_path.join(&template_path).display(),
        output_path.display()
    );

    let preprocessors =
        retrieve_preprocessors(&template.preprocessors, &custom_processors.preprocessors);
    let default_preprocessors = retrieve_preprocessors(
        &Some(DEFAULT_TYPST_PREPROCESSORS.0.clone()),
        &DEFAULT_TYPST_PREPROCESSORS.1,
    );
    let preprocessors = merge_preprocessors(vec![preprocessors, default_preprocessors]);

    debug!(
        "Using preprocessors: {:?}",
        preprocessors
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>()
    );

    let combined_output =
        retrieve_combined_output(template, &Some(DEFAULT_TYPST_PREPROCESSORS.0.clone()))?;

    if template.multi_file_output.unwrap_or(false) || combined_output.is_none() {
        return Err(eyre!(
            "Multi-file outputs are currently not supported for templatetype '{}'.",
            template.template_type
        ));
    }

    let combined_output = combined_output.unwrap();

    debug!("Collecting input files for preprocessing...");

    let injections = retrieve_injections(template, compiled_directory_path, injections)?;

    let input_files = get_sorted_files(
        conversion_input_dir,
        project_directory_path,
        compiled_directory_path,
        &injections,
        template.multi_file_output.unwrap_or(false),
    )?;
    debug!("Found {} input files.", input_files.len());

    debug!("Processing injections.");

    let header_injection_output = run_preprocessors_on_injections(
        template,
        project_directory_path,
        compiled_directory_path,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        &injections.header_injections,
    )?;

    let footer_injection_output = run_preprocessors_on_injections(
        template,
        project_directory_path,
        compiled_directory_path,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        &injections.footer_injections,
    )?;

    let results = run_preprocessors_on_inputs(
        template,
        project_directory_path,
        compiled_directory_path,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        &input_files,
    )?;

    write_combined_output(
        compiled_directory_path,
        &combined_output,
        &results,
        &header_injection_output,
        &footer_injection_output,
    )?;

    debug!("Generating Typst metadata...");

    generate_typst_metadata(compiled_directory_path, metadata_fields, metadata_settings)?;

    let mut processor_args = vec![];

    debug!("Compiling Typst document...");

    if let Some(processor) = &template.processor {
        if let Some(processor_pos) = custom_processors
            .processors
            .iter()
            .position(|p| p.name == *processor)
        {
            processor_args.extend(
                custom_processors.processors[processor_pos]
                    .processor_args
                    .clone(),
            );
        } else {
            return Err(eyre!(
                "Processor {} not found in custom processors.",
                processor
            ));
        }
    }

    let mut typst_command = Command::new("typst");

    typst_command
        .current_dir(compiled_directory_path)
        .arg("compile")
        .arg(template_path)
        .arg(&output_path)
        .args(processor_args);

    run_with_logging(typst_command, "typst", false)?;

    let output_path = compiled_directory_path.join(output_path);
    debug!("Typst result path: {}", output_path.display());

    Ok(output_path)
}

fn generate_typst_metadata(
    compiled_directory_path: &Path,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
) -> Result<()> {
    let metadata_path = compiled_directory_path.join("metadata.typ");
    if metadata_path.exists() {
        return Ok(());
    }

    let mut metadata_file = fs::File::create(&metadata_path)?;
    let mut metadata_file_content = String::new();

    let prefix = metadata_settings
        .metadata_prefix
        .as_deref()
        .unwrap_or("meta");

    metadata_file_content.push_str(format!(r"#let {} = (", prefix).as_str());
    metadata_file_content.push_str("\n");

    for (key, value) in metadata_fields.iter() {
        if let Some(value) = value.as_str() {
            metadata_file_content.push_str(format!(r#"  {}: "{}","#, key, value).as_str());
            metadata_file_content.push_str("\n");
        } else {
            return Err(eyre!(
                "Metadata field {} is not a string, and is not supported by TiefDownConverter.",
                key
            ));
        }
    }

    metadata_file_content.push_str(")");

    metadata_file.write_all(metadata_file_content.as_bytes())?;

    Ok(())
}
