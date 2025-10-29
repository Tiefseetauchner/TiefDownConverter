use std::path::PathBuf;

use crate::{manifest_model::Injection, project_management::load_and_convert_manifest};
use color_eyre::eyre::{Result, eyre};
use log::debug;

/// Adds an injection to the project manifest.
///
/// # Arguments
///
/// * `manifest` - The manifest object of the project.
/// * `name` - The name of the injection.
/// * `files` - The files to be injected by the injection.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_injection(project: Option<PathBuf>, name: String, files: Vec<PathBuf>) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let injection = Injection {
        name: name.clone(),
        files: files.clone(),
    };

    if let Some(injections) = &mut manifest.injections {
        if injections.iter().any(|i| i.name == name) {
            return Err(eyre!("Injection '{}' already exists.", name));
        }

        injections.push(injection);
    } else {
        manifest.injections = Some(vec![injection]);
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;
    debug!("Injection '{}' added.", name);

    Ok(())
}

/// Removes an injection from the project manifest.
///
/// # Arguments
///
/// * `manifest` - The manifest object of the project.
/// * `name` - The name of the injection.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_injection(project: Option<PathBuf>, name: String) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(injections) = manifest.injections.as_mut() {
        if let Some(pos) = injections.iter().position(|i| i.name == name) {
            injections.remove(pos);
        } else {
            return Err(eyre!("Injection '{}' was not found in the manifest.", name));
        }
    } else {
        return Err(eyre!("No injections defined in the manifest."));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;
    debug!("Removed injection '{}'", name);

    Ok(())
}

/// Retrieves the list of injections from the project's manifest.
///
/// # Arguments
///
/// * `manifest` - The manifest object of the project.
///
/// # Returns
///
/// A Result containing either an error or a vector of Injection objects.
pub fn get_injections(project: Option<PathBuf>) -> Result<Vec<Injection>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(injections) = manifest.injections {
        Ok(injections)
    } else {
        Ok(vec![])
    }
}

/// Adds files to an injection.
///
/// # Argumentsc
///
/// * `manifest` - The manifest object of the project.
/// * `name` - The name of the injection.
/// * `files` - The files to be added to the injection.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_files_to_injection(
    project: Option<PathBuf>,
    name: String,
    files: Vec<PathBuf>,
) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let mut new_files = files.clone();

    manifest
        .injections
        .as_mut()
        .ok_or(eyre!(
            "There is no valid injection defined in the manifest."
        ))?
        .iter_mut()
        .find(|i| i.name == name)
        .ok_or(eyre!("Injection with name '{}' does not exist.", name))?
        .files
        .append(&mut new_files);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;
    debug!("Added {} files to the injection '{}'.", files.len(), name);

    Ok(())
}
