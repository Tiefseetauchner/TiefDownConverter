use std::path::{Path, PathBuf};

use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::Table;

use crate::{
    converters::common::{
        retrieve_combined_output, retrieve_output_extension, retrieve_preprocessors,
        run_preprocessors_on_injections, run_preprocessors_on_inputs, write_combined_output,
        write_multi_file_outputs,
    },
    file_retrieval::get_sorted_files,
    injections::retrieve_injections,
    manifest_model::{Injection, MetadataSettings, Processors, Template},
    nav_meta_generation::{generate_nav_meta_file, retrieve_nav_meta},
    nav_meta_generation_feature::NavMetaGenerationFeature,
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

    let injections = retrieve_injections(template, compiled_directory_path, injections)?;

    let input_files = get_sorted_files(
        conversion_input_dir,
        project_directory_path,
        compiled_directory_path,
        &injections,
        template.multi_file_output.unwrap_or(false),
    )?;

    debug!("Found {} input files.", input_files.len());

    debug!("Retrieving navigation metadata.");

    let nav_meta_path = if let Some(nav_meta_gen) = &template.nav_meta_gen
        && nav_meta_gen.feature != NavMetaGenerationFeature::None
    {
        let nav_meta =
            retrieve_nav_meta(&input_files, compiled_directory_path, conversion_input_dir)?;
        Some(generate_nav_meta_file(
            nav_meta_gen,
            &nav_meta,
            compiled_directory_path,
        )?)
    } else {
        None
    };

    // TODO: There needs to be a nav meta generated for each file processed if multi-file processing is enabled to allow injection of current file data
    //       This needs to include the conversion of header and footer injections
    debug!("Running preprocessors on inputs...");
    let results = run_preprocessors_on_inputs(
        template,
        project_directory_path,
        compiled_directory_path,
        metadata_fields,
        metadata_settings,
        &nav_meta_path,
        &preprocessors,
        &input_files,
    )?;

    let combined_output = retrieve_combined_output(template, &None)?;

    if template.multi_file_output.unwrap_or(false) {
        let header_injections = run_preprocessors_on_injections(
            template,
            project_directory_path,
            compiled_directory_path,
            metadata_fields,
            metadata_settings,
            &nav_meta_path,
            &preprocessors,
            &injections.header_injections,
        )?;
        let footer_injections = run_preprocessors_on_injections(
            template,
            project_directory_path,
            compiled_directory_path,
            metadata_fields,
            metadata_settings,
            &nav_meta_path,
            &preprocessors,
            &injections.footer_injections,
        )?;

        let output_extension = retrieve_output_extension(template, &None)?;

        write_multi_file_outputs(
            project_directory_path,
            compiled_directory_path,
            conversion_input_dir,
            &output_path,
            output_extension,
            &input_files,
            &header_injections,
            &footer_injections,
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
