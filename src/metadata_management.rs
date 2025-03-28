use color_eyre::eyre::{Result, eyre};
use toml::Value;

use crate::project_management::load_and_convert_manifest;

pub(crate) fn set_metadata(project: Option<String>, key: String, value: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    manifest
        .metadata_fields
        .insert(key.to_string(), Value::String(value.to_string()));

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub(crate) fn remove_metadata(project: Option<String>, key: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if !manifest.metadata_fields.contains_key(&key) {
        return Err(eyre!("Metadata field '{}' not found.", key));
    }

    manifest.metadata_fields.remove(&key);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub(crate) fn list_metadata(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let metadata_fields = manifest.metadata_fields;

    for (key, value) in metadata_fields {
        println!("{}: {}", key, value);
    }

    Ok(())
}
