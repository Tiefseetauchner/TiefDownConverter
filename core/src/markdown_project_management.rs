use color_eyre::eyre::{Result, eyre};
use log::info;
use std::path::PathBuf;
use toml::{Table, Value};

use crate::{manifest_model::MarkdownProject, project_management::load_and_convert_manifest};

pub fn add_markdown_project(
    project: Option<String>,
    name: String,
    path: PathBuf,
    output: PathBuf,
    default_profile: Option<String>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    if markdown_projects.iter().any(|p| p.name == name) {
        return Err(eyre!(
            "Markdown project with name '{}' already exists.",
            name
        ));
    }

    markdown_projects.push(MarkdownProject {
        name,
        path,
        output,
        metadata_fields: None,
        default_profile,
        resources: None,
    });

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn remove_markdown_project(project: Option<String>, name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    if let Some(pos) = markdown_projects.iter().position(|p| p.name == name) {
        markdown_projects.remove(pos);
    } else {
        return Err(eyre!(
            "Markdown project with name '{}' does not exist.",
            name
        ));
    }

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn update_markdown_project(
    project: Option<String>,
    name: String,
    path: Option<PathBuf>,
    output: Option<PathBuf>,
    default_profile: Option<String>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    for project in &mut markdown_projects {
        if project.name == name {
            if let Some(path) = path {
                project.path = path;
            }
            if let Some(output) = output {
                project.output = output;
            }
            if let Some(default_profile) = default_profile {
                project.default_profile = Some(default_profile);
            }
            break;
        }
    }

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn set_metadata(
    project: Option<String>,
    name: String,
    key: String,
    value: String,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    if let Some(project) = markdown_projects.iter_mut().find(|p| p.name == name) {
        if let Some(metadata_fields) = &mut project.metadata_fields {
            metadata_fields.insert(key, Value::String(value));
        } else {
            project.metadata_fields = Some(Table::new());
            project
                .metadata_fields
                .as_mut()
                .unwrap()
                .insert(key, Value::String(value));
        }
    } else {
        return Err(eyre!(
            "Markdown project with name '{}' does not exist.",
            name
        ));
    }

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn remove_metadata(project: Option<String>, name: String, key: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    if let Some(project) = markdown_projects.iter_mut().find(|p| p.name == name) {
        if let Some(metadata_fields) = &mut project.metadata_fields {
            if !metadata_fields.contains_key(&key) {
                return Err(eyre!("Metadata field '{}' not found.", key));
            }

            metadata_fields.remove(&key);
        }
    } else {
        return Err(eyre!(
            "Markdown project with name '{}' does not exist.",
            name
        ));
    }

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn list_metadata(project: Option<String>, name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);
    if let Some(project) = markdown_projects.iter().find(|p| p.name == name) {
        if let Some(metadata_fields) = &project.metadata_fields {
            for (key, value) in metadata_fields {
                info!("{}: {}", key, value);
            }
        } else {
            info!("No metadata fields found.");
        }
    } else {
        return Err(eyre!(
            "Markdown project with name '{}' does not exist.",
            name
        ));
    }
    Ok(())
}

pub fn list_markdown_projects(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

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

pub fn add_resources(project: Option<String>, name: String, resources: Vec<PathBuf>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    if let Some(project) = markdown_projects.iter_mut().find(|p| p.name == name) {
        if let Some(project_resources) = &mut project.resources {
            project_resources.extend(resources);
        } else {
            project.resources = Some(resources);
        }
    } else {
        return Err(eyre!(
            "Markdown project with name '{}' does not exist.",
            name
        ));
    }

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn remove_resource(project: Option<String>, name: String, resource: PathBuf) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    if let Some(project) = markdown_projects.iter_mut().find(|p| p.name == name) {
        if let Some(pos) = project
            .resources
            .clone()
            .unwrap_or(vec![])
            .iter()
            .position(|r| r == &resource)
        {
            project.resources.as_mut().unwrap().remove(pos);
        } else {
            return Err(eyre!("Resource '{}' not found.", resource.display()));
        }
    } else {
        return Err(eyre!(
            "Markdown project with name '{}' does not exist.",
            name
        ));
    }

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn list_resources(project: Option<String>, name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);
    if let Some(project) = markdown_projects.iter().find(|p| p.name == name) {
        if let Some(resources) = &project.resources {
            for resource in resources {
                info!("{}", resource.display());
            }
        }
    }

    Ok(())
}
