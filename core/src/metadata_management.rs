use crate::{manifest_model::MetadataField, project_handle::ProjectHandle};
use color_eyre::eyre::{Result, eyre};
use log::debug;
use toml::{Table, Value};

/// Sets the shared metadata fields for a TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
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
/// use tiefdownlib::{project_handle::ProjectHandle, metadata_management::set_metadata};
/// use std::path::PathBuf;
///
/// set_metadata(
///     &mut ProjectHandle::open(Some(PathBuf::from("my_project"))).unwrap(),
///     "author".to_string(),
///     "Jane Doe".to_string(),
/// ).unwrap();
/// ```
pub fn set_metadata(project_handle: &mut ProjectHandle, key: String, value: String) -> Result<()> {
    debug!("metadata.set: key='{}'", key);
    project_handle
        .manifest
        .shared_metadata
        .get_or_insert_with(&mut || Table::new())
        .insert(key, Value::String(value));

    project_handle.mark_dirty();

    Ok(())
}

/// Removes a metadata field from the shared metadata of a TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `key` - The key of the metadata field to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, metadata_management::remove_metadata};
/// use std::path::PathBuf;
///
/// remove_metadata(&mut ProjectHandle::open(Some(PathBuf::from("my_project"))).unwrap(), "author".to_string()).unwrap();
/// ```
pub fn remove_metadata(project_handle: &mut ProjectHandle, key: String) -> Result<()> {
    let shared_metadata = project_handle
        .manifest
        .shared_metadata
        .as_mut()
        .ok_or(eyre!("No shared metadata found."))?;

    if !shared_metadata.contains_key(&key) {
        return Err(eyre!("Metadata field '{}' not found.", key));
    }

    shared_metadata.remove(&key);

    project_handle.mark_dirty();

    Ok(())
}

/// Retrieves the shared metadata fields for a TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
///
/// # Returns
///
/// A Result containing either an error or a Vec of MetadataField.
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::{project_handle::ProjectHandle, metadata_management::get_metadata};
/// use std::path::PathBuf;
///
/// let fields = get_metadata(&ProjectHandle::open(Some(PathBuf::from("my_project"))).unwrap()).unwrap();
/// for field in fields {
///     println!("{} = {}", field.key, field.value);
/// }
/// ```
pub fn get_metadata(project_handle: &ProjectHandle) -> Result<Vec<MetadataField>> {
    let metadata_fields = project_handle
        .manifest
        .shared_metadata
        .clone()
        .unwrap_or_default();
    debug!("metadata.get: {} entries", metadata_fields.len());

    Ok(metadata_fields
        .iter()
        .map(|e| MetadataField {
            key: e.0.clone(),
            value: e.1.clone().to_string(),
        })
        .collect())
}
