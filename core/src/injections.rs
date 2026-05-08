use color_eyre::eyre::{Result, eyre};
use log::debug;
use std::path::{Path, PathBuf};

use crate::{
    manifest_model::{Injection, Template},
    project_handle::ProjectHandle,
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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::injections::add_injection;
/// use std::path::PathBuf;
///
/// add_injection(
///     Some(PathBuf::from("my_project")),
///     "header".to_string(),
///     vec![PathBuf::from("header.tex")],
/// ).unwrap();
/// ```
pub fn add_injection(
    project_handle: &mut ProjectHandle,
    name: String,
    files: Vec<PathBuf>,
) -> Result<()> {
    let injection = Injection {
        name: name.clone(),
        files: files.clone(),
    };

    if let Some(injections) = &mut project_handle.manifest.injections {
        if injections.iter().any(|i| i.name == name) {
            return Err(eyre!("Injection '{}' already exists.", name));
        }

        injections.push(injection);
    } else {
        project_handle.manifest.injections = Some(vec![injection]);
    }

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::injections::remove_injection;
/// use std::path::PathBuf;
///
/// remove_injection(Some(PathBuf::from("my_project")), "header".to_string()).unwrap();
/// ```
pub fn remove_injection(project_handle: &mut ProjectHandle, name: String) -> Result<()> {
    if let Some(injections) = project_handle.manifest.injections.as_mut() {
        if let Some(pos) = injections.iter().position(|i| i.name == name) {
            injections.remove(pos);
        } else {
            return Err(eyre!("Injection '{}' was not found in the manifest.", name));
        }
    } else {
        return Err(eyre!("No injections defined in the manifest."));
    }

    project_handle.mark_dirty();

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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::injections::get_injections;
/// use std::path::PathBuf;
///
/// let injections = get_injections(Some(PathBuf::from("my_project"))).unwrap();
/// for injection in injections {
///     println!("{}: {:?}", injection.name, injection.files);
/// }
/// ```
pub fn get_injections(project_handle: &ProjectHandle) -> Result<Vec<Injection>> {
    if let Some(injections) = &project_handle.manifest.injections {
        Ok(injections.to_vec())
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
///
/// # Examples
///
/// ```no_run
/// use tiefdownlib::injections::add_files_to_injection;
/// use std::path::PathBuf;
///
/// add_files_to_injection(
///     Some(PathBuf::from("my_project")),
///     "header".to_string(),
///     vec![PathBuf::from("extra_header.tex")],
/// ).unwrap();
/// ```
pub fn add_files_to_injection(
    project_handle: &mut ProjectHandle,
    name: String,
    files: Vec<PathBuf>,
) -> Result<()> {
    let mut new_files = files.clone();

    project_handle
        .manifest
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

    project_handle.mark_dirty();

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
