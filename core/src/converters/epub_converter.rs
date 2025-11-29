use std::{
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::Table;

use crate::{
    converters::common::{
        add_lua_filters, combine_pandoc_native, merge_preprocessors, preprocess_cli_args,
        retrieve_combined_output, retrieve_preprocessors, run_preprocessors_on_inputs,
        run_with_logging, write_output,
    },
    file_retrieval::{get_relative_path_from_compiled_dir, get_sorted_files},
    injections::retrieve_injections,
    manifest_model::{
        DEFAULT_CUSTOM_PROCESSOR_PREPROCESSORS, Injection, MetadataSettings, Processors, Template,
    },
    nav_meta_generation::{generate_nav_meta_file, retrieve_nav_meta},
    nav_meta_generation_feature::NavMetaGenerationFeature,
    template_management::{get_output_path, get_template_path},
};

pub(crate) fn convert_epub(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &Template,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
    injections: &Vec<Injection>,
) -> Result<PathBuf> {
    debug!("Starting EPUB conversion process.");

    let output_path = get_output_path(
        template.output.clone(),
        &template.name,
        template.template_type.clone(),
    )?;

    let template_path = get_template_path(template.template_file.clone(), &template.name);

    debug!(
        "Template path: {} | Output path: {}",
        compiled_directory_path.join(&template_path).display(),
        output_path.display()
    );

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

    let nav_meta_data = if let Some(nav_meta_gen) = &template.nav_meta_gen
        && nav_meta_gen.feature != NavMetaGenerationFeature::None
    {
        let nav_meta = retrieve_nav_meta(
            &input_files,
            compiled_directory_path,
            conversion_input_dir,
            &None,
        )?;
        Some((
            nav_meta.clone(),
            generate_nav_meta_file(nav_meta_gen, &nav_meta, compiled_directory_path)?,
        ))
    } else {
        None
    };

    debug!("Processing injections.");

    debug!("Running preprocessors on inputs...");
    let results = run_preprocessors_on_inputs(
        template,
        compiled_directory_path,
        metadata_fields,
        metadata_settings,
        &nav_meta_data,
        &preprocessors,
        &input_files,
    )?;

    let pandoc_native = combine_pandoc_native(results);

    write_output(compiled_directory_path, &combined_output, &pandoc_native)?;

    debug!("Preparing pandoc command...");

    let mut processor_args = vec!["-t", "epub3", "-f", "native"]
        .iter()
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
    if let Some(processor) = &template.processor {
        processor_args.append(&mut preprocess_cli_args(
            &custom_processors
                .processors
                .iter()
                .find(|p| p.name == *processor)
                .ok_or_else(|| eyre!("Processor {} not found in custom processors.", processor))?
                .processor_args,
            metadata_fields,
        ));
    }

    let mut pandoc = Command::new("pandoc");
    pandoc
        .current_dir(compiled_directory_path)
        .args(&processor_args)
        .arg("-o")
        .arg(&output_path);

    add_meta_args(metadata_fields, &mut pandoc)?;
    debug!("Added metadata fields to pandoc command.");

    add_css_files(
        compiled_directory_path,
        &compiled_directory_path.join(&template_path),
        &mut pandoc,
    )?;
    debug!("Added CSS files from template directory if present.");

    add_fonts(
        compiled_directory_path,
        &compiled_directory_path.join(&template_path),
        &mut pandoc,
    )?;
    debug!("Added embedded fonts if present.");

    add_lua_filters(template, compiled_directory_path, &mut pandoc)?;
    debug!("Added lua filters if configured.");

    pandoc.arg(&combined_output);

    run_with_logging(pandoc, "pandoc", false)?;

    let output_path = compiled_directory_path.join(output_path);
    debug!("EPUB result path: {}", output_path.display());

    Ok(output_path)
}

fn add_meta_args(metadata_fields: &Table, pandoc: &mut Command) -> Result<()> {
    for (key, value) in metadata_fields {
        if let Some(value) = value.as_str() {
            pandoc.arg("-M").arg(format!("{}:{}", key, value));
        } else {
            return Err(eyre!(
                "Metadata field {} is not a string, and is not supported by TiefDownConverter.",
                key
            ));
        }
    }

    Ok(())
}

fn add_css_files(
    compiled_directory_path: &Path,
    template_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    let files = template_path.read_dir()?;
    for file in files {
        let file = file?.path();
        if file.is_file() && file.extension().unwrap_or_default() == "css" {
            debug!("Adding CSS file to EPUB: {}", file.display());

            pandoc.arg("-c").arg(
                get_relative_path_from_compiled_dir(&file, compiled_directory_path).unwrap_or(file),
            );
        }
    }

    Ok(())
}

fn add_fonts(
    compiled_directory_path: &Path,
    template_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    let fonts_dir = template_path.join("fonts");

    if !fonts_dir.exists() {
        return Ok(());
    }

    let font_files = fonts_dir.read_dir()?;

    for font_file in font_files {
        let font_file = font_file?.path();
        if font_file.is_file()
            && ["ttf", "otf", "woff"]
                .contains(&&*font_file.extension().unwrap_or_default().to_string_lossy())
        {
            debug!("Embedding font in EPUB: {}", font_file.display());

            pandoc.arg("--epub-embed-font").arg(
                get_relative_path_from_compiled_dir(&font_file, compiled_directory_path)
                    .unwrap_or(font_file),
            );
        }
    }

    Ok(())
}
