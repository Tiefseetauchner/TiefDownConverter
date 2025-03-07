use chrono::prelude::DateTime;
use chrono::prelude::Utc;
use color_eyre::eyre::eyre;
use color_eyre::eyre::Result;
use pandoc::Pandoc;
use std::fs;
use std::fs::DirEntry;
use std::path::Path;
use std::path::PathBuf;

use crate::conversion_decider;
use crate::manifest_model::Manifest;

pub(crate) fn convert(project: Option<String>, templates: Option<Vec<String>>) -> Result<()> {
    let project = project.unwrap_or_else(|| ".".to_string());
    let project_path = Path::new(&project);

    if !project_path.exists() {
        return Err(eyre!("Project path does not exist."));
    }

    let manifest_path = project_path.join("manifest.toml");
    if !manifest_path.exists() {
        return Err(eyre!(
            "No manifest file found. Please initialize a project first."
        ));
    }

    let manifest_content = fs::read_to_string(&manifest_path)?;
    let manifest: Manifest = toml::from_str(&manifest_content).unwrap();

    println!("Converting project: {}", project);

    let compiled_directory_path = create_build_directory(project_path)?;

    let combined_markdown_path = compiled_directory_path.join("combined.md");
    let markdown_dir = project_path.join(manifest.markdown_dir.unwrap_or("Markdown".to_string()));
    let mut combined_content = String::new();

    let markdown_files = get_markdown_files(markdown_dir)?;

    for entry in markdown_files {
        if entry.path().extension() == Some("md".as_ref()) {
            combined_content.push_str(&fs::read_to_string(entry.path())?);
            combined_content.push_str("\n\n");
        }
    }

    fs::write(&combined_markdown_path, combined_content)?;

    convert_md_to_tex(
        project_path,
        &compiled_directory_path,
        &combined_markdown_path,
    )?;

    convert_md_to_typst(&compiled_directory_path, &combined_markdown_path)?;

    let templates = templates.unwrap_or_else(|| manifest.templates);

    for template in &templates {
        convert_template(&compiled_directory_path, &template, &project_path)?;
    }

    Ok(())
}

fn convert_md_to_tex(
    project_path: &Path,
    compiled_directory_path: &PathBuf,
    combined_markdown_path: &PathBuf,
) -> Result<(), color_eyre::eyre::Error> {
    let mut pandoc = Pandoc::new();
    pandoc.add_input(&combined_markdown_path);
    pandoc.set_output(pandoc::OutputKind::File(
        compiled_directory_path.join("output.tex"),
    ));
    for filter in get_lua_filters(project_path)? {
        pandoc.add_option(pandoc::PandocOption::LuaFilter(filter.path()));
    }
    let pandoc_result = pandoc.execute();
    if pandoc_result.is_err() {
        return Err(eyre!(
            "Pandoc conversion to .tex failed: {}",
            pandoc_result.err().unwrap()
        ));
    }

    Ok(())
}

fn convert_md_to_typst(
    compiled_directory_path: &PathBuf,
    combined_markdown_path: &PathBuf,
) -> Result<(), color_eyre::eyre::Error> {
    let mut pandoc = Pandoc::new();
    pandoc.add_input(&combined_markdown_path);
    pandoc.set_output(pandoc::OutputKind::File(
        compiled_directory_path.join("output.typ"),
    ));
    let pandoc_result = pandoc.execute();

    if pandoc_result.is_err() {
        return Err(eyre!(
            "Pandoc conversion to .typ failed: {}",
            pandoc_result.err().unwrap()
        ));
    }

    Ok(())
}

fn create_build_directory(project_path: &Path) -> Result<std::path::PathBuf> {
    let current_time = std::time::SystemTime::now();
    let current_time: DateTime<Utc> = current_time.into();
    let current_time = current_time.format("%Y-%m-%d_%H-%M-%S").to_string();
    let compiled_directory_path = project_path.join(current_time);
    copy_dir::copy_dir(project_path.join("template/"), &compiled_directory_path)?;
    Ok(compiled_directory_path)
}

fn get_markdown_files(markdown_dir: std::path::PathBuf) -> Result<Vec<fs::DirEntry>> {
    let chapter_name_regex = regex::Regex::new(r"Chapter (\d+).*").unwrap();

    let mut markdown_files: Vec<_> = fs::read_dir(markdown_dir)?.filter_map(Result::ok).collect();

    markdown_files.sort_by_key(|entry| {
        let binding = entry.file_name();
        let filename = binding.to_string_lossy();
        chapter_name_regex
            .captures(&filename)
            .and_then(|caps| caps.get(1)?.as_str().parse::<u32>().ok())
            .unwrap_or(0)
    });
    Ok(markdown_files)
}

fn get_lua_filters(project_path: &Path) -> Result<Vec<DirEntry>> {
    let luafilters_path = project_path.join("luafilters");

    if !luafilters_path.exists() {
        return Ok(Vec::new());
    }
    let dirs: Vec<DirEntry> = fs::read_dir(luafilters_path)?
        .filter_map(Result::ok)
        .collect();

    Ok(dirs)
}

fn convert_template(
    compiled_directory_path: &PathBuf,
    template: &str,
    project_path: &Path,
) -> Result<()> {
    let template_path = compiled_directory_path.join(template);
    if !template_path.exists() {
        return Err(eyre!("Warning: Template path does not exist: {}", template));
    }

    let converter = conversion_decider::get_converter(template)?;

    let result_file_path = converter(&compiled_directory_path, &template)?;
    fs::copy(
        &result_file_path,
        project_path.join(result_file_path.file_name().unwrap_or_default()),
    )?;

    println!("Converted template: {}", template);
    Ok(())
}
