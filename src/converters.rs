use std::{
    fs,
    path::{Path, PathBuf},
    process::{Command, Stdio},
};

use color_eyre::eyre::{Ok, Result, eyre};

use crate::{
    TemplateType,
    manifest_model::{
        DEFAULT_TEX_PREPROCESSOR, DEFAULT_TYPST_PREPROCESSOR, PreProcessor, Processors,
        TemplateMapping,
    },
    template_management::{get_output_path, get_template_path},
};

pub(crate) fn convert_latex(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = compiled_directory_path.join(get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    )?);

    run_preprocessor_on_markdown(
        template,
        project_directory_path,
        compiled_directory_path,
        combined_markdown_path,
        &custom_processors.preprocessors,
        Some(&DEFAULT_TEX_PREPROCESSOR),
    )?;

    let mut processor_args = vec![];

    if let Some(processor) = &template.processor {
        if let Some(processor_pos) = custom_processors
            .processors
            .iter()
            .position(|p| p.name == *processor)
        {
            processor_args.extend(
                custom_processors.processors[processor_pos]
                    .processor_args
                    .clone(),
            );
        } else {
            return Err(eyre!(
                "Processor {} not found in custom processors.",
                processor
            ));
        }
    }

    compile_latex(compiled_directory_path, &template_path, &processor_args)?;
    compile_latex(compiled_directory_path, &template_path, &processor_args)?;

    let template_path = compiled_directory_path.join(template_path.with_extension("pdf"));
    if template_path.exists() && template_path.as_os_str() != output_path.as_os_str() {
        fs::copy(&template_path, &output_path)?;
    }

    Ok(output_path)
}

fn compile_latex(
    compiled_directory_path: &Path,
    template_path: &Path,
    processor_args: &Vec<String>,
) -> Result<()> {
    Command::new("xelatex")
        .current_dir(compiled_directory_path)
        .arg("-interaction=nonstopmode")
        .arg("-synctex=1")
        .arg(template_path)
        .args(processor_args)
        .stdout(Stdio::null())
        .status()?;

    Ok(())
}

pub(crate) fn convert_custom_pandoc(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    if template.processor != None {
        return Err(eyre!(
            "Custom Pandoc templates cannot have a processor. Use preprocessors instead.",
        ));
    }

    if template.preprocessor == None {
        return Err(eyre!(
            "Template type {} has to define a preprocessor.",
            TemplateType::CustomPandoc
        ));
    }

    let output_path = template.output.clone();

    if output_path == None {
        return Err(eyre!(
            "Output Path is required for Custom Pandoc conversions."
        ));
    }

    let output_path = output_path.unwrap();

    run_preprocessor_on_markdown(
        template,
        project_directory_path,
        compiled_directory_path,
        combined_markdown_path,
        &custom_processors.preprocessors,
        None,
    )?;

    let output_path = compiled_directory_path.join(&output_path);

    Ok(output_path)
}

pub(crate) fn convert_epub(
    project_directory_path: &Path,
    combined_markdown_path: &Path,
    compiled_directory_path: &Path,
    template: &TemplateMapping,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    if template.preprocessor.is_some() {
        return Err(eyre!(
            "EPUB conversion is not supported with a preprocessor. Use processors instead."
        ));
    }
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    )?;

    let template_path = compiled_directory_path.join(template_path);

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

    if let Some(processor) = &template.processor {
        if let Some(processor_pos) = custom_processors
            .processors
            .iter()
            .position(|p| p.name == *processor)
        {
            pandoc.args(
                custom_processors.processors[processor_pos]
                    .processor_args
                    .clone(),
            );
        } else {
            return Err(eyre!(
                "Processor {} not found in custom processors.",
                processor
            ));
        }
    }

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
    custom_processors: &Processors,
) -> Result<PathBuf> {
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    )?;

    run_preprocessor_on_markdown(
        template,
        project_directory_path,
        compiled_directory_path,
        combined_markdown_path,
        &custom_processors.preprocessors,
        Some(&DEFAULT_TYPST_PREPROCESSOR),
    )?;

    let mut processor_args = vec![];

    if let Some(processor) = &template.processor {
        if let Some(processor_pos) = custom_processors
            .processors
            .iter()
            .position(|p| p.name == *processor)
        {
            processor_args.extend(
                custom_processors.processors[processor_pos]
                    .processor_args
                    .clone(),
            );
        } else {
            return Err(eyre!(
                "Processor {} not found in custom processors.",
                processor
            ));
        }
    }

    Command::new("typst")
        .current_dir(compiled_directory_path)
        .arg("compile")
        .arg(template_path)
        .arg(&output_path)
        .args(processor_args)
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
    default_preprocessor: Option<&PreProcessor>,
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
    } else if let Some(preprocessor) = default_preprocessor {
        pandoc.args(&preprocessor.pandoc_args);
    } else {
        return Err(eyre!(
            "Preprocessor not defined and no custom preprocessor found for template '{}'",
            template.name
        ));
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
