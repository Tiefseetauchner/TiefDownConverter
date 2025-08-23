use color_eyre::eyre::Result;
use log::info;
use tiefdownlib::{markdown_project_management, metadata_management, project_management};

pub(crate) fn list_preprocessors(project: Option<String>) -> Result<()> {
    let preprocessors = project_management::get_preprocessors(&project)?;

    if preprocessors.is_empty() {
        info!("No preprocessors found.");
        return Ok(());
    }

    for preprocessor in preprocessors {
        info!(
            "{}: {} {}",
            preprocessor.name,
            preprocessor.cli.unwrap_or("pandoc".to_string()),
            preprocessor.cli_args.join(" ")
        );
    }

    Ok(())
}
pub(crate) fn list_processors(project: Option<String>) -> Result<()> {
    let processors = project_management::get_processors(&project)?;

    if processors.is_empty() {
        info!("No processors found.");
        return Ok(());
    }

    for processor in processors {
        info!("{}: {}", processor.name, processor.processor_args.join(" "));
    }

    Ok(())
}
pub(crate) fn list_profiles(project: Option<String>) -> Result<()> {
    let profiles = project_management::get_profiles(&project)?;

    if profiles.is_empty() {
        info!("No profiles found.");
        return Ok(());
    }

    for profile in profiles {
        info!("{}", profile.name);

        for template in profile.templates {
            info!("  {}", template);
        }
    }

    Ok(())
}
pub(crate) fn list_shared_metadata(project: Option<String>) -> Result<()> {
    let metadata = metadata_management::get_metadata(&project)?;

    if metadata.is_empty() {
        info!("No shared metadata fields found.");
        return Ok(());
    }

    for metadata_field in metadata {
        info!("{}={}", metadata_field.key, metadata_field.value);
    }

    Ok(())
}
pub(crate) fn list_markdown_project_metadata(
    project: Option<String>,
    markdown_project_name: String,
) -> Result<()> {
    let metadata = markdown_project_management::get_metadata(&project, &markdown_project_name)?;

    if metadata.is_empty() {
        info!("No metadata found for project {}.", markdown_project_name);
        return Ok(());
    }

    for metadata_field in metadata {
        info!("{}={}", metadata_field.key, metadata_field.value);
    }

    Ok(())
}
pub(crate) fn list_resources(project: Option<String>, markdown_project_name: String) -> Result<()> {
    let resources = markdown_project_management::get_resources(&project, &markdown_project_name)?;

    if resources.is_empty() {
        info!("No resources found for project {}.", markdown_project_name);
        return Ok(());
    }

    for resource in resources {
        info!("{}", resource.display());
    }

    Ok(())
}
pub(crate) fn list_markdown_projects(project: Option<String>) -> Result<()> {
    let markdown_projects = markdown_project_management::get_markdown_projects(&project)?;

    if markdown_projects.is_empty() {
        info!("No markdown projects found.");
        return Ok(());
    }

    for project in markdown_projects {
        info!("Name: {}", project.name);
        info!("  Path: {}", project.path.display());
        info!("  Output: {}", project.output.display());
        info!(
            "  Default Profile: {}",
            project.default_profile.unwrap_or_default()
        );
        info!("  Metadata Fields:");
        for (key, value) in project.metadata_fields.unwrap_or_default() {
            info!("    {}: {}", key, value);
        }
        info!("  Resources:");
        for resource in project.resources.unwrap_or_default() {
            info!("    {}", resource.display());
        }
    }

    Ok(())
}
pub(crate) fn list_templates(project: Option<String>) -> Result<()> {
    let templates = project_management::get_templates(&project)?;

    if templates.is_empty() {
        info!("No templates found.");
        return Ok(());
    }

    for template in templates {
        info!("{}:", template.name);
        info!("  Template type: {}", &template.template_type);
        if let Some(file) = &template.template_file {
            info!("  Template file: {}", file.display());
        }
        if let Some(output) = &template.output {
            info!("  Output file: {}", output.display());
        }
        if let Some(filters) = &template.filters {
            info!("  Filters: {}", filters.join(", "));
        }
    }

    Ok(())
}
