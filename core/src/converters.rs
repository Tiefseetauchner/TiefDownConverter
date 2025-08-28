use crate::{
    manifest_model::{
        DEFAULT_TEX_PREPROCESSORS, DEFAULT_TYPST_PREPROCESSORS, MetadataSettings, PreProcessor,
        PreProcessors, Processors, TemplateMapping,
    },
    template_management::{get_output_path, get_template_path},
    template_type::TemplateType,
};
use color_eyre::eyre::{Ok, Result, eyre};
use fast_glob::glob_match;
use log::{debug, error};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    io::{BufRead, BufReader, Write},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};
use toml::Table;

pub(crate) fn convert_latex(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &TemplateMapping,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = compiled_directory_path.join(get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    )?);

    let preprocessors =
        retrieve_preprocessors(&template.preprocessors, &custom_processors.preprocessors);
    let default_preprocessors = retrieve_preprocessors(
        &Some(DEFAULT_TEX_PREPROCESSORS.0.clone()),
        &DEFAULT_TEX_PREPROCESSORS.1,
    );
    let preprocessors = merge_preprocessors(vec![preprocessors, default_preprocessors]);

    let combined_output =
        retrieve_combined_output(template, &Some(DEFAULT_TEX_PREPROCESSORS.0.clone()))?;

    run_preprocessor_on_inputs(
        template,
        project_directory_path,
        compiled_directory_path,
        conversion_input_dir,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        &combined_output,
    )?;

    generate_tex_metadata(compiled_directory_path, metadata_fields, metadata_settings)?;

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

fn generate_tex_metadata(
    compiled_directory_path: &Path,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
) -> Result<()> {
    let metadata_path = compiled_directory_path.join("metadata.tex");
    if metadata_path.exists() {
        return Ok(());
    }

    let mut metadata_file = fs::File::create(&metadata_path)?;
    let mut metadata_file_content = String::new();

    let prefix = metadata_settings
        .metadata_prefix
        .as_deref()
        .unwrap_or("meta");

    metadata_file_content.push_str(
        format!(
            r"\newcommand{{\{}}}[1]{{\csname {}@#1\endcsname}}",
            prefix, prefix
        )
        .as_str(),
    );

    metadata_file_content.push_str("\n\n");

    for (key, value) in metadata_fields {
        if let Some(value) = value.as_str() {
            metadata_file_content.push_str(&format!(
                r"\expandafter\def\csname {}@{}\endcsname{{{}}}",
                prefix, key, value
            ));
            metadata_file_content.push('\n');
        } else {
            return Err(eyre!(
                "Metadata field {} is not a string, and is not supported by TiefDownConverter.",
                key
            ));
        }
    }

    metadata_file.write_all(metadata_file_content.as_bytes())?;

    Ok(())
}

fn compile_latex(
    compiled_directory_path: &Path,
    template_path: &Path,
    processor_args: &Vec<String>,
) -> Result<()> {
    let mut latex_command = Command::new("xelatex");

    latex_command
        .current_dir(compiled_directory_path)
        .arg("-interaction=nonstopmode")
        .arg("-synctex=1")
        .arg(template_path)
        .args(processor_args);

    run_with_logging(latex_command, "xelatex", false)?;

    Ok(())
}

pub(crate) fn convert_custom_pandoc(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &TemplateMapping,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    if template.processor != None {
        return Err(eyre!(
            "Custom Pandoc templates cannot have a processor. Use preprocessors instead.",
        ));
    }

    if template.preprocessors.is_none() {
        return Err(eyre!(
            "Template type {} has to define a preprocessor.",
            TemplateType::CustomPandoc
        ));
    }

    let output_path = template.output.clone();

    let Some(output_path) = output_path else {
        return Err(eyre!(
            "Output Path is required for Custom Pandoc conversions."
        ));
    };

    let preprocessors =
        retrieve_preprocessors(&template.preprocessors, &custom_processors.preprocessors);

    let combined_output = retrieve_combined_output(template, &None)?;

    run_preprocessor_on_inputs(
        template,
        project_directory_path,
        compiled_directory_path,
        conversion_input_dir,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        &combined_output,
    )?;

    let output_path = compiled_directory_path.join(&output_path);

    Ok(output_path)
}

