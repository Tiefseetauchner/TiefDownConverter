use std::fs;

use color_eyre::eyre::{eyre, Result};

use crate::{manifest_model::Manifest, template_management};

pub fn init(
    project: Option<String>,
    templates: Option<Vec<String>>,
    force: bool,
    markdown_dir: Option<String>,
) -> Result<()> {
    let project = project.unwrap_or_else(|| ".".to_string());
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
            "Manifest file already exists. Please remove it before initializing a new project."
        ));
    }

    let templates = templates.unwrap_or(vec!["template.tex".to_string()]);

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
        markdown_dir,
        templates: templates.clone(),
    };

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    create_templates(project_path, templates)?;

    Ok(())
}

pub(crate) fn add_template(project: Option<String>, template: String) -> Result<()> {
    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = std::path::Path::new(&project);

    let manifest_path = project_path.join("manifest.toml");
    if !manifest_path.exists() {
        return Err(eyre!(
            "Manifest file does not exist. Please initialize a project before adding templates."
        ));
    }

    let manifest_content = fs::read_to_string(&manifest_path)?;
    let mut manifest: Manifest = toml::from_str(&manifest_content)?;
    manifest.templates.extend([template.clone()]);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    create_templates(project_path, vec![template.clone()])?;

    Ok(())
}

pub(crate) fn remove_template(project: Option<String>, template: String) -> Result<()> {
    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = std::path::Path::new(&project);

    let manifest_path = project_path.join("manifest.toml");
    if !manifest_path.exists() {
        return Err(eyre!(
            "Manifest file does not exist. Please initialize a project before removing templates."
        ));
    }

    let manifest_content = fs::read_to_string(&manifest_path)?;
    let mut manifest: Manifest = toml::from_str(&manifest_content)?;
    manifest.templates.retain(|t| t != &template);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    let template_dir = project_path.join("template");
    let template_path = template_dir.join(&template);

    fs::remove_file(template_path)?;

    Ok(())
}

fn create_templates(
    project_path: &std::path::Path,
    templates: Vec<String>,
) -> Result<(), color_eyre::eyre::Error> {
    for template in templates {
        let template_creator = template_management::get_template_creator(template.as_str())?;

        template_creator(&project_path, &template)?;
    }

    Ok(())
}
