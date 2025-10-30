use crate::{
    manifest_model::{Injection, MetadataSettings, PreProcessor, PreProcessors, Template},
    template_type::TemplateType,
};
use color_eyre::eyre::{Ok, Result, eyre};
use fast_glob::glob_match;
use log::{debug, error};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::{
    fs,
    io::{BufRead, BufReader},
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
};
use toml::Table;

pub(crate) struct RenderingInjections {
    header_injections: Vec<PathBuf>,
    body_injections: Vec<PathBuf>,
    footer_injections: Vec<PathBuf>,
}

impl RenderingInjections {
    pub fn new() -> RenderingInjections {
        RenderingInjections {
            header_injections: vec![],
            body_injections: vec![],
            footer_injections: vec![],
        }
    }
}

pub(crate) fn retrieve_preprocessors(
    preprocessors: &Option<PreProcessors>,
    custom_preprocessors: &Vec<PreProcessor>,
) -> Vec<PreProcessor> {
    let selected = preprocessors
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
        .unwrap_or(vec![]);
    debug!(
        "retrieve_preprocessors -> {} selected: {:?}",
        selected.len(),
        selected.iter().map(|p| p.name.clone()).collect::<Vec<_>>()
    );
    selected
}

pub(crate) fn merge_preprocessors(preprocessor_lists: Vec<Vec<PreProcessor>>) -> Vec<PreProcessor> {
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
    debug!(
        "merge_preprocessors -> {} merged: {:?}",
        merged.len(),
        merged.iter().map(|p| p.name.clone()).collect::<Vec<_>>()
    );
    merged
}

pub(crate) fn retrieve_combined_output(
    template: &Template,
    default_processors: &Option<PreProcessors>,
) -> Result<Option<PathBuf>> {
    let from_template = template
        .preprocessors
        .clone()
        .and_then(|p| p.combined_output);

    if template.multi_file_output.unwrap_or(false) {
        if from_template.is_some() {
            return Err(eyre!(
                "A template with multi-file output cannot have a preprocessor combined output defined."
            ));
        }

        return Ok(None);
    }

    let from_defaults = default_processors
        .as_ref()
        .and_then(|p| p.clone().combined_output);

    let chosen = from_template.or(from_defaults).ok_or(eyre!(
        "No combined output defined for this template's preprocessor."
    ))?;

    debug!("retrieve_combined_output -> {}", chosen.display());

    Ok(Some(chosen))
}

pub(crate) fn retrieve_output_extension(
    template: &Template,
    default_processors: &Option<PreProcessors>,
) -> Result<String> {
    let from_template = template
        .preprocessors
        .clone()
        .and_then(|p| p.output_extension);

    let from_defaults = default_processors
        .as_ref()
        .and_then(|p| p.clone().output_extension);

    let chosen = from_template.or(from_defaults).ok_or(eyre!(
        "No output extension defined for this template's preprocessor."
    ))?;

    debug!("retrieve_output_extension -> {}", chosen);

    Ok(chosen)
}