pub(crate) fn convert_epub(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &TemplateMapping,
    metadata_fields: &Table,
    _metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    if template.preprocessors.is_some() {
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

    add_meta_args(metadata_fields, &mut pandoc)?;

    add_css_files(
        project_directory_path,
        compiled_directory_path,
        &template_path,
        &mut pandoc,
    )?;
    add_fonts(
        project_directory_path,
        compiled_directory_path,
        &template_path,
        &mut pandoc,
    )?;

    add_lua_filters(
        template,
        project_directory_path,
        compiled_directory_path,
        &mut pandoc,
    )?;

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

    pandoc.args(get_sorted_files(
        conversion_input_dir,
        project_directory_path,
        compiled_directory_path,
    )?);

    run_with_logging(pandoc, "pandoc", false)?;

    let output_path = compiled_directory_path.join(output_path);

    Ok(output_path)
}

fn add_meta_args(metadata_fields: &Table, pandoc: &mut Command) -> Result<()> {
    for (key, value) in metadata_fields {
        if let Some(value) = value.as_str() {
            pandoc.arg("-M").arg(format!("{}:{}", key, value));
        } else {
            return Err(eyre!(
                "Metadata field {} is not a string, and is not supported by TiefDownConverter.",
                key
            ));
        }
    }

    Ok(())
}

fn add_css_files(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    template_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    let css_files = template_path.read_dir()?;
    for css_file in css_files {
        let css_file = css_file?.path();
        if css_file.is_file() && css_file.extension().unwrap_or_default() == "css" {
            pandoc.arg("-c").arg(
                get_relative_path_from_compiled_dir(
                    &css_file,
                    project_directory_path,
                    compiled_directory_path,
                )
                .unwrap_or(css_file),
            );
        }
    }

    Ok(())
}

fn add_fonts(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
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
            pandoc.arg("--epub-embed-font").arg(
                get_relative_path_from_compiled_dir(
                    &font_file,
                    project_directory_path,
                    compiled_directory_path,
                )
                .unwrap_or(font_file),
            );
        }
    }

    Ok(())
}

pub(crate) fn convert_typst(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    template: &TemplateMapping,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
    custom_processors: &Processors,
) -> Result<PathBuf> {
    let template_path = get_template_path(template.template_file.clone(), &template.name);
    let output_path = get_output_path(
        template.output.clone(),
        &template_path,
        template.template_type.clone(),
    )?;

    let preprocessors =
        retrieve_preprocessors(&template.preprocessors, &custom_processors.preprocessors);
    let default_preprocessors = retrieve_preprocessors(
        &Some(DEFAULT_TYPST_PREPROCESSORS.0.clone()),
        &DEFAULT_TYPST_PREPROCESSORS.1,
    );
    let preprocessors = merge_preprocessors(vec![preprocessors, default_preprocessors]);

    let combined_output =
        retrieve_combined_output(template, &Some(DEFAULT_TYPST_PREPROCESSORS.0.clone()))?;

    run_preprocessor_on_inputs(
        template,
        project_directory_path,
        compiled_directory_path,
        conversion_input_dir,
        metadata_fields,
        metadata_settings,
        &preprocessors,
        &combined_output,
    )?;

    generate_typst_metadata(compiled_directory_path, metadata_fields, metadata_settings)?;

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

    let mut typst_command = Command::new("typst");

    typst_command
        .current_dir(compiled_directory_path)
        .arg("compile")
        .arg(template_path)
        .arg(&output_path)
        .args(processor_args);

    run_with_logging(typst_command, "typst", false)?;

    let output_path = compiled_directory_path.join(output_path);

    Ok(output_path)
}

