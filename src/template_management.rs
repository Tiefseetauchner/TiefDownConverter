use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};

pub(crate) fn get_template_creator(template: &str) -> Result<fn(&Path, &str) -> Result<()>> {
    if template.ends_with(".tex") {
        return Ok(create_tex_templates);
    } else if template.trim_matches('/').ends_with("_epub") {
        return Ok(create_epub_templates);
    } else if template.ends_with(".typ") {
        return Ok(create_typst_templates);
    } else {
        return Err(eyre!(
            "Unsupported template type for template name '{}'.",
            template
        ));
    }
}

fn create_tex_templates(project_path: &Path, template: &str) -> Result<()> {
    let template_dir = project_path.join("template");
    fs::create_dir_all(&template_dir)?;

    let content: Vec<u8> = match template {
        "template.tex" => {
            create_latex_meta(&template_dir)?;
            include_bytes!("resources/templates/default/default.tex").to_vec()
        }
        "booklet.tex" => {
            create_latex_meta(&template_dir)?;
            include_bytes!("resources/templates/default/booklet.tex").to_vec()
        }
        "lix_novel_a4.tex" => {
            create_lix_meta(&template_dir)?;
            println!("Using the lix_novel_a4 template. Make sure to install lix.sty and novel.cls. -h for more information.");
            include_bytes!("resources/templates/lix/lix_novel_a4.tex").to_vec()
        }
        "lix_novel_book.tex" => {
            create_lix_meta(&template_dir)?;
            println!("Using the lix_novel_book template. Make sure to install lix.sty and novel.cls. -h for more information.");
            include_bytes!("resources/templates/lix/lix_novel_book.tex").to_vec()
        }
        _ => return Err(eyre!("Unknown template: {}", template)),
    };

    let template_path = template_dir.join(template);
    fs::write(&template_path, content)?;

    Ok(())
}

fn create_latex_meta(template_dir: &PathBuf) -> Result<(), color_eyre::eyre::Error> {
    let meta_path = template_dir.join("meta.tex");
    Ok(if !meta_path.exists() {
        fs::write(
            &meta_path,
            include_bytes!("resources/templates/default/meta.tex"),
        )?;
        println!("meta.tex was written to the template directory. Make sure to adjust the metadata in the file.");
    })
}

fn create_lix_meta(template_dir: &PathBuf) -> Result<(), color_eyre::eyre::Error> {
    let meta_path = template_dir.join("meta_lix.tex");
    Ok(if !meta_path.exists() {
        fs::write(
            &meta_path,
            include_bytes!("resources/templates/lix/meta.tex"),
        )?;
        println!("meta_lix.tex was written to the template directory. Make sure to adjust the metadata in the file.");
    })
}

fn create_epub_templates(project_path: &Path, template: &str) -> Result<()> {
    let template_dir = project_path.join("template");
    fs::create_dir_all(&template_dir)?;

    fs::create_dir_all(&template_dir.join(template))?;

    Ok(())
}

fn create_typst_templates(project_path: &Path, template: &str) -> Result<()> {
    let template_dir = project_path.join("template");
    fs::create_dir_all(&template_dir)?;

    let content: Vec<u8> = match template {
        "template_typ.typ" => include_bytes!("resources/templates/default/default.typ").to_vec(),
        _ => return Err(eyre!("Unknown template: {}", template)),
    };

    let template_path = template_dir.join(template);
    fs::write(&template_path, content)?;

    let meta_path = template_dir.join("meta.typ");
    if !meta_path.exists() {
        fs::write(
            &meta_path,
            include_bytes!("resources/templates/default/meta.typ"),
        )?;
        println!("meta.typ was written to the template directory. Make sure to adjust the metadata in the file.");
    }

    Ok(())
}
