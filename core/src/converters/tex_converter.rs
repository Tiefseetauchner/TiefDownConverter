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
        run_preprocessors_on_inputs, run_with_logging, write_combined_output,
    },
    manifest_model::{
        DEFAULT_TEX_PREPROCESSORS, Injection, MetadataSettings, Processors, Template,
    },
    template_management::{get_output_path, get_template_path},
};

pub fn convert_latex(
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
        "Starting LaTeX conversion for template '{}'...",
        template.name
    );
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = compiled_directory_path.join(get_output_path(
        template.output.clone(),
        &template.name,
        template.template_type.clone(),
    )?);
    debug!(
        "Template path: {} | Output path: {}",
        compiled_directory_path.join(&template_path).display(),
        output_path.display()
    );

    let preprocessors =
        retrieve_preprocessors(&template.preprocessors, &custom_processors.preprocessors);
    let default_preprocessors = retrieve_preprocessors(
        &Some(DEFAULT_TEX_PREPROCESSORS.0.clone()),
        &DEFAULT_TEX_PREPROCESSORS.1,
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
        retrieve_combined_output(template, &Some(DEFAULT_TEX_PREPROCESSORS.0.clone()))?;
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

    debug!("Generating LaTeX metadata...");
    generate_tex_metadata(compiled_directory_path, metadata_fields, metadata_settings)?;

    let mut processor_args = vec![];

    if let Some(processor) = &template.processor {
        if let Some(processor_pos) = custom_processors
            .processors
            .iter()
            .position(|p| p.name == *processor)
        {
            debug!("Adding processor args from '{}'.", processor);
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

    debug!("Compiling LaTeX (first pass)...");
    compile_latex(compiled_directory_path, &template_path, &processor_args)?;
    debug!("Compiling LaTeX (second pass)...");
    compile_latex(compiled_directory_path, &template_path, &processor_args)?;

    let template_path = compiled_directory_path.join(template_path.with_extension("pdf"));
    if template_path.exists() && template_path.as_os_str() != output_path.as_os_str() {
        debug!(
            "Copying compiled PDF from '{}' to '{}'",
            template_path.display(),
            output_path.display()
        );
        fs::copy(&template_path, &output_path)?;
    }

    debug!("LaTeX result path: {}", output_path.display());
    Ok(output_path)
}

fn generate_tex_metadata(
    compiled_directory_path: &Path,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
) -> Result<()> {
    let metadata_path = compiled_directory_path.join("metadata.tex");
    if metadata_path.exists() {
        return Ok(());
    }

    let mut metadata_file = fs::File::create(&metadata_path)?;
    let mut metadata_file_content = String::new();

    let prefix = metadata_settings
        .metadata_prefix
        .as_deref()
        .unwrap_or("meta");

    metadata_file_content.push_str(
        format!(
            r"\newcommand{{\{}}}[1]{{\csname {}@#1\endcsname}}",
            prefix, prefix
        )
        .as_str(),
    );

    metadata_file_content.push_str("\n\n");

    for (key, value) in metadata_fields {
        if let Some(value) = value.as_str() {
            metadata_file_content.push_str(&format!(
                r"\expandafter\def\csname {}@{}\endcsname{{{}}}",
                prefix, key, value
            ));
            metadata_file_content.push('\n');
        } else {
            return Err(eyre!(
                "Metadata field {} is not a string, and is not supported by TiefDownConverter.",
                key
            ));
        }
    }

    metadata_file.write_all(metadata_file_content.as_bytes())?;

    Ok(())
}

fn compile_latex(
    compiled_directory_path: &Path,
    template_path: &Path,
    processor_args: &Vec<String>,
) -> Result<()> {
    let mut latex_command = Command::new("xelatex");

    latex_command
        .current_dir(compiled_directory_path)
        .arg("-interaction=nonstopmode")
        .arg("-synctex=1")
        .arg(template_path)
        .args(processor_args);

    run_with_logging(latex_command, "xelatex", false)?;

    Ok(())
}
