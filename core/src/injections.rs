use color_eyre::eyre::{Result, eyre};
use log::debug;
use std::path::{Path, PathBuf};

use crate::{
    manifest_model::{Injection, Template},
    project_management::load_and_convert_manifest,
};

pub(crate) struct RenderingInjections {
    pub header_injections: Vec<PathBuf>,
    pub body_injections: Vec<PathBuf>,
    pub footer_injections: Vec<PathBuf>,
}

impl RenderingInjections {
    pub fn new() -> RenderingInjections {
        RenderingInjections {
            header_injections: vec![],
            body_injections: vec![],
            footer_injections: vec![],
        }
    }
}

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

pub(crate) fn retrieve_injections(
    template: &Template,
    compiled_directory_path: &Path,
    injections: &Vec<Injection>,
) -> Result<RenderingInjections> {
    let header_injections = retrieve_injections_from_manifest(
        &injections,
        compiled_directory_path,
        template.header_injections.clone().unwrap_or(vec![]),
        &template.name,
    )?;
    let body_injections = retrieve_injections_from_manifest(
        &injections,
        compiled_directory_path,
        template.body_injections.clone().unwrap_or(vec![]),
        &template.name,
    )?;
    let footer_injections = retrieve_injections_from_manifest(
        &injections,
        compiled_directory_path,
        template.footer_injections.clone().unwrap_or(vec![]),
        &template.name,
    )?;

    Ok(RenderingInjections {
        header_injections,
        body_injections,
        footer_injections,
    })
}

fn retrieve_injections_from_manifest(
    injections: &Vec<Injection>,
    compiled_directory_path: &Path,
    template_injections: Vec<String>,
    template_name: &String,
) -> Result<Vec<PathBuf>> {
    debug!(
        "Retrieving injections '{}' from Manifest.",
        template_injections.join(",")
    );

    let injections = template_injections
        .iter()
        .map(|n| {
            injections
                .iter()
                .find(|i| i.name == *n)
                .ok_or(eyre!(
                    "Injection '{}' referenced in template '{}' was not found in manifest.",
                    n,
                    template_name
                ))
                .and_then(|i| Ok(i.files.clone()))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .flatten()
        .map(|f| {
            let template_injection_path = compiled_directory_path.join(f);
            debug!(
                "Found injection file '{}'.",
                template_injection_path.display()
            );
            if !template_injection_path.exists() {
                return Err(eyre!(
                    "Injection file '{}' is not a file or directory.",
                    f.display(),
                ));
            }

            Ok(template_injection_path)
        })
        .collect::<Result<Vec<_>>>()?;

    debug!("Retrieved {} injections.", injections.len());

    Ok(injections)
}