pub(crate) fn run_preprocessors_on_inputs(
    template: &Template,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    metadata_fields: &Table,
    _metadata_settings: &MetadataSettings,
    preprocessors: &Vec<PreProcessor>,
    input_files: &Vec<PathBuf>,
    injections: &RenderingInjections,
) -> Result<Vec<String>> {
    let processing_chunks = if template.multi_file_output.unwrap_or(false) {
        get_single_file_preprocessing_chunks(
            &input_files,
            &project_directory_path,
            &compiled_directory_path,
            injections,
        )?
    } else {
        get_preprocessing_chunks(&input_files)?
    };
    debug!("Created {} preprocessing chunks.", processing_chunks.len());

    let results = processing_chunks
        .par_iter()
        .map(|chunk| {
            debug!("Processing chunk with extension {}: {:?}", chunk.1, chunk.0);

            let preprocessor = choose_preprocessor(preprocessors, &chunk.1)?;

            run_preprocessor(
                template,
                project_directory_path,
                compiled_directory_path,
                metadata_fields,
                &preprocessor,
                &chunk.0,
            )
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(results)
}

fn choose_preprocessor(preprocessors: &Vec<PreProcessor>, extension: &str) -> Result<PreProcessor> {
    let preprocessor = preprocessors
        .iter()
        .filter(|p| p.extension_filter.is_some())
        .find(|p| glob_match(p.extension_filter.as_ref().unwrap(), extension))
        .or(preprocessors.iter().find(|p| p.extension_filter.is_none()))
        .ok_or(eyre!(
            "No preprocessor found for files with extension {}",
            extension
        ))?;

    Ok(preprocessor.clone())
}

fn run_preprocessor(
    template: &Template,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    metadata_fields: &toml::map::Map<String, toml::Value>,
    preprocessor: &PreProcessor,
    files: &Vec<PathBuf>,
) -> std::result::Result<String, color_eyre::eyre::Error> {
    debug!(
        "Running preprocessor '{}' on {} files.",
        preprocessor.name,
        files.len()
    );

    let cli_name = preprocessor.cli.clone().unwrap_or("pandoc".to_string());
    let cli_args = preprocess_cli_args(&preprocessor.cli_args, &metadata_fields);

    let mut cli = Command::new(&cli_name);
    cli.args(&cli_args);
    cli.current_dir(compiled_directory_path);

    if template.template_type != TemplateType::CustomProcessor
        && template.template_type != TemplateType::Epub
    {
        add_lua_filters(
            template,
            project_directory_path,
            compiled_directory_path,
            &mut cli,
        )?;
    }
    cli.args(files.clone());
    debug!(
        "Running preprocessor '{}' with args: \"{}\"",
        cli.get_program().to_string_lossy(),
        cli.get_args()
            .into_iter()
            .map(|a| a.to_string_lossy())
            .collect::<Vec<_>>()
            .join("\" \"")
    );
    run_with_logging(cli, &cli_name, true)
}

fn get_single_file_preprocessing_chunks(
    input_files: &Vec<PathBuf>,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    injections: &RenderingInjections,
) -> Result<Vec<(Vec<PathBuf>, String)>> {
    let mut chunks = Vec::new();

    for file in input_files {
        let current_extension = file
            .extension()
            .ok_or(eyre!("Input file {} has no extension", file.display()))?
            .to_owned();

        let mut injected_files = injections
            .header_injections
            .iter()
            .map(|f| {
                get_relative_path_from_compiled_dir(
                    &f,
                    &project_directory_path,
                    &compiled_directory_path,
                )
                .unwrap_or(f.clone())
            })
            .collect::<Vec<_>>();
        injected_files.push(file.clone());
        injected_files.extend(
            injections
                .footer_injections
                .iter()
                .map(|f| {
                    get_relative_path_from_compiled_dir(
                        &f,
                        &project_directory_path,
                        &compiled_directory_path,
                    )
                    .unwrap_or(f.clone())
                })
                .collect::<Vec<_>>(),
        );

        chunks.push((
            injected_files,
            current_extension.to_string_lossy().to_string(),
        ))
    }

    Ok(chunks)
}

fn get_preprocessing_chunks(input_files: &Vec<PathBuf>) -> Result<Vec<(Vec<PathBuf>, String)>> {
    debug!("Chunking {} input files.", input_files.len());

    let mut chunks = Vec::new();
    let mut current_chunk = Vec::new();
    let mut chunk_extension: Option<std::ffi::OsString> = None;

    for input_file in input_files {
        let current_extension = input_file
            .extension()
            .ok_or(eyre!(
                "Input file {} has no extension",
                input_file.display()
            ))?
            .to_owned();

        if current_chunk.is_empty() {
            current_chunk.push(input_file.clone());
            chunk_extension = Some(current_extension);
            continue;
        }

        if Some(&current_extension) != chunk_extension.as_ref() {
            // Push the previous chunk
            if let Some(ext) = &chunk_extension {
                chunks.push((current_chunk, ext.to_string_lossy().to_string()));
            }
            current_chunk = vec![input_file.clone()];
            chunk_extension = Some(current_extension);
        } else {
            current_chunk.push(input_file.clone());
        }
    }

    // Push the last chunk if not empty
    if !current_chunk.is_empty() {
        if let Some(ext) = &chunk_extension {
            chunks.push((current_chunk, ext.to_string_lossy().to_string()));
        }
    }
    debug!(
        "get_preprocessing_chunks -> {} chunks: {:?}",
        chunks.len(),
        chunks
            .iter()
            .map(|(_, ext)| ext.clone())
            .collect::<Vec<_>>()
    );
    Ok(chunks)
}

pub(crate) fn preprocess_cli_args(cli_args: &[String], metadata_fields: &Table) -> Vec<String> {
    let mut processed_args = Vec::new();

    for arg in cli_args.iter() {
        let mut processed_arg = arg.clone();
        for (metadata_key, metadata_value) in metadata_fields.iter() {
            let value = metadata_value.as_str().unwrap_or("");
            processed_arg = processed_arg.replace(&format!("{{{{{}}}}}", metadata_key), value);
        }
        processed_args.push(processed_arg);
    }
    debug!(
        "preprocess_cli_args -> {} args: {:?}",
        processed_args.len(),
        processed_args
    );
    processed_args
}

pub(crate) fn add_lua_filters(
    template: &Template,
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
    debug!("add_lua_filters -> filters processed.");
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
        let rel = get_relative_path_from_compiled_dir(
            &filter,
            project_directory_path,
            compiled_directory_path,
        )
        .unwrap_or(filter.clone());
        debug!("Adding lua filter: {}", rel.display());
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

pub(crate) fn get_relative_path_from_compiled_dir(
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

pub(crate) fn retrieve_injections(
    template: &Template,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    injections: &Vec<Injection>,
) -> Result<RenderingInjections> {
    let header_injections = retrieve_injections_from_manifest(
        &injections,
        project_directory_path,
        compiled_directory_path,
        template.header_injections.clone().unwrap_or(vec![]),
        &template.name,
    )?;
    let body_injections = retrieve_injections_from_manifest(
        &injections,
        project_directory_path,
        compiled_directory_path,
        template.body_injections.clone().unwrap_or(vec![]),
        &template.name,
    )?;
    let footer_injections = retrieve_injections_from_manifest(
        &injections,
        project_directory_path,
        compiled_directory_path,
        template.footer_injections.clone().unwrap_or(vec![]),
        &template.name,
    )?;

    Ok(RenderingInjections {
        header_injections,
        body_injections,
        footer_injections,
    })
}

fn retrieve_injections_from_manifest(
    injections: &Vec<Injection>,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    template_injections: Vec<String>,
    template_name: &String,
) -> Result<Vec<PathBuf>> {
    debug!(
        "Retrieving injections '{}' from Manifest.",
        template_injections.join(",")
    );

    let injections = template_injections
        .iter()
        .map(|n| {
            injections
                .iter()
                .find(|i| i.name == *n)
                .ok_or(eyre!(
                    "Injection '{}' referenced in template '{}' was not found in manifest.",
                    n,
                    template_name
                ))
                .and_then(|i| Ok(i.files.clone()))
        })
        .collect::<Result<Vec<_>>>()?
        .iter()
        .flatten()
        .map(|f| {
            let template_injection_path = compiled_directory_path.join(f);
            debug!(
                "Found injection file '{}'.",
                template_injection_path.display()
            );
            if !template_injection_path.exists() {
                return Err(eyre!(
                    "Injection file '{}' is not a file or directory.",
                    f.display(),
                ));
            }

            Ok(template_injection_path)

            // Ok(get_relative_path_from_compiled_dir(
            //     &template_injection_path,
            //     project_directory_path,
            //     compiled_directory_path,
            // )
            // .unwrap_or(f.to_path_buf()))
        })
        .collect::<Result<Vec<_>>>()?;

    debug!("Retrieved {} injections.", injections.len());

    Ok(injections)
}

pub(crate) fn get_sorted_files(
    input_dir: &Path,
    project_directory_path: &Path,
    compiled_directory_path: &Path,
    injections: &RenderingInjections,
    multi_file_output: bool,
) -> Result<Vec<PathBuf>> {
    let dir_content = fs::read_dir(input_dir)?;

    let mut dir_content = dir_content
        .filter_map(|f| {
            let entry = f.ok()?;

            Some(entry.path())
        })
        .collect::<Vec<_>>();

    dir_content.append(&mut injections.body_injections.clone());

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

    let injected_content: Vec<PathBuf>;

    if multi_file_output {
        injected_content = dir_content
    } else {
        let temp_injected_vec: &mut Vec<PathBuf> = &mut vec![];
        temp_injected_vec.append(&mut injections.header_injections.clone());
        temp_injected_vec.append(&mut dir_content);
        temp_injected_vec.append(&mut injections.footer_injections.clone());

        injected_content = temp_injected_vec.clone();
    };

    let input_files = injected_content
        .iter()
        .map(|f| {
            if f.is_file() {
                return Ok(vec![f.clone()]);
            } else if f.is_dir() {
                get_sorted_files(
                    f,
                    project_directory_path,
                    compiled_directory_path,
                    &RenderingInjections::new(),
                    multi_file_output,
                )
            } else {
                Err(eyre!(
                    "Input file '{}' was not found or does not exist.",
                    f.display()
                ))
            }
        })
        .collect::<Vec<_>>();

    let input_files: Vec<PathBuf> = input_files
        .into_iter()
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .flatten()
        .collect();

    let input_files: Vec<PathBuf> = input_files
        .iter()
        .map(|f| {
            get_relative_path_from_compiled_dir(f, project_directory_path, compiled_directory_path)
                .unwrap_or(f.to_path_buf())
        })
        .collect();
    debug!(
        "get_sorted_files('{}') -> {} files",
        input_dir.display(),
        input_files.len()
    );
    Ok(input_files)
}

fn retrieve_file_order_number(p: &Path) -> u32 {
    let file_name_regex = regex::Regex::new(r".*?(\d+).*").unwrap();

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

pub(crate) fn write_combined_output(
    compiled_directory_path: &Path,
    combined_output: &Path,
    results: &Vec<String>,
) -> Result<()> {
    debug!(
        "Writing combined output to file: {}",
        combined_output.display()
    );

    std::fs::write(
        compiled_directory_path.join(&combined_output),
        results.join("\n\n"),
    )?;
    Ok(())
}

pub(crate) fn write_single_file_outputs(
    project_root: &Path,
    compiled_directory_path: &Path,
    conversion_input_dir: &Path,
    output_path: &Path,
    output_extension: String,
    input_files: &Vec<PathBuf>,
    results: &Vec<String>,
) -> Result<()> {
    debug!(
        "Writing {} files to directory: {}",
        results.len(),
        output_path.display()
    );

    if compiled_directory_path.join(output_path).exists()
        && !compiled_directory_path.join(output_path).is_dir()
    {
        return Err(eyre!(
            "The output path for a multi-file export must be a directory."
        ));
    }

    if !compiled_directory_path.join(output_path).exists() {
        std::fs::create_dir_all(compiled_directory_path.join(output_path))?;
    }

    let results = results.iter().zip(input_files).collect::<Vec<_>>();

    for res in results {
        let relative_conversion_input_dir = get_relative_path_from_compiled_dir(
            conversion_input_dir,
            project_root,
            compiled_directory_path,
        )
        .unwrap();
        let relative_file_name = res.1.clone().with_extension(&output_extension);
        let file_name = relative_file_name.strip_prefix(relative_conversion_input_dir)?;

        if let Some(parent) = file_name.parent() {
            std::fs::create_dir_all(compiled_directory_path.join(output_path).join(parent))?;
        }

        std::fs::write(
            compiled_directory_path.join(output_path).join(file_name),
            res.0,
        )?;
    }

    Ok(())
}

pub(crate) fn combine_pandoc_native(results: Vec<String>) -> String {
    let mut combined = String::new();

    combined.push_str(&format!(
        "[\n{}\n]",
        results
            .iter()
            .map(|r| r.trim()[1..r.trim().len() - 1].trim())
            .filter(|r| !r.is_empty())
            .collect::<Vec<&str>>()
            .join(",\n")
    ));

    combined
}

pub(crate) fn run_with_logging(
    mut command: Command,
    command_name: &str,
    supress_verbose: bool,
) -> Result<String> {
    debug!(
        "Executing command: {} (suppress_verbose={})",
        command_name, supress_verbose
    );
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
    } else {
        debug!("Command {} completed successfully.", command_name);
    }

    Ok(stdout_str)
}
