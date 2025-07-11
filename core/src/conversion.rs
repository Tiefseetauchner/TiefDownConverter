use crate::conversion_decider;
use crate::manifest_model::Manifest;
use crate::manifest_model::MarkdownProject;
use crate::manifest_model::MetadataSettings;
use crate::manifest_model::Processors;
use crate::manifest_model::TemplateMapping;
use crate::project_management::get_missing_dependencies;
use crate::project_management::load_and_convert_manifest;
use crate::project_management::run_smart_clean;
use chrono::prelude::DateTime;
use chrono::prelude::Utc;
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use fs_extra::dir;
use fs_extra::file;
use log::debug;
use log::error;
use log::info;
use log::warn;
use std::path::Path;
use std::path::PathBuf;
use toml::Table;

/// Converts a TiefDown project to specified templates.
///
/// Runs the conversion process for all markdown projects in the project.
///
/// If no templates are specified, all templates are converted, a profile will be tried.
///
/// If no profile is specified, the default profile for the corresponding markdown project is used.
///
/// If no profile is specified and no default profile is available, all templates are converted.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
/// ** Defaults to the current directory if not provided.
/// * `templates` - A list of template names to convert to.
/// ** Defaults to all templates if not provided.
/// * `profile` - The name of the profile to use for conversion.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn convert(
    project: Option<String>,
    templates: Option<Vec<String>>,
    profile: Option<String>,
) -> Result<()> {
    let pandoc_errors = get_missing_dependencies(vec!["pandoc"])?;

    if !pandoc_errors.is_empty() {
        error!("{}", pandoc_errors.join("\n"));
        return Err(eyre!("Pandoc is not installed or not in the PATH."));
    }

    let other_dependencies = get_missing_dependencies(vec!["xelatex", "typst"])?;

    if !other_dependencies.is_empty() {
        warn!("{}", other_dependencies.join("\n"));
        warn!(
            "Some dependencies are missing. Some features may not work, and conversion may fail."
        );
    }

    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = Path::new(&project);

    if !project_path.exists() {
        return Err(eyre!("Project path does not exist."));
    }

    let manifest_path = project_path.join("manifest.toml");
    let manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(true) = manifest.smart_clean {
        let threshold = manifest.smart_clean_threshold.unwrap_or(5);
        run_smart_clean(project_path, threshold.saturating_sub(1))?;
    }

    info!("Converting project: {}", project);

    let compiled_directory_path = create_build_directory(project_path)?;

    debug!(
        "Converting in directory: {}",
        compiled_directory_path.display()
    );

    for markdown_project in manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![MarkdownProject {
            name: "Default".to_string(),
            path: PathBuf::from("Markdown"),
            output: PathBuf::from("."),
            metadata_fields: None,
            default_profile: None,
            resources: None,
        }])
    {
        info!("Converting markdown project: {}", markdown_project.name);

        let profile = profile.clone().or(markdown_project.default_profile.clone());

        if let Some(ref profile) = profile {
            debug!("Using profile: {}", profile);
        }

        let templates = get_template_names(&templates, profile, &manifest)?;
        let templates = get_template_mappings_from_names(&templates, &manifest)?;
        let markdown_project_compiled_directory_path =
            compiled_directory_path.join(markdown_project.output.clone());

        dir::create_all(&markdown_project_compiled_directory_path, false)?;
        dir::copy(
            project_path.join("template/"),
            &markdown_project_compiled_directory_path,
            &dir::CopyOptions::new().overwrite(true).content_only(true),
        )?;

        debug!("Copied template directory.");

        let input_dir = project_path.join(markdown_project.path.clone());

        copy_resources(
            &markdown_project,
            &markdown_project_compiled_directory_path,
            &input_dir,
        )?;

        let shared_metadata = manifest.shared_metadata.clone().unwrap_or(Table::new());
        let project_metadata = markdown_project.metadata_fields.unwrap_or(Table::new());

        let merged_metadata = merge_metadata(&shared_metadata, &project_metadata);

        debug!(
            "Merged {} metadata fields ({} shared, {} project specific).",
            merged_metadata.len(),
            shared_metadata.len(),
            project_metadata.len()
        );

        for template in &templates {
            let conversion_input_dir =
                &markdown_project_compiled_directory_path.join(template.name.clone() + "_convdir/");

            copy_markdown_directory(
                &input_dir,
                &conversion_input_dir,
                &markdown_project.resources,
            )?;

            convert_template(
                &markdown_project_compiled_directory_path,
                template,
                project_path,
                &conversion_input_dir,
                &markdown_project.output,
                &merged_metadata,
                &manifest.metadata_settings,
                &manifest.custom_processors,
            )?;
        }
    }

    Ok(())
}

