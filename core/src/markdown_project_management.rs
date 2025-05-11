use crate::{
    manifest_model::{MarkdownProject, MetadataField},
    project_management::load_and_convert_manifest,
};
use color_eyre::eyre::{Result, eyre};
use std::path::PathBuf;
use toml::{Table, Value};

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

    let project = markdown_projects
        .iter_mut()
        .find(|p| p.name == name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    if let Some(path) = path {
        project.path = path;
    }

    if let Some(output) = output {
        project.output = output;
    }

    if let Some(default_profile) = default_profile {
        project.default_profile = Some(default_profile);
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

    let project = markdown_projects
        .iter_mut()
        .find(|p| p.name == name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    project
        .metadata_fields
        .get_or_insert_with(Table::new)
        .insert(key, Value::String(value));

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

    let project = markdown_projects
        .iter_mut()
        .find(|p| p.name == name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    if let Some(metadata_fields) = &mut project.metadata_fields {
        let removed = metadata_fields.remove(&key);

        if removed.is_none() {
            return Err(eyre!(
                "Metadata field '{}' does not exist in project '{}'.",
                key,
                name
            ));
        }
    }

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn get_metadata(project: &Option<String>, name: &String) -> Result<Vec<MetadataField>> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;
    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    let project = markdown_projects
        .iter()
        .find(|p| p.name == *name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    Ok(project
        .metadata_fields
        .clone()
        .map(|m| {
            m.iter()
                .map(|e| MetadataField {
                    key: e.0.clone(),
                    value: e.1.clone().to_string(),
                })
                .collect()
        })
        .unwrap_or(vec![]))
}

pub fn get_markdown_projects(project: &Option<String>) -> Result<Vec<MarkdownProject>> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    Ok(markdown_projects)
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

    let project = markdown_projects
        .iter_mut()
        .find(|p| p.name == name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

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

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn get_resources(project: &Option<String>, name: &String) -> Result<Vec<PathBuf>> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    let project = markdown_projects
        .iter()
        .find(|p| p.name == *name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    Ok(project.resources.clone().unwrap_or(vec![]))
}
