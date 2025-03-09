use std::{fs, path::PathBuf};

use color_eyre::eyre::{eyre, Result};
use toml::Table;

use crate::{
    consts::CURRENT_MANIFEST_VERSION,
    manifest_model::{upgrade_manifest, Manifest, TemplateMapping, TemplateType},
    template_management::{self, get_template_path, get_template_type_from_path},
};

pub fn init(
    project: Option<String>,
    template_names: Option<Vec<String>>,
    no_templates: bool,
    force: bool,
    markdown_dir: Option<String>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);

    if project_path.exists() && force {
        if project == "." {
            return Err(eyre!(
                "Cannot force initialization in the current directory."
            ));
        }
        std::fs::remove_dir_all(project_path)?;
    }

    if !project_path.exists() {
        std::fs::create_dir(project_path)?;
    }

    let manifest_path = project_path.join("manifest.toml");
    if manifest_path.exists() {
        return Err(eyre!(
            "Manifest file already exists. Please remove it before initializing a new project or use the --force flag."
        ));
    }

    let mut templates: Vec<TemplateMapping> = Vec::new();

    if !no_templates {
        templates.extend(
            template_names
                .unwrap_or(vec!["template.tex".to_string()])
                .iter()
                .map(get_template_mapping_for_preset)
                .collect::<Result<Vec<_>>>()?,
        );
    }

    let markdown_dir_path =
        project_path.join(markdown_dir.clone().unwrap_or("Markdown".to_string()));
    if !markdown_dir_path.exists() {
        std::fs::create_dir(&markdown_dir_path)?;
        std::fs::write(
            &markdown_dir_path.join("Chapter 1 - Introduction.md"),
            r#"# Test Document
This is a simple test document for you to edit or overwrite."#,
        )?;
    }

    let manifest: Manifest = Manifest {
        version: CURRENT_MANIFEST_VERSION,
        markdown_dir,
        templates: templates.clone(),
    };

    std::fs::write(manifest_path, toml::to_string(&manifest)?)?;

    create_templates(project_path, &templates)?;

    Ok(())
}

fn get_template_mapping_for_preset(template: &String) -> Result<TemplateMapping> {
    // NOTE: As this is just the preset templates, we set the minimal implementation.
    Ok(TemplateMapping {
        name: template.clone(),
        template_type: get_template_type_from_path(template)?,
        output: None,
        template_file: None,
        filters: None,
    })
}

pub(crate) fn add_template(
    project: Option<String>,
    template_name: String,
    template_type: Option<TemplateType>,
    template_file: Option<PathBuf>,
    output: Option<PathBuf>,
    filters: Option<Vec<String>>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let template = TemplateMapping {
        name: template_name.clone(),
        template_type: template_type.unwrap_or(get_template_type_from_path(get_template_path(
            template_file.clone(),
            &template_name,
        ))?),
        output,
        template_file,
        filters,
    };

    manifest.templates.extend([template.clone()]);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    create_templates(project_path, &vec![template.clone()])?;

    Ok(())
}

pub(crate) fn remove_template(project: Option<String>, template_name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(pos) = manifest
        .templates
        .iter()
        .position(|t| t.name == template_name)
    {
        let removed_template = manifest.templates.swap_remove(pos);

        let manifest_content = toml::to_string(&manifest)?;
        std::fs::write(&manifest_path, manifest_content)?;

        let template_dir = project_path.join("template");
        let template_path = template_dir.join(
            removed_template
                .template_file
                .as_ref()
                .unwrap_or(&PathBuf::from(&removed_template.name)),
        );

        if template_path.is_dir() {
            std::fs::remove_dir_all(&template_path)?;
        } else {
            fs::remove_file(template_path)?;
        }
    } else {
        return Err(eyre!(
            "Template {} could not be found in the project.",
            template_name
        ));
    }

    Ok(())
}

fn load_and_convert_manifest(manifest_path: &std::path::PathBuf) -> Result<Manifest> {
    if !manifest_path.exists() {
        return Err(eyre!(
            "Manifest file does not exist. Please initialize a project before editing it."
        ));
    }

    let manifest_content = fs::read_to_string(manifest_path)?;

    let mut manifest: Table = toml::from_str(&manifest_content)?;

    let current_manifest_version: u32 = manifest["version"].as_integer().unwrap_or(0).try_into()?;
    if current_manifest_version < CURRENT_MANIFEST_VERSION {
        upgrade_manifest(&mut manifest, current_manifest_version)?;
    } else if current_manifest_version > CURRENT_MANIFEST_VERSION {
        return Err(eyre!(
            "Manifest file is from a newer version of the program. Please update the program."
        ));
    }

    let manifest: Manifest = toml::from_str(&toml::to_string(&manifest)?)?;

    Ok(manifest)
}

fn create_templates(
    project_path: &std::path::Path,
    templates: &Vec<TemplateMapping>,
) -> Result<()> {
    for template in templates {
        let template_creator = template_management::get_template_creator(template.name.as_str())?;

        template_creator(&project_path, template)?;
    }

    Ok(())
}
