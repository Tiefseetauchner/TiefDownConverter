use crate::{
    manifest_model::{MarkdownProject, MetadataField},
    project_management::load_and_convert_manifest,
};
use color_eyre::eyre::{Result, eyre};
use log::debug;
use std::path::PathBuf;
use toml::{Table, Value};

/// Adds a new markdown project to the TiefDown project.
///
/// # Arguments
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project.
/// * `path` - The path to the markdown directory.
/// * `output` - The path to the output directory.
/// * `default_profile` - The name of the default profile to use for conversion.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_markdown_project(
    project: Option<PathBuf>,
    name: String,
    path: PathBuf,
    output: PathBuf,
    default_profile: Option<String>,
) -> Result<()> {
    debug!(
        "Adding markdown project '{}' (path='{}', output='{}')",
        name,
        path.display(),
        output.display()
    );
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    if markdown_projects.iter().any(|p| p.name == name) {
        return Err(eyre!(
            "Markdown project with name '{}' already exists.",
            name
        ));
    }

    markdown_projects.push(MarkdownProject {
        name: name.clone(),
        path,
        output,
        metadata_fields: None,
        default_profile,
        resources: None,
    });

    manifest.markdown_projects = Some(markdown_projects);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;
    debug!("Markdown project '{}' added.", name);

    Ok(())
}

/// Removes a markdown project from the TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_markdown_project(project: Option<PathBuf>, name: String) -> Result<()> {
    debug!("Removing markdown project '{}'", name);
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

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
    debug!("Markdown project '{}' removed.", name);

    Ok(())
}

/// Updates a markdown project in the TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project to update.
/// * `path` - The new path to the markdown directory.
/// * `output` - The new path to the output directory.
/// * `default_profile` - The new name of the default profile to use for conversion.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn update_markdown_project(
    project: Option<PathBuf>,
    name: String,
    path: Option<PathBuf>,
    output: Option<PathBuf>,
    default_profile: Option<String>,
) -> Result<()> {
    debug!(
        "Updating markdown project '{}' (path={:?}, output={:?}, default_profile={:?})",
        name, path, output, default_profile
    );
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

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
    debug!("Markdown project '{}' updated.", name);

    Ok(())
}

/// Sets the metadata fields for a markdown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project.
/// * `key` - The key of the metadata field to set.
/// * `value` - The value to set for the metadata field.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn set_metadata(
    project: Option<PathBuf>,
    name: String,
    key: String,
    value: String,
) -> Result<()> {
    debug!("markdown.set_metadata: project='{}' key='{}'", name, key);
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

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
    debug!("markdown.set_metadata: updated manifest for '{}'", name);

    Ok(())
}

/// Removes the metadata fields for a markdown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project to remove.
/// * `key` - The key of the metadata field to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_metadata(project: Option<PathBuf>, name: String, key: String) -> Result<()> {
    debug!("markdown.remove_metadata: project='{}' key='{}'", name, key);
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

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
    debug!("markdown.remove_metadata: updated manifest for '{}'", name);

    Ok(())
}

/// Gets the metadata fields for a markdown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project to remove.
///
/// # Returns
///
/// A Result containing either an error or a Vec of MetadataField.
pub fn get_metadata(project: Option<PathBuf>, name: &String) -> Result<Vec<MetadataField>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;
    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    let project = markdown_projects
        .iter()
        .find(|p| p.name == *name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    let result = project
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
        .unwrap_or(vec![]);
    debug!("markdown.get_metadata: {} entries", result.len());
    Ok(result)
}

/// Gets the markdown projects.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
///
/// # Returns
///
/// A Result containing either an error or a Vec of MarkdownProject.

pub fn get_markdown_projects(project: Option<PathBuf>) -> Result<Vec<MarkdownProject>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);
    debug!(
        "markdown.get_markdown_projects: {} projects",
        markdown_projects.len()
    );
    Ok(markdown_projects)
}

/// Adds a resource to a markdown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project to remove.
/// * `resources` - The resources to add.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_resources(
    project: Option<PathBuf>,
    name: String,
    resources: Vec<PathBuf>,
) -> Result<()> {
    debug!(
        "markdown.add_resources: project='{}' count={}",
        name,
        resources.len()
    );
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

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
    debug!("markdown.add_resources: updated manifest for '{}'", name);

    Ok(())
}

/// Removes a resource from a markdown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project to remove.
/// * `resource` - The resource to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_resource(project: Option<PathBuf>, name: String, resource: PathBuf) -> Result<()> {
    debug!(
        "markdown.remove_resource: project='{}' resource='{}'",
        name,
        resource.display()
    );
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

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
    debug!("markdown.remove_resource: updated manifest for '{}'", name);

    Ok(())
}

/// Gets the resources of a markdown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the markdown project to remove.
///
/// # Returns
///
/// A Result containing either an error or a Vec of PathBuf.
pub fn get_resources(project: Option<PathBuf>, name: &String) -> Result<Vec<PathBuf>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![]);

    let project = markdown_projects
        .iter()
        .find(|p| p.name == *name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    let res = project.resources.clone().unwrap_or(vec![]);
    debug!("markdown.get_resources: {} entries", res.len());
    Ok(res)
}
