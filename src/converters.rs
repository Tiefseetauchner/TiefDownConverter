use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use color_eyre::eyre::{Ok, Result, eyre};

use crate::{
    manifest_model::{
        DEFAULT_TEX_PREPROCESSOR, DEFAULT_TYPST_PREPROCESSOR, PreProcessor, TemplateMapping,
    },
    template_management::{get_output_path, get_template_path},
};

pub(crate) fn convert_latex(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    preprocessors: &Vec<PreProcessor>,
) -> Result<PathBuf> {
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = compiled_directory_path.join(get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    ));

    run_preprocessor_on_markdown(
        template,
        project_directory_path,
        compiled_directory_path,
        combined_markdown_path,
        preprocessors,
        &DEFAULT_TEX_PREPROCESSOR,
    )?;

    compile_latex(compiled_directory_path, &template_path)?;
    compile_latex(compiled_directory_path, &template_path)?;

    let template_path = compiled_directory_path.join(template_path.with_extension("pdf"));
    if template_path.exists() && template_path.as_os_str() != output_path.as_os_str() {
        fs::copy(&template_path, &output_path)?;
    }

    Ok(output_path)
}

// NOTE: This requires xelatex to be installed. I don't particularly like that, but I tried tectonic and it didn't work.
//       For now we'll keep it simple and just use xelatex. I'm not sure if there's a way to get tectonic to work with the current setup.
fn compile_latex(compiled_directory_path: &Path, template_path: &Path) -> Result<()> {
    Command::new("xelatex")
        .current_dir(compiled_directory_path)
        .arg("-interaction=nonstopmode")
        .arg("-synctex=1")
        .arg(template_path)
        .stdout(Stdio::null())
        .status()?;

    Ok(())
}

pub(crate) fn convert_epub(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    _preprocessors: &Vec<PreProcessor>,
) -> Result<PathBuf> {
    if template.preprocessor.is_some() {
        return Err(eyre!(
            "EPUB conversion is not supported with a preprocessor. Please remove the preprocessor from the template."
        ));
    }
    let template_path = compiled_directory_path.join(get_template_path(
        template.template_file.clone(),
        &template.name,
    ));
    let output_path = get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    );

    let mut pandoc = Command::new("pandoc");
    pandoc
        .current_dir(compiled_directory_path)
        .arg("-f")
        .arg("markdown")
        .arg("-t")
        .arg("epub3")
        .arg("-o")
        .arg(&output_path);

    add_css_files(project_directory_path, &template_path, &mut pandoc)?;
    add_fonts(project_directory_path, &template_path, &mut pandoc)?;

    add_lua_filters(template, project_directory_path, &mut pandoc)?;

    pandoc
        .arg(combined_markdown_path)
        .stdout(Stdio::null())
        .status()?;

    let output_path = compiled_directory_path.join(output_path);

    Ok(output_path)
}

fn add_css_files(
    project_directory_path: &Path,
    template_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    let css_files = template_path.read_dir()?;
    for css_file in css_files {
        let css_file = css_file?.path();
        if css_file.is_file() && css_file.extension().unwrap_or_default() == "css" {
            pandoc
                .arg("-c")
                .arg(get_path_relative_to_compiled_directory(
                    &css_file,
                    project_directory_path,
                )?);
        }
    }

    Ok(())
}

fn add_fonts(
    project_directory_path: &Path,
    template_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    let fonts_dir = template_path.join("fonts");

    if !fonts_dir.exists() {
        return Ok(());
    }

    let font_files = fonts_dir.read_dir()?;

    for font_file in font_files {
        let font_file = font_file?.path();
        if font_file.is_file()
            && ["ttf", "otf", "woff"]
                .contains(&&*font_file.extension().unwrap_or_default().to_string_lossy())
        {
            pandoc
                .arg("--epub-embed-font")
                .arg(get_path_relative_to_compiled_directory(
                    &font_file,
                    project_directory_path,
                )?);
        }
    }

    Ok(())
}

pub(crate) fn convert_typst(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    preprocessors: &Vec<PreProcessor>,
) -> Result<PathBuf> {
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    );

    run_preprocessor_on_markdown(
        template,
        project_directory_path,
        compiled_directory_path,
        combined_markdown_path,
        preprocessors,
        &DEFAULT_TYPST_PREPROCESSOR,
    )?;

    Command::new("typst")
        .current_dir(compiled_directory_path)
        .arg("compile")
        .arg(template_path)
        .arg(&output_path)
        .stdout(Stdio::null())
        .status()?;

    let output_path = compiled_directory_path.join(output_path);

    Ok(output_path)
}

fn run_preprocessor_on_markdown(
    template: &TemplateMapping,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    combined_markdown_path: &Path,
    preprocessors: &Vec<PreProcessor>,
    default_preprocessor: &PreProcessor,
) -> Result<()> {
    let mut pandoc = Command::new("pandoc");

    if let Some(preprocessor) = template.preprocessor.as_ref() {
        if let Some(preprocessor) = preprocessors.iter().find(|p| &p.name == preprocessor) {
            pandoc.args(&preprocessor.pandoc_args);
        } else {
            return Err(eyre!(
                "Preprocessor {} not found. Please define it in your manifest file.",
                preprocessor
            ));
        }
    } else {
        pandoc.args(&default_preprocessor.pandoc_args);
    }

    pandoc
        .current_dir(compiled_directory_path)
        .arg("-f")
        .arg("markdown")
        .arg(combined_markdown_path);

    add_lua_filters(template, project_directory_path, &mut pandoc)?;

    pandoc.stdout(Stdio::null()).status()?;

    Ok(())
}

fn add_lua_filters(
    template: &TemplateMapping,
    project_directory_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    for filter in template.filters.clone().unwrap_or_default() {
        let filter = project_directory_path.join(&filter);

        if !filter.exists() {
            return Err(eyre!("Filter file does not exist: {}", filter.display()));
        }

        add_lua_filter_or_directory(project_directory_path, filter, pandoc)?;
    }

    Ok(())
}

fn add_lua_filter_or_directory(
    project_directory_path: &Path,
    filter: PathBuf,
    pandoc: &mut Command,
) -> Result<()> {
    if filter.is_dir() {
        for entry in fs::read_dir(filter)? {
            let entry = entry?.path();

            add_lua_filter_or_directory(project_directory_path, entry, pandoc)?;
        }
    } else if filter.is_file() && filter.extension().unwrap_or_default() == "lua" {
        pandoc
            .arg("--lua-filter")
            .arg(get_path_relative_to_compiled_directory(
                &filter,
                project_directory_path,
            )?);
    }

    Ok(())
}

fn get_path_relative_to_compiled_directory(
    original_path: &Path,
    project_directory_path: &Path,
) -> Result<PathBuf> {
    if project_directory_path.to_string_lossy() == "." {
        return Ok(PathBuf::from("../").join(original_path));
    }

    Ok(PathBuf::from("../").join(original_path.strip_prefix(project_directory_path)?))
}
