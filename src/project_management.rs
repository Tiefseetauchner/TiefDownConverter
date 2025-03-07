use std::fs;

use color_eyre::eyre::{eyre, Result};

use crate::manifest_model::Manifest;

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
    let t = filter_templates(&templates, ".tex");
    if !t.is_empty() {
        create_tex_templates(&project_path, t)?;
    }

    let t = filter_templates(&templates, "_epub");
    if !t.is_empty() {
        create_epub_templates(&project_path, t)?;
    }

    let t = filter_templates(&templates, ".typ");
    if !t.is_empty() {
        create_typst_templates(&project_path, t)?;
    }

    Ok(())
}

fn filter_templates<'a>(templates: &'a Vec<String>, suffix: &str) -> Vec<&'a str> {
    templates
        .iter()
        .map(|t| t.as_str())
        .filter(|t| t.ends_with(suffix))
        .collect()
}

fn create_tex_templates(project_path: &std::path::Path, templates: Vec<&str>) -> Result<()> {
    let template_dir = project_path.join("template");
    std::fs::create_dir_all(&template_dir)?;

    for template in templates {
        let content: Vec<u8> = match template {
            "template.tex" => include_bytes!("resources/templates/default/default.tex").to_vec(),
            "booklet.tex" => include_bytes!("resources/templates/default/booklet.tex").to_vec(),
            "lix_novel_a4.tex" => {
                println!("Using the lix_novel_a4 template. Make sure to install lix.sty and novel.cls. -h for more information.");
                include_bytes!("resources/templates/lix_novel/lix_novel_a4.tex").to_vec()
            }
            "lix_novel_book.tex" => {
                println!("Using the lix_novel_book template. Make sure to install lix.sty and novel.cls. -h for more information.");
                include_bytes!("resources/templates/lix_novel/lix_novel_book.tex").to_vec()
            }
            _ => return Err(eyre!("Unknown template: {}", template)),
        };

        let template_path = template_dir.join(template);
        std::fs::write(&template_path, content)?;
    }

    let meta_path = template_dir.join("meta.tex");
    if !meta_path.exists() {
        std::fs::write(&meta_path, include_bytes!("resources/templates/meta.tex"))?;
        println!("meta.tex was written to the template directory. Make sure to adjust the metadata in the file.");
    }

    Ok(())
}

fn create_epub_templates(project_path: &std::path::Path, templates: Vec<&str>) -> Result<()> {
    let template_dir = project_path.join("template");
    std::fs::create_dir_all(&template_dir)?;

    for template in templates {
        fs::create_dir_all(&template_dir.join(template))?;
    }

    Ok(())
}

fn create_typst_templates(project_path: &std::path::Path, templates: Vec<&str>) -> Result<()> {
    let template_dir = project_path.join("template");
    std::fs::create_dir_all(&template_dir)?;

    for template in templates {
        let content: Vec<u8> = match template {
            "template_typ.typ" => {
                include_bytes!("resources/templates/default/default.typ").to_vec()
            }
            _ => return Err(eyre!("Unknown template: {}", template)),
        };

        let template_path = template_dir.join(template);
        std::fs::write(&template_path, content)?;
    }

    let meta_path = template_dir.join("meta.typ");
    if !meta_path.exists() {
        std::fs::write(&meta_path, include_bytes!("resources/templates/meta.typ"))?;
        println!("meta.typ was written to the template directory. Make sure to adjust the metadata in the file.");
    }

    Ok(())
}
