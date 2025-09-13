use crate::{
    consts::POSSIBLE_TEMPLATES, manifest_model::TemplateMapping, template_type::TemplateType,
};
use color_eyre::eyre::{Result, eyre};
use log::{debug, info, warn};
use reqwest::blocking::get;
use std::{
    fs,
    io::{self, Write},
    path::{Path, PathBuf},
};

pub(crate) fn get_template_creator(
    template: &str,
) -> Result<fn(project_path: &Path, template: &TemplateMapping) -> Result<()>> {
    debug!("get_template_creator -> template: '{}'", template);
    if is_preset_template(template) {
        let template_type = get_template_type_from_path(template)?;
        debug!("Preset template detected. Type: {}", template_type);
        match template_type {
            TemplateType::Tex => Ok(create_tex_presets),
            TemplateType::Typst => Ok(create_typst_presets),
            TemplateType::Epub => Ok(create_epub_presets),
            _ => Err(eyre!(
                "No templates for {} Conversion found.",
                template_type
            )),
        }
    } else {
        Ok(|_, template| {
            info!(
                "Creating a custom template. Don't forget to add your template file. The template was created with the following parameters:"
            );
            debug!("  Template name: {}", template.name);
            debug!("  Template type: {}", &template.template_type);
            if let Some(file) = &template.template_file {
                debug!("  Template file: {}", file.display());
            }
            if let Some(output) = &template.output {
                debug!("  Output file: {}", output.display());
            }
            if let Some(filters) = &template.filters {
                debug!("  Filters: {}", filters.join(", "));
            }

            debug!("Custom template prepared.");
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
    debug!(
        "Wrote TeX preset template to '{}'.",
        template_path.display()
    );

    Ok(())
}

fn create_latex_meta(template_dir: &Path) -> Result<()> {
    let meta_path = template_dir.join("meta.tex");
    if !meta_path.exists() {
        fs::write(
            &meta_path,
            include_bytes!("resources/templates/default/meta.tex"),
        )?;
        info!(
            "meta.tex was written to the template directory. Make sure to adjust the metadata in the file."
        );
        debug!("Created 'meta.tex' in '{}'.", meta_path.display());
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
        info!(
            "meta_lix.tex was written to the template directory. Make sure to adjust the metadata in the file."
        );
        debug!("Created 'meta_lix.tex' in '{}'.", meta_path.display());
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

pub(crate) fn download_lix_files(template_dir: &Path) -> Result<()> {
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

    info!("Some required LaTeX files (licensed under GPLv3) are not included in this tool.");
    info!("Do you want to download them now? [Y/n] ");
    io::stdout().flush()?; // ensure the prompt is shown before read

    let mut input = String::new();
    io::stdin().read_line(&mut input)?;
    let input = input.trim().to_lowercase();

    if !["", "y", "yes"].contains(&input.as_str()) {
        warn!(
            "Download cancelled. You must manually install the required files. -h for more information."
        );
        return Ok(());
    }

    for file_url in &lix_files {
        let filename = file_url.split('/').last().unwrap();
        let file_path = template_dir.join(filename);

        if file_path.exists() {
            debug!("{} already exists, skipping...", filename);
            continue;
        }

        info!("Downloading {}...", filename);
        let response = get(*file_url)?;
        let content = response.bytes()?;
        fs::write(&file_path, &content)?;
        info!("Saved to {}", file_path.display());
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
    debug!(
        "Wrote Typst preset template to '{}'.",
        template_path.display()
    );

    let meta_path = template_dir.join("meta.typ");
    if !meta_path.exists() {
        fs::write(
            &meta_path,
            include_bytes!("resources/templates/default/meta.typ"),
        )?;
        info!(
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
    template_name: &str,
    template_type: TemplateType,
) -> Result<PathBuf> {
    Ok(output_path.unwrap_or(
        PathBuf::from(template_name).with_extension(get_template_output_extension(template_type)?),
    ))
}

pub(crate) fn get_template_output_extension(template_type: TemplateType) -> Result<&'static str> {
    match template_type {
        TemplateType::Tex => Ok("pdf"),
        TemplateType::Typst => Ok("pdf"),
        TemplateType::Epub => Ok("epub"),
        TemplateType::CustomPreprocessors => Err(eyre!(
            "Cannot determine the output extension of a custom conversion. Specify the output to be equal to the output of your preprocessor."
        )),
        TemplateType::CustomProcessor => Err(eyre!(
            "Cannot determine the output extension of a custom conversion. Specify the output to be equal to the output of your processor."
        )),
    }
}

pub(crate) fn get_template_type_from_path<P: AsRef<Path>>(path: P) -> Result<TemplateType> {
    let path = path.as_ref();

    if path.to_string_lossy().ends_with("_epub") {
        debug!("get_template_type_from_path: '{}' -> Epub", path.display());
        return Ok(TemplateType::Epub);
    }

    if let Some(extension) = path.extension().and_then(|ext| ext.to_str()) {
        match extension.to_lowercase().as_str() {
            "tex" => {
                debug!("get_template_type_from_path: '{}' -> Tex", path.display());
                return Ok(TemplateType::Tex);
            }
            "typ" => {
                debug!("get_template_type_from_path: '{}' -> Typst", path.display());
                return Ok(TemplateType::Typst);
            }
            _ => {}
        }
    }

    Err(eyre!(
        "Unknown template type for path '{}'.",
        path.display()
    ))
}
