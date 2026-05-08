use crate::{
    manifest_model::{MarkdownProject, MetadataField},
    project_handle::ProjectHandle,
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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::add_markdown_project};
/// use std::path::PathBuf;
///
/// add_markdown_project(
///     &mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap(),
///     "chapter1".to_string(),
///     PathBuf::from("Markdown/Chapter1"),
///     PathBuf::from("output/chapter1"),
///     None,
/// ).unwrap();
/// ```
pub fn add_markdown_project(
    project_handle: &mut ProjectHandle,
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

    let mut markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

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

    project_handle.manifest.markdown_projects = Some(markdown_projects);

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::remove_markdown_project};
/// use std::path::PathBuf;
///
/// remove_markdown_project(&mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap(), "chapter1".to_string()).unwrap();
/// ```
pub fn remove_markdown_project(project_handle: &mut ProjectHandle, name: String) -> Result<()> {
    debug!("Removing markdown project '{}'", name);

    let mut markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

    if let Some(pos) = markdown_projects.iter().position(|p| p.name == name) {
        markdown_projects.remove(pos);
    } else {
        return Err(eyre!(
            "Markdown project with name '{}' does not exist.",
            name
        ));
    }

    project_handle.manifest.markdown_projects = Some(markdown_projects);

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::update_markdown_project};
/// use std::path::PathBuf;
///
/// update_markdown_project(
///     &mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap(),
///     "chapter1".to_string(),
///     Some(PathBuf::from("Markdown/NewChapter1")),
///     None,
///     None,
/// ).unwrap();
/// ```
pub fn update_markdown_project(
    project_handle: &mut ProjectHandle,
    name: String,
    path: Option<PathBuf>,
    output: Option<PathBuf>,
    default_profile: Option<String>,
) -> Result<()> {
    debug!(
        "Updating markdown project '{}' (path={:?}, output={:?}, default_profile={:?})",
        name, path, output, default_profile
    );

    let mut markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

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

    project_handle.manifest.markdown_projects = Some(markdown_projects);

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::set_metadata};
/// use std::path::PathBuf;
///
/// set_metadata(
///     &mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap(),
///     "chapter1".to_string(),
///     "title".to_string(),
///     "Chapter One".to_string(),
/// ).unwrap();
/// ```
pub fn set_metadata(
    project_handle: &mut ProjectHandle,
    name: String,
    key: String,
    value: String,
) -> Result<()> {
    debug!("markdown.set_metadata: project='{}' key='{}'", name, key);

    let mut markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

    let project = markdown_projects
        .iter_mut()
        .find(|p| p.name == name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    project
        .metadata_fields
        .get_or_insert_with(Table::new)
        .insert(key, Value::String(value));

    project_handle.manifest.markdown_projects = Some(markdown_projects);

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::remove_metadata};
/// use std::path::PathBuf;
///
/// remove_metadata(
///     &mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap(),
///     "chapter1".to_string(),
///     "title".to_string(),
/// ).unwrap();
/// ```
pub fn remove_metadata(
    project_handle: &mut ProjectHandle,
    name: String,
    key: String,
) -> Result<()> {
    debug!("markdown.remove_metadata: project='{}' key='{}'", name, key);

    let mut markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

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

    project_handle.manifest.markdown_projects = Some(markdown_projects);

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::get_metadata};
/// use std::path::PathBuf;
///
/// let name = "chapter1".to_string();
/// let fields = get_metadata(&ProjectHandle::open(Some(PathBuf::from("."))).unwrap(), &name).unwrap();
/// for field in fields {
///     println!("{} = {}", field.key, field.value);
/// }
/// ```
pub fn get_metadata(project_handle: &ProjectHandle, name: &String) -> Result<Vec<MetadataField>> {
    let markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::get_markdown_projects};
/// use std::path::PathBuf;
///
/// let projects = get_markdown_projects(&mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap()).unwrap();
/// for project in projects {
///     println!("{}: {}", project.name, project.path.display());
/// }
/// ```
pub fn get_markdown_projects(project_handle: &ProjectHandle) -> Result<Vec<MarkdownProject>> {
    let markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);
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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::add_resources};
/// use std::path::PathBuf;
///
/// add_resources(
///     &mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap(),
///     "chapter1".to_string(),
///     vec![PathBuf::from("images/")],
/// ).unwrap();
/// ```
pub fn add_resources(
    project_handle: &mut ProjectHandle,
    name: String,
    resources: Vec<PathBuf>,
) -> Result<()> {
    debug!(
        "markdown.add_resources: project='{}' count={}",
        name,
        resources.len()
    );

    let mut markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

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

    project_handle.manifest.markdown_projects = Some(markdown_projects);

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::remove_resource};
/// use std::path::PathBuf;
///
/// remove_resource(
///     &mut ProjectHandle::open(Some(PathBuf::from("."))).unwrap(),
///     "chapter1".to_string(),
///     PathBuf::from("images/"),
/// ).unwrap();
/// ```
pub fn remove_resource(
    project_handle: &mut ProjectHandle,
    name: String,
    resource: PathBuf,
) -> Result<()> {
    debug!(
        "markdown.remove_resource: project='{}' resource='{}'",
        name,
        resource.display()
    );

    let mut markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

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

    project_handle.manifest.markdown_projects = Some(markdown_projects);

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, markdown_project_management::get_resources};
/// use std::path::PathBuf;
///
/// let name = "chapter1".to_string();
/// let resources = get_resources(&ProjectHandle::open(Some(PathBuf::from("."))).unwrap(), &name).unwrap();
/// for resource in resources {
///     println!("{}", resource.display());
/// }
/// ```
pub fn get_resources(project_handle: &ProjectHandle, name: &String) -> Result<Vec<PathBuf>> {
    let markdown_projects = project_handle
        .manifest
        .markdown_projects
        .clone()
        .unwrap_or(vec![]);

    let project = markdown_projects
        .iter()
        .find(|p| p.name == *name)
        .ok_or_else(|| eyre!("Markdown project with name '{}' does not exist.", name))?;

    let res = project.resources.clone().unwrap_or(vec![]);
    debug!("markdown.get_resources: {} entries", res.len());
    Ok(res)
}
