use std::error::Error;

use crate::manifest_model::Manifest;

pub fn init(
    project: Option<String>,
    force: bool,
    markdown_dir: Option<String>,
) -> Result<(), Box<dyn Error>> {
    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = std::path::Path::new(&project);

    if project_path.exists() && force {
        if project == "." {
            return Err("Cannot force initialization in the current directory.".into());
        }
        std::fs::remove_dir_all(project_path)?;
    }

    if !project_path.exists() {
        std::fs::create_dir(project_path)?;
    }

    let manifest_path = project_path.join("manifest.toml");
    if manifest_path.exists() {
        return Err(
            "Manifest file already exists. Please remove it before initializing a new project."
                .into(),
        );
    }

    let templates = vec!["template.tex".to_string()];

    let markdown_dir_path =
        project_path.join(markdown_dir.clone().unwrap_or("Markdown".to_string()));
    if !markdown_dir_path.exists() {
        std::fs::create_dir(&markdown_dir_path)?;
        std::fs::write(
            &markdown_dir_path.join("Chapter 1: Introduction.md"),
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

    create_latex_templates(&project_path, templates)?;

    Ok(())
}

fn create_latex_templates(
    project_path: &std::path::Path,
    templates: Vec<String>,
) -> Result<(), Box<dyn Error>> {
    let template_dir = project_path.join("template");
    std::fs::create_dir_all(&template_dir)?;

    for template in templates {
        let content = match template.as_str() {
            "template.tex" => include_bytes!("resources/templates/article/default.tex"),
            _ => return Err(format!("Unknown template: {}", template).into()),
        };

        let template_path = template_dir.join(&template);
        std::fs::write(&template_path, content)?;
    }

    let meta_content = include_bytes!("resources/templates/article/meta.tex");
    let meta_path = template_dir.join("meta.tex");
    std::fs::write(&meta_path, meta_content)?;

    Ok(())
}
