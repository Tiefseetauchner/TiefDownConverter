use std::{
    path::{Path, PathBuf},
    process::Command,
};

use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::Table;

use crate::{
    converters::common::{
        add_lua_filters, get_relative_path_from_compiled_dir, get_sorted_files, run_with_logging,
    },
    manifest_model::{MetadataSettings, Processors, TemplateMapping},
    template_management::{get_output_path, get_template_path},
};

pub(crate) fn convert_epub(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &TemplateMapping,
    metadata_fields: &Table,
    _metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    debug!("Starting EPUB conversion for template '{}'...", template.name);
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    )?;
    debug!(
        "Template path: {} | Output path: {}",
        compiled_directory_path.join(&template_path).display(),
        output_path.display()
    );

    let template_path = compiled_directory_path.join(template_path);

    let mut pandoc = Command::new("pandoc");
    pandoc
        .current_dir(compiled_directory_path)
        .arg("-f")
        .arg("markdown")
        .arg("-t")
        .arg("epub3")
        .arg("-o")
        .arg(&output_path);
    debug!("Initialized pandoc command for EPUB output.");

    add_meta_args(metadata_fields, &mut pandoc)?;
    debug!("Added {} metadata entries to pandoc.", metadata_fields.len());

    add_css_files(
        project_directory_path,
        compiled_directory_path,
        &template_path,
        &mut pandoc,
    )?;
    debug!("Added CSS files from template directory if present.");
    add_fonts(
        project_directory_path,
        compiled_directory_path,
        &template_path,
        &mut pandoc,
    )?;
    debug!("Added embedded fonts if present.");

    add_lua_filters(
        template,
        project_directory_path,
        compiled_directory_path,
        &mut pandoc,
    )?;
    debug!("Added lua filters if configured.");

    if let Some(processor) = &template.processor {
        if let Some(processor_pos) = custom_processors
            .processors
            .iter()
            .position(|p| p.name == *processor)
        {
            debug!("Adding processor args from '{}'.", processor);
            pandoc.args(
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

    let sorted_files = get_sorted_files(
        conversion_input_dir,
        project_directory_path,
        compiled_directory_path,
    )?;
    debug!("Adding {} input files to pandoc.", sorted_files.len());
    pandoc.args(sorted_files);

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
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    template_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    let css_files = template_path.read_dir()?;
    for css_file in css_files {
        let css_file = css_file?.path();
        if css_file.is_file() && css_file.extension().unwrap_or_default() == "css" {
            pandoc.arg("-c").arg(
                get_relative_path_from_compiled_dir(
                    &css_file,
                    project_directory_path,
                    compiled_directory_path,
                )
                .unwrap_or(css_file),
            );
        }
    }

    Ok(())
}

fn add_fonts(
    project_directory_path: &Path,
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
            pandoc.arg("--epub-embed-font").arg(
                get_relative_path_from_compiled_dir(
                    &font_file,
                    project_directory_path,
                    compiled_directory_path,
                )
                .unwrap_or(font_file),
            );
        }
    }

    Ok(())
}