fn copy_resources(
    markdown_project: &MarkdownProject,
    markdown_project_compiled_directory_path: &PathBuf,
    markdown_dir: &PathBuf,
) -> Result<()> {
    for resource in markdown_project.resources.clone().unwrap_or(vec![]) {
        let resource = markdown_dir.join(resource.clone());

        if !resource.exists() {
            return Err(eyre!(
                "Resource file {} does not exist.",
                resource.display()
            ));
        }

        if resource.is_dir() {
            debug!("Copying directory: {}", resource.display());

            dir::copy(
                resource,
                markdown_project_compiled_directory_path,
                &dir::CopyOptions::new().overwrite(true).content_only(false),
            )?;
        } else {
            debug!("Copying file: {}", resource.display());

            file::copy(
                &resource,
                &markdown_project_compiled_directory_path
                    .join(resource.file_name().unwrap_or(std::ffi::OsStr::new("."))),
                &file::CopyOptions::new().overwrite(true),
            )?;
        }
    }

    Ok(())
}

fn merge_metadata(shared_metadata: &Table, project_metadata: &Table) -> Table {
    let mut merged_metadata = shared_metadata.clone();
    for (key, value) in project_metadata {
        merged_metadata.insert(key.clone(), value.clone());
    }
    merged_metadata
}

fn get_template_names(
    templates: &Option<Vec<String>>,
    profile: Option<String>,
    manifest: &Manifest,
) -> Result<Vec<String>> {
    if let Some(templates) = templates {
        return Ok(templates.clone());
    }

    if let Some(profile) = profile {
        if let Some(available_profiles) = &manifest.profiles {
            if let Some(profile_pos) = available_profiles.iter().position(|p| p.name == profile) {
                return Ok(available_profiles[profile_pos]
                    .templates
                    .iter()
                    .map(|t| t.clone())
                    .collect());
            } else {
                return Err(eyre!("Profile '{}' could not be found.", profile));
            }
        } else {
            return Err(eyre!("No profiles are defined in the manifest.toml file."));
        }
    }

    Ok(manifest.templates.iter().map(|t| t.name.clone()).collect())
}

fn get_template_mappings_from_names(
    templates: &Vec<String>,
    manifest: &Manifest,
) -> Result<Vec<TemplateMapping>> {
    let templates = templates
        .iter()
        .map(|t| manifest.templates.iter().find(|mapping| mapping.name == *t))
        .filter_map(|t| t.cloned())
        .collect::<Vec<_>>();

    Ok(templates)
}

fn create_build_directory(project_path: &Path) -> Result<std::path::PathBuf> {
    let current_time = std::time::SystemTime::now();
    let current_time: DateTime<Utc> = current_time.into();
    let current_time = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
    let build_directory_path = project_path.join(current_time);

    dir::create_all(&build_directory_path, false)?;

    Ok(build_directory_path)
}

fn copy_markdown_directory(
    markdown_dir: &Path,
    output_dir: &Path,
    resources: &Option<Vec<PathBuf>>,
) -> Result<()> {
    debug!(
        "Copying markdown directory '{}' to '{}'",
        markdown_dir.display(),
        output_dir.display()
    );

    if !output_dir.exists() {
        dir::create_all(output_dir, false)?;
    }

    dir::copy(
        markdown_dir,
        output_dir,
        &dir::CopyOptions::new().overwrite(true).content_only(true),
    )?;

    for resource in resources.clone().unwrap_or(vec![]) {
        let resource = output_dir.join(resource.clone());

        if resource.is_dir() {
            dir::remove(resource)?;
        } else {
            file::remove(resource)?;
        }
    }

    Ok(())
}

fn convert_template(
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    project_path: &Path,
    conversion_input_dir: &Path,
    output_dir: &Path,
    metadata_fields: &Table,
    metadata_settings: &Option<MetadataSettings>,
    custom_processors: &Processors,
) -> Result<()> {
    debug!("Starting template conversion for '{}'.", template.name);
    debug!("  Template type: '{}'.", template.template_type);

    let converter = conversion_decider::get_converter(&template.template_type)?;

    let metadata_settings = metadata_settings
        .clone()
        .unwrap_or(MetadataSettings::default());

    debug!("Running converter...");

    let result_file_path = converter(
        project_path,
        compiled_directory_path,
        conversion_input_dir,
        template,
        metadata_fields,
        &metadata_settings,
        custom_processors,
    )?;

    debug!("Converter finished.");
    debug!("  Result file path: {}", result_file_path.display());

    debug!("Copying result file to output directory...");

    dir::create_all(project_path.join(output_dir), false)?;
    file::copy(
        &result_file_path,
        project_path
            .join(output_dir)
            .join(result_file_path.file_name().unwrap_or_default()),
        &file::CopyOptions::new().overwrite(true),
    )?;

    debug!("Copying finished.");

    info!("Converted template: {}", template.name);
    Ok(())
}
