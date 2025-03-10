use std::{
    fs,
    path::{Path, PathBuf},
};

use color_eyre::eyre::{eyre, Result};

use crate::{
    consts::POSSIBLE_TEMPLATES,
    manifest_model::{TemplateMapping, TemplateType},
};

pub(crate) fn get_template_creator(
    template: &str,
) -> Result<fn(project_path: &Path, template: &TemplateMapping) -> Result<()>> {
    if is_preset_template(template) {
        let template_type = get_template_type_from_path(template)?;
        return match template_type {
            TemplateType::Tex => Ok(create_tex_presets),
            TemplateType::Typst => Ok(create_typst_presets),
            TemplateType::Epub => Ok(create_epub_presets),
        };
    } else {
        return Ok(|_, template| {
            println!("Creating a custom template. Don't forget to add your template file. The template was created with the following parameters:");
            println!("  Template name: {}", template.name);
            println!("  Template type: {}", &template.template_type);
            if let Some(file) = &template.template_file {
                println!("  Template file: {}", file.display());
            }
            if let Some(output) = &template.output {
                println!("  Output file: {}", output.display());
            }
            if let Some(filters) = &template.filters {
                println!("  Filters: {}", filters.join(", "));
            }

            Ok(())
        });
    }
}

fn is_preset_template(template: &str) -> bool {
    return POSSIBLE_TEMPLATES.contains(&template);
}

fn create_tex_presets(project_path: &Path, template: &TemplateMapping) -> Result<()> {
    let template_dir = project_path.join("template");
    fs::create_dir_all(&template_dir)?;

    let content: Vec<u8> = match template.name.as_str() {
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
        _ => return Err(eyre!("Unknown template: {}", template.name.as_str())),
    };

    let template_path = template_dir.join(get_template_path(
        template.template_file.clone(),
        &template.name,
    ));
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

fn create_epub_presets(project_path: &Path, template: &TemplateMapping) -> Result<()> {
    let template_dir = project_path.join("template");
    fs::create_dir_all(&template_dir)?;

    fs::create_dir_all(&template_dir.join(get_template_path(
        template.template_file.clone(),
        &template.name,
    )))?;

    Ok(())
}

fn create_typst_presets(project_path: &Path, template: &TemplateMapping) -> Result<()> {
    let template_dir = project_path.join("template");
    fs::create_dir_all(&template_dir)?;

    let content: Vec<u8> = match template.name.as_str() {
        "template_typ.typ" => include_bytes!("resources/templates/default/default.typ").to_vec(),
        _ => return Err(eyre!("Unknown template: {}", template.name)),
    };

    let template_path = template_dir.join(get_template_path(
        template.template_file.clone(),
        &template.name,
    ));
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

pub(crate) fn get_template_path(template_file: Option<PathBuf>, template_name: &str) -> PathBuf {
    template_file.unwrap_or(PathBuf::from(template_name))
}

pub(crate) fn get_output_path(
    output_path: Option<PathBuf>,
    template_path: &PathBuf,
    template_type: TemplateType,
) -> PathBuf {
    output_path
        .unwrap_or(template_path.with_extension(get_template_output_extension(template_type)))
}

pub(crate) fn get_template_output_extension(template_type: TemplateType) -> &'static str {
    match template_type {
        TemplateType::Tex => "pdf",
        TemplateType::Typst => "pdf",
        TemplateType::Epub => "epub",
    }
}

pub(crate) fn get_template_type_from_path<P: AsRef<Path>>(path: P) -> Result<TemplateType> {
    let path = path.as_ref();

    if path.to_string_lossy().ends_with("_epub") {
        return Ok(TemplateType::Epub);
    }

    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "tex" => return Ok(TemplateType::Tex),
            "typ" => return Ok(TemplateType::Typst),
            _ => {}
        }
    }

    Err(eyre!(
        "Unknown template type for path '{}'.",
        path.display()
    ))
}