fn generate_typst_metadata(
    compiled_directory_path: &Path,
    metadata_fields: &Table,
    metadata_settings: &MetadataSettings,
) -> Result<()> {
    let metadata_path = compiled_directory_path.join("metadata.typ");
    if metadata_path.exists() {
        return Ok(());
    }

    let mut metadata_file = fs::File::create(&metadata_path)?;
    let mut metadata_file_content = String::new();

    let prefix = metadata_settings
        .metadata_prefix
        .as_deref()
        .unwrap_or("meta");

    metadata_file_content.push_str(format!(r"#let {} = (", prefix).as_str());
    metadata_file_content.push_str("\n");

    for (key, value) in metadata_fields.iter() {
        if let Some(value) = value.as_str() {
            metadata_file_content.push_str(format!(r#"  {}: "{}","#, key, value).as_str());
            metadata_file_content.push_str("\n");
        } else {
            return Err(eyre!(
                "Metadata field {} is not a string, and is not supported by TiefDownConverter.",
                key
            ));
        }
    }

    metadata_file_content.push_str(")");

    metadata_file.write_all(metadata_file_content.as_bytes())?;

    Ok(())
}

fn retrieve_preprocessors(
    preprocessors: &Option<PreProcessors>,
    custom_preprocessors: &Vec<PreProcessor>,
) -> Vec<PreProcessor> {
    preprocessors
        .clone()
        .and_then(|p| Some(p.preprocessors.clone()))
        .as_ref()
        .map(|p| {
            custom_preprocessors
                .iter()
                .filter(|cp| p.contains(&cp.name))
                .cloned()
                .collect::<Vec<PreProcessor>>()
        })
        .unwrap_or(vec![])
}

fn merge_preprocessors(preprocessor_lists: Vec<Vec<PreProcessor>>) -> Vec<PreProcessor> {
    let mut merged = vec![];

    for preprocessors in preprocessor_lists.iter() {
        for preprocessor in preprocessors {
            if !merged
                .iter()
                .any(|p: &PreProcessor| p.extension_filter == preprocessor.extension_filter)
            {
                merged.push(preprocessor.clone());
            }
        }
    }

    merged
}

fn retrieve_combined_output(
    template: &TemplateMapping,
    default_processors: &Option<PreProcessors>,
) -> Result<PathBuf> {
    Ok(template
        .preprocessors
        .clone()
        .and_then(|p| Some(p.combined_output))
        .or(default_processors
            .as_ref()
            .and_then(|p| Some(p.clone().combined_output)))
        .ok_or(eyre!(
            "No combined output defined for this template's preprocessor."
        ))?)
}

fn run_preprocessor_on_inputs(
    template: &TemplateMapping,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    metadata_fields: &Table,
    _metadata_settings: &MetadataSettings,
    preprocessors: &Vec<PreProcessor>,
    combined_output: &Path,
) -> Result<()> {
    let input_files = get_sorted_files(
        conversion_input_dir,
        project_directory_path,
        compiled_directory_path,
    )?;

    let chunks = get_preprocessing_chunks(&input_files)?;

    let results = chunks
        .par_iter()
        .map(|chunk| {
            let preprocessor = preprocessors
                .iter()
                .filter(|p| p.extension_filter.is_some())
                .find(|p| glob_match(p.extension_filter.as_ref().unwrap(), chunk.1.clone()))
                .or(preprocessors.iter().find(|p| p.extension_filter.is_none()))
                .ok_or(eyre!(
                    "No preprocessor found for files with extension {}",
                    chunk.1
                ))?;

            let cli_name = preprocessor.cli.clone().unwrap_or("pandoc".to_string());
            let cli_args = preprocess_cli_args(&preprocessor.cli_args, &metadata_fields);

            let mut cli = Command::new(&cli_name);
            cli.args(&cli_args);
            cli.current_dir(compiled_directory_path);
            add_lua_filters(
                template,
                project_directory_path,
                compiled_directory_path,
                &mut cli,
            )?;
            cli.args(chunk.0.clone());
            run_with_logging(cli, &cli_name, true)
        })
        .collect::<Result<Vec<_>>>()?;

    std::fs::write(
        compiled_directory_path.join(&combined_output),
        results.join("\n\n"),
    )?;

    Ok(())
}

fn get_preprocessing_chunks(input_files: &Vec<PathBuf>) -> Result<Vec<(Vec<PathBuf>, String)>> {
    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();
    let mut chunk_extension = std::ffi::OsString::new();

    for input_file in input_files {
        if current_chunk.is_empty() {
            current_chunk.push(input_file.clone());
            continue;
        }

        let current_extension = input_file.extension().ok_or(eyre!(
            "Input file {} has no extension",
            input_file.display()
        ))?;
        chunk_extension = current_chunk
            .last()
            .and_then(|file: &PathBuf| file.extension())
            .ok_or(eyre!(
                "Input file {} has no extension",
                input_file.display()
            ))?
            .to_owned();

        if current_extension != chunk_extension {
            chunks.push((current_chunk, chunk_extension.to_string_lossy().to_string()));
            current_chunk = Vec::new();
        }

        current_chunk.push(input_file.clone());
    }

    if !current_chunk.is_empty() {
        chunks.push((current_chunk, chunk_extension.to_string_lossy().to_string()));
    }

    Ok(chunks)
}

fn preprocess_cli_args(cli_args: &[String], metadata_fields: &Table) -> Vec<String> {
    let mut processed_args = Vec::new();

    for arg in cli_args.iter() {
        let mut processed_arg = arg.clone();
        for (metadata_key, metadata_value) in metadata_fields.iter() {
            let value = metadata_value.as_str().unwrap_or("");
            processed_arg = processed_arg.replace(&format!("{{{{{}}}}}", metadata_key), value);
        }
        processed_args.push(processed_arg);
    }

    processed_args
}

fn add_lua_filters(
    template: &TemplateMapping,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    pandoc: &mut Command,
) -> Result<()> {
    for filter in template.filters.clone().unwrap_or_default() {
        let filter = project_directory_path.join(&filter);

        if !filter.exists() {
            return Err(eyre!(
                "Filter file or directory does not exist: {}",
                filter.display()
            ));
        }

        add_lua_filter_or_directory(
            project_directory_path,
            compiled_directory_path,
            filter,
            pandoc,
        )?;
    }

    Ok(())
}

fn add_lua_filter_or_directory(
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    filter: PathBuf,
    pandoc: &mut Command,
) -> Result<()> {
    if filter.is_dir() {
        for entry in fs::read_dir(filter)? {
            let entry = entry?.path();

            add_lua_filter_or_directory(
                project_directory_path,
                compiled_directory_path,
                entry,
                pandoc,
            )?;
        }
    } else if filter.is_file() && filter.extension().unwrap_or_default() == "lua" {
        pandoc.arg("--lua-filter").arg(
            get_relative_path_from_compiled_dir(
                &filter,
                project_directory_path,
                compiled_directory_path,
            )
            .unwrap_or(filter),
        );
    }

    Ok(())
}

fn get_relative_path_from_compiled_dir(
    original_path: &Path,
    project_root: &Path,
    compiled_dir: &Path,
) -> Option<PathBuf> {
    let relative_to_project = original_path.strip_prefix(project_root).ok()?;

    let depth = compiled_dir
        .strip_prefix(project_root)
        .ok()?
        .components()
        .count();
    let mut relative_path = PathBuf::new();
    for _ in 0..depth {
        relative_path.push("..");
    }

    relative_path.push(relative_to_project);
    Some(relative_path)
}

fn get_sorted_files(
    input_dir: &Path,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
) -> Result<Vec<PathBuf>> {
    let dir_content = fs::read_dir(input_dir)?;

    let mut dir_content = dir_content
        .filter_map(|f| {
            let entry = f.ok()?;

            Some(entry.path())
        })
        .collect::<Vec<_>>();

    dir_content.sort_by(|a, b| {
        let a_num = retrieve_file_order_number(a);
        let b_num = retrieve_file_order_number(b);

        match a_num.cmp(&b_num) {
            std::cmp::Ordering::Equal => {
                let a_is_file = a.is_file();
                let b_is_file = b.is_file();
                match (a_is_file, b_is_file) {
                    (true, false) => std::cmp::Ordering::Less,
                    (false, true) => std::cmp::Ordering::Greater,
                    _ => std::cmp::Ordering::Equal,
                }
            }
            other => other,
        }
    });

    let input_files = dir_content
        .iter()
        .map(|f| {
            if f.is_file() {
                return Ok(vec![f.clone()]);
            }

            get_sorted_files(f, project_directory_path, compiled_directory_path)
        })
        .collect::<Vec<_>>();

    let input_files: Vec<PathBuf> = input_files
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    let input_files = input_files
        .iter()
        .map(|f| {
            get_relative_path_from_compiled_dir(f, project_directory_path, compiled_directory_path)
                .unwrap_or(f.to_path_buf())
        })
        .collect();

    Ok(input_files)
}

fn retrieve_file_order_number(p: &Path) -> u32 {
    let file_name_regex = regex::Regex::new(r".*(\d+).*").unwrap();

    if let Some(order_number) = p
        .file_name()
        .and_then(|name| name.to_str().map(|s| s.to_string()))
        .and_then(|s| file_name_regex.captures(&s).map(|cap| cap[1].to_string()))
        .and_then(|n| match n.parse::<u32>() {
            Result::Ok(n) => Some(n),
            Err(_e) => None,
        })
    {
        return order_number;
    }

    0
}

fn run_with_logging(
    mut command: Command,
    command_name: &str,
    supress_verbose: bool,
) -> Result<String> {
    let mut out = command
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?;

    let Some(stdout) = out.stdout.take() else {
        return Err(eyre!(
            "Failed to capture stdout for command: {}",
            command_name
        ));
    };
    let Some(stderr) = out.stderr.take() else {
        return Err(eyre!(
            "Failed to capture stderr for command: {}",
            command_name
        ));
    };

    let mut stdout_reader = BufReader::new(stdout);
    let mut stderr_reader = BufReader::new(stderr);

    let stdout_thread = thread::spawn(move || {
        let mut buffer = String::new();
        let mut content = String::new();

        while let std::io::Result::Ok(bytes_read) = stdout_reader.read_line(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            content.push_str(&buffer);

            if !supress_verbose {
                debug!("{}", buffer);
            }

            buffer.clear();
        }

        return content;
    });

    let stderr_thread = thread::spawn(move || {
        let mut buffer = String::new();
        while let std::io::Result::Ok(bytes_read) = stderr_reader.read_line(&mut buffer) {
            if bytes_read == 0 {
                break;
            }

            error!("{}", buffer);

            buffer.clear();
        }
    });

    let status = out.wait()?;

    let std::result::Result::Ok(stdout_str) = stdout_thread.join() else {
        return Err(eyre!("Error reading stdout thread"));
    };
    let std::result::Result::Ok(_stderr_str) = stderr_thread.join() else {
        return Err(eyre!("Error reading stderr thread"));
    };

    if !status.success() {
        if command_name != "xelatex" {
            return Err(eyre!(
                "Command {} failed with status code {}.",
                command_name,
                status.code().unwrap()
            ));
        }

        debug!(
            "{} failed with status code {}.",
            command_name,
            status.code().unwrap()
        );
        debug!(
            "Note: For xelatex, this is expected if there are warnings. These are ignored, but genuine errors may be present."
        );
    }

    Ok(stdout_str)
}
