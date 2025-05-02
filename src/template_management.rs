use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

use color_eyre::eyre::{Result, eyre};
use reqwest::blocking::get;

use crate::{
    consts::POSSIBLE_TEMPLATES, manifest_model::TemplateMapping, template_type::TemplateType,
};

pub(crate) fn get_template_creator(
    template: &str,
) -> Result<fn(project_path: &Path, template: &TemplateMapping) -> Result<()>> {
    if is_preset_template(template) {
        let template_type = get_template_type_from_path(template)?;
        match template_type {
            TemplateType::Tex => Ok(create_tex_presets),
            TemplateType::Typst => Ok(create_typst_presets),
            TemplateType::Epub => Ok(create_epub_presets),
            TemplateType::CustomPandoc => {
                Err(eyre!("No templates for Custom Pandoc Conversion found."))
            }
        }
    } else {
        Ok(|_, template| {
            println!(
                "Creating a custom template. Don't forget to add your template file. The template was created with the following parameters:"
            );
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
        })
    }
}

pub(crate) fn add_lix_filters(template: &mut TemplateMapping) {
    if is_preset_template(&template.name)
        && ["lix_novel_a4.tex", "lix_novel_book.tex"].contains(&template.name.as_str())
    {
        if let Some(filters) = &mut template.filters {
            filters.push("lix_chapter_filter.lua".to_string());
        } else {
            template.filters = Some(vec!["lix_chapter_filter.lua".to_string()]);
        }
    }
}

fn is_preset_template(template: &str) -> bool {
    POSSIBLE_TEMPLATES.contains(&template)
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
            create_lix_luafilter(&template_dir)?;
            download_lix_files(&template_dir)?;
            include_bytes!("resources/templates/lix/lix_novel_a4.tex").to_vec()
        }
        "lix_novel_book.tex" => {
            create_lix_meta(&template_dir)?;
            create_lix_luafilter(&project_path)?;
            download_lix_files(&template_dir)?;
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

fn create_latex_meta(template_dir: &Path) -> Result<(), color_eyre::eyre::Error> {
    let meta_path = template_dir.join("meta.tex");
    if !meta_path.exists() {
        fs::write(
            &meta_path,
            include_bytes!("resources/templates/default/meta.tex"),
        )?;
        println!(
            "meta.tex was written to the template directory. Make sure to adjust the metadata in the file."
        );
    };
    Ok(())
}

fn create_lix_meta(template_dir: &Path) -> Result<()> {
    let meta_path = template_dir.join("meta_lix.tex");
    if !meta_path.exists() {
        fs::write(
            &meta_path,
            include_bytes!("resources/templates/lix/meta.tex"),
        )?;
        println!(
            "meta_lix.tex was written to the template directory. Make sure to adjust the metadata in the file."
        );
    };
    Ok(())
}

fn create_lix_luafilter(project_path: &Path) -> Result<()> {
    let lufilter_path = project_path.join("lix_chapter_filter.lua");
    if !lufilter_path.exists() {
        fs::write(
            &lufilter_path,
            include_bytes!("resources/templates/lix/lix_chapter_filter.lua"),
        )?;
    };
    Ok(())
}

pub fn download_lix_files(template_dir: &Path) -> Result<()> {
    let lix_files = [
        "https://raw.githubusercontent.com/NicklasVraa/LiX/refs/heads/master/lix.sty",
        "https://raw.githubusercontent.com/NicklasVraa/LiX/refs/heads/master/classes/custom_classes/novel.cls",
    ];

    let all_exist = lix_files.iter().all(|file_url| {
        let filename = file_url.split('/').last().unwrap();
        let file_path = template_dir.join(filename);
        file_path.exists()
    });

    if all_exist {
        return Ok(());
    }

    println!("Some required LaTeX files (licensed under GPLv3) are not included in this tool.");
    print!("Do you want to download them now? [Y/n] ");
    io::stdout().flush()?; // ensure the prompt is shown before read

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if !["", "y", "yes"].contains(&input.as_str()) {
        println!(
            "Download cancelled. You must manually install the required files. -h for more information."
        );
        return Ok(());
    }

    for file_url in &lix_files {
        let filename = file_url.split('/').last().unwrap();
        let file_path = template_dir.join(filename);

        if file_path.exists() {
            println!("{} already exists, skipping...", filename);
            continue;
        }

        println!("Downloading {}...", filename);
        let response = get(*file_url)?;
        let content = response.bytes()?;
        fs::write(&file_path, &content)?;
        println!("Saved to {}", file_path.display());
    }

    Ok(())
}

fn create_epub_presets(project_path: &Path, template: &TemplateMapping) -> Result<()> {
    let template_dir = project_path.join("template");
    fs::create_dir_all(&template_dir)?;

    fs::create_dir_all(template_dir.join(get_template_path(
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
        println!(
            "meta.typ was written to the template directory. Make sure to adjust the metadata in the file."
        );
    }

    Ok(())
}

pub(crate) fn get_template_path(template_file: Option<PathBuf>, template_name: &str) -> PathBuf {
    template_file.unwrap_or(PathBuf::from(template_name))
}

pub(crate) fn get_output_path(
    output_path: Option<PathBuf>,
    template_path: &Path,
    template_type: TemplateType,
) -> Result<PathBuf> {
    Ok(output_path
        .unwrap_or(template_path.with_extension(get_template_output_extension(template_type)?)))
}

pub(crate) fn get_template_output_extension(template_type: TemplateType) -> Result<&'static str> {
    match template_type {
        TemplateType::Tex => Ok("pdf"),
        TemplateType::Typst => Ok("pdf"),
        TemplateType::Epub => Ok("epub"),
        TemplateType::CustomPandoc => Err(eyre!(
            "Cannot determine the output extension of a custom conversion. Specify the output to be equal to the output of your preprocessor."
        )),
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
