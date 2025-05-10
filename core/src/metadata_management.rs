use crate::{manifest_model::MetadataField, project_management::load_and_convert_manifest};
use color_eyre::eyre::{Result, eyre};
use toml::{Table, Value};

pub fn set_metadata(project: Option<String>, key: String, value: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    manifest
        .shared_metadata
        .as_mut()
        .unwrap_or(&mut Table::new())
        .insert(key, Value::String(value));

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn remove_metadata(project: Option<String>, key: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let shared_metadata = manifest
        .shared_metadata
        .as_mut()
        .ok_or(eyre!("No shared metadata found."))?;

    if !shared_metadata.contains_key(&key) {
        return Err(eyre!("Metadata field '{}' not found.", key));
    }

    shared_metadata.remove(&key);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn get_metadata(project: &Option<String>) -> Result<Vec<MetadataField>> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let metadata_fields = manifest.shared_metadata.unwrap_or_default();

    Ok(metadata_fields
        .iter()
        .map(|e| MetadataField {
            key: e.0.clone(),
            value: e.1.clone().to_string(),
        })
        .collect())
}
