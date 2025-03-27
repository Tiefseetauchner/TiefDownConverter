use chrono::prelude::DateTime;
use chrono::prelude::Utc;
use color_eyre::eyre::Result;
use color_eyre::eyre::eyre;
use std::fs;
use std::path::Path;
use std::path::PathBuf;

use crate::conversion_decider;
use crate::manifest_model::Manifest;
use crate::manifest_model::PreProcessor;
use crate::manifest_model::TemplateMapping;
use crate::project_management::load_and_convert_manifest;
use crate::project_management::run_smart_clean;

pub(crate) fn convert(
    project: Option<String>,
    templates: Option<Vec<String>>,
    profile: Option<String>,
) -> Result<()> {
    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = Path::new(&project);

    if !project_path.exists() {
        return Err(eyre!("Project path does not exist."));
    }

    let manifest_path = project_path.join("manifest.toml");
    let manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(true) = manifest.smart_clean {
        let threshold = manifest.smart_clean_threshold.unwrap_or(5);
        run_smart_clean(project_path, threshold.saturating_sub(1))?;
    }

    println!("Converting project: {}", project);

    let compiled_directory_path = create_build_directory(project_path)?;

    let combined_markdown_name = PathBuf::from("combined.md");
    let combined_markdown_path = compiled_directory_path.join(&combined_markdown_name);
    let markdown_dir = project_path.join(manifest.markdown_dir.as_deref().unwrap_or("Markdown"));

    let combined_content = combine_markdown(&combined_markdown_path, &markdown_dir)?;
    fs::write(&combined_markdown_path, combined_content)?;

    let templates = get_template_names(templates, profile, &manifest)?;
    let templates = get_template_mappings_from_names(&templates, &manifest)?;

    for template in &templates {
        convert_template(
            &combined_markdown_name,
            &compiled_directory_path,
            template,
            project_path,
            &manifest.custom_processors.preprocessors,
        )?;
    }

    Ok(())
}

fn get_template_names(
    templates: Option<Vec<String>>,
    profile: Option<String>,
    manifest: &Manifest,
) -> Result<Vec<String>> {
    if let Some(profile) = profile {
        if let Some(available_profiles) = &manifest.profiles {
            if let Some(profile_pos) = available_profiles.iter().position(|p| p.name == profile) {
                return Ok(available_profiles[profile_pos]
                    .templates
                    .iter()
                    .map(|t| t.clone())
                    .collect());
            } else {
                return Err(eyre!("Profile '{}' could not be found.", profile));
            }
        } else {
            return Err(eyre!("No profiles are defined in the manifest.toml file."));
        }
    }

    if let Some(templates) = templates {
        return Ok(templates);
    }

    Ok(manifest.templates.iter().map(|t| t.name.clone()).collect())
}

fn get_template_mappings_from_names(
    templates: &Vec<String>,
    manifest: &Manifest,
) -> Result<Vec<TemplateMapping>> {
    let templates = templates
        .iter()
        .map(|t| manifest.templates.iter().find(|mapping| mapping.name == *t))
        .filter_map(|t| t.cloned())
        .collect::<Vec<_>>();

    Ok(templates)
}

fn create_build_directory(project_path: &Path) -> Result<std::path::PathBuf> {
    let current_time = std::time::SystemTime::now();
    let current_time: DateTime<Utc> = current_time.into();
    let current_time = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
    let compiled_directory_path = project_path.join(current_time);
    copy_dir::copy_dir(project_path.join("template/"), &compiled_directory_path)?;
    Ok(compiled_directory_path)
}

fn combine_markdown(_combined_markdown_path: &PathBuf, markdown_dir: &PathBuf) -> Result<String> {
    let markdown_files = get_markdown_files(markdown_dir)?;

    let mut combined_content = String::new();

    for entry in markdown_files {
        if entry.path().extension() == Some("md".as_ref()) {
            combined_content.push_str(&fs::read_to_string(entry.path())?);
            combined_content.push_str("\n\n");
        } else if entry.path().is_dir() {
            combined_content.push_str(&combine_markdown(_combined_markdown_path, &entry.path())?);
        }
    }

    Ok(combined_content)
}

fn get_markdown_files(markdown_dir: &PathBuf) -> Result<Vec<fs::DirEntry>> {
    let chapter_name_regex = regex::Regex::new(r"Chapter (\d+).*").unwrap();

    let mut markdown_files: Vec<_> = fs::read_dir(markdown_dir)?.filter_map(Result::ok).collect();

    markdown_files.sort_by(|a, b| {
        let a_binding = a.file_name();
        let b_binding = b.file_name();
        let a_name = a_binding.to_string_lossy();
        let b_name = b_binding.to_string_lossy();

        let a_num = chapter_name_regex
            .captures(&a_name)
            .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
            .unwrap_or(0);
        let b_num = chapter_name_regex
            .captures(&b_name)
            .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
            .unwrap_or(0);

        match a_num.cmp(&b_num) {
            std::cmp::Ordering::Equal => {
                let a_is_file = a.metadata().map(|m| m.is_file()).unwrap_or(false);
                let b_is_file = b.metadata().map(|m| m.is_file()).unwrap_or(false);
                match (a_is_file, b_is_file) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            }
            other => other,
        }
    });

    Ok(markdown_files)
}

fn convert_template(
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    project_path: &Path,
    preprocessors: &Vec<PreProcessor>,
) -> Result<()> {
    let converter = conversion_decider::get_converter(&template.template_type)?;

    let result_file_path = converter(
        project_path,
        combined_markdown_path,
        compiled_directory_path,
        template,
        preprocessors,
    )?;

    fs::copy(
        &result_file_path,
        project_path.join(result_file_path.file_name().unwrap_or_default()),
    )?;

    println!("Converted template: {}", template.name);
    Ok(())
}
