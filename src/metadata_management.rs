use color_eyre::eyre::{Result, eyre};
use log::info;
use toml::{Table, Value};

use crate::project_management::load_and_convert_manifest;

pub(crate) fn set_metadata(project: Option<String>, key: String, value: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(shared_metadata) = &mut manifest.shared_metadata {
        shared_metadata.insert(key, Value::String(value));
    } else {
        manifest.shared_metadata = Some(Table::new());
        manifest
            .shared_metadata
            .as_mut()
            .unwrap()
            .insert(key, Value::String(value));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub(crate) fn remove_metadata(project: Option<String>, key: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(shared_metadata) = &mut manifest.shared_metadata {
        if !shared_metadata.contains_key(&key) {
            return Err(eyre!("Metadata field '{}' not found.", key));
        }

        shared_metadata.remove(&key);
    } else {
        return Err(eyre!("Metadata field '{}' not found.", key));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub(crate) fn list_metadata(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let metadata_fields = manifest.shared_metadata.unwrap_or_default();

    if metadata_fields.is_empty() {
        info!("No shared metadata fields found.");
        return Ok(());
    }
    for (key, value) in metadata_fields {
        info!("{}: {}", key, value);
    }

    Ok(())
}
