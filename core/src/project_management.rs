use crate::{
    consts::CURRENT_MANIFEST_VERSION,
    manifest_model::{
        Manifest, MarkdownProject, PreProcessor, PreProcessors, Processor, Processors, Profile,
        Template, upgrade_manifest,
    },
    template_management::{self, add_lix_filters, get_template_path, get_template_type_from_path},
    template_type::TemplateType,
};
use color_eyre::eyre::{Result, eyre};
use fs_extra::dir;
use log::{debug, error, info};
use std::{env::current_dir, fs, path::PathBuf, process::Command};
use toml::{Table, Value};

/// Initializes a new TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `template_names` - A list of preset template names to initialize the project with.
///   * Defaults to the default TeX template if not provided.
/// * `no_templates` - A flag indicating whether to skip initializing the templates.
/// * `force` - A flag indicating whether to force initialization even if the project already exists.
/// * `markdown_dir` - The path to the markdown directory.
/// * `smart_clean` - A flag indicating whether to enable smart clean.
/// * `smart_clean_threshold` - The threshold for smart clean.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn init(
    project: Option<PathBuf>,
    template_names: Option<Vec<String>>,
    no_templates: bool,
    force: bool,
    markdown_dir: Option<String>,
    smart_clean: bool,
    smart_clean_threshold: Option<u32>,
) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));

    if project.clone().exists() && force {
        if project == current_dir()? {
            return Err(eyre!(
                "Cannot force initialization in the current directory."
            ));
        }
        std::fs::remove_dir_all(project.clone())?;
    }

    if !project.clone().exists() {
        std::fs::create_dir(project.clone())?;
        debug!("Created project directory at '{}'.", project.display());
    }

    let manifest_path = project.join("manifest.toml");
    if manifest_path.exists() {
        return Err(eyre!(
            "Manifest file already exists. Please remove it before initializing a new project or use the --force flag."
        ));
    }

    let mut templates: Vec<Template> = Vec::new();

    if !no_templates {
        templates.extend(
            template_names
                .unwrap_or(vec!["template.tex".to_string()])
                .iter()
                .map(get_template_mapping_for_preset)
                .collect::<Result<Vec<_>>>()?,
        );
    }

    let markdown_dir = markdown_dir.clone().unwrap_or("Markdown".to_string());

    let markdown_dir_path = project.join(&markdown_dir);
    if !markdown_dir_path.exists() {
        std::fs::create_dir(&markdown_dir_path)?;
        std::fs::write(
            markdown_dir_path.join("Chapter 1 - Introduction.md"),
            r#"# Test Document
This is a simple test document for you to edit or overwrite."#,
        )?;
        debug!(
            "Initialized markdown directory at '{}'.",
            markdown_dir_path.display()
        );
    }

    let smart_clean_value = if smart_clean { Some(true) } else { None };

    debug!(
        "Initializing templates ({}): {:?}",
        templates.len(),
        templates.iter().map(|t| t.name.clone()).collect::<Vec<_>>()
    );
    create_templates(&project, &templates)?;

    let manifest: Manifest = Manifest {
        version: CURRENT_MANIFEST_VERSION,
        markdown_projects: Some(vec![MarkdownProject {
            name: markdown_dir.clone(),
            path: PathBuf::from(markdown_dir.clone()),
            output: PathBuf::from("."),
            metadata_fields: None,
            default_profile: None,
            resources: None,
        }]),
        templates: templates.clone(),
        custom_processors: Processors {
            preprocessors: Vec::new(),
            processors: Vec::new(),
        },
        smart_clean: smart_clean_value,
        smart_clean_threshold,
        shared_metadata: None,
        metadata_settings: None,
        profiles: None,
    };

    std::fs::write(manifest_path.clone(), toml::to_string(&manifest)?)?;
    debug!("Wrote manifest to '{}'.", manifest_path.display());

    Ok(())
}

fn get_template_mapping_for_preset(template: &String) -> Result<Template> {
    // NOTE: As this is just the preset templates, we set the minimal implementation.
    let mut template = Template {
        name: template.clone(),
        template_type: get_template_type_from_path(template)?,
        output: None,
        template_file: None,
        filters: None,
        preprocessors: None,
        processor: None,
    };

    add_lix_filters(&mut template);

    Ok(template)
}

/// Adds a new template to the TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `template_name` - The name of the template to add.
/// * `template_type` - The type of the template.
/// * `template_file` - The path to the template file.
/// * `output` - The output file for the template.
/// * `filters` - A list of lua-filters to apply to the template.
///   * Can be either a file or directory.
/// * `preprocessor` - The name of the preprocessor to use.
/// * `processor` - The name of the processor to use.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_template(
    project: Option<PathBuf>,
    template_name: String,
    template_type: Option<TemplateType>,
    template_file: Option<PathBuf>,
    output: Option<PathBuf>,
    filters: Option<Vec<String>>,
    preprocessors: Option<Vec<String>>,
    preprocessor_output: Option<PathBuf>,
    processor: Option<String>,
) -> Result<()> {
    debug!(
        "Adding template '{}' (type: {:?})...",
        template_name, template_type
    );
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if manifest.templates.iter().any(|t| t.name == template_name) {
        return Err(eyre!(
            "Template with name '{}' already exists.",
            template_name
        ));
    }

    let template_type = match template_type {
        Some(t) => t,
        None => {
            get_template_type_from_path(get_template_path(template_file.clone(), &template_name))?
        }
    };

    if preprocessors.is_some() && preprocessor_output.is_none() {
        return Err(eyre!(
            "Cannot set preprocessors without setting a combined output."
        ));
    }

    let mut template_preprocessors = None;
    if preprocessor_output.is_some() {
        template_preprocessors = Some(PreProcessors {
            preprocessors: preprocessors.unwrap_or(vec![]),
            combined_output: PathBuf::from(preprocessor_output.unwrap()),
        });
    }

    let mut template = Template {
        name: template_name.clone(),
        template_type,
        output,
        template_file,
        filters,
        preprocessors: template_preprocessors,
        processor,
    };

    create_templates(&project, &vec![template.clone()])?;
    add_lix_filters(&mut template);

    manifest.templates.extend([template.clone()]);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;
    debug!("Template '{}' added and manifest updated.", template_name);

    Ok(())
}

/// Removes a template from the TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `template_name` - The name of the template to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_template(project: Option<PathBuf>, template_name: String) -> Result<()> {
    debug!("Removing template '{}'...", template_name);
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(pos) = manifest
        .templates
        .iter()
        .position(|t| t.name == template_name)
    {
        let removed_template = manifest.templates.swap_remove(pos);

        let manifest_content = toml::to_string(&manifest)?;
        std::fs::write(&manifest_path, manifest_content)?;

        let template_dir = project.join("template");
        let template_path = template_dir.join(
            removed_template
                .template_file
                .as_ref()
                .unwrap_or(&PathBuf::from(&removed_template.name)),
        );

        if template_path.is_dir() {
            std::fs::remove_dir_all(&template_path)?;
        } else {
            fs::remove_file(template_path)?;
        }
        debug!("Removed template resources for '{}'.", template_name);
    } else {
        return Err(eyre!(
            "Template {} could not be found in the project.",
            template_name
        ));
    }

    Ok(())
}

/// Updates a template in the TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `template_name` - The name of the template to update.
/// * `template_type` - The type of the template.
///   * It is advised to not update template types, but rather create a new template.
/// * `template_file` - The path to the template file.
/// * `output` - The output file for the template.
/// * `filters` - A list of lua-filters to apply to the template.
///   * Can be either a file or directory.
///   * Overwrites existing filters!
/// * `add_filters` - A list of lua-filters to add to the template.
///   * Can be either a file or directory.
/// * `remove_filters` - A list of lua-filters to remove from the template.
///   * Can be either a file or directory.
/// * `preprocessor` - The name of the preprocessor to use.
/// * `processor` - The name of the processor to use.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn update_template(
    project: Option<PathBuf>,
    template_name: String,
    template_type: Option<TemplateType>,
    template_file: Option<PathBuf>,
    output: Option<PathBuf>,
    filters: Option<Vec<String>>,
    add_filters: Option<Vec<String>>,
    remove_filters: Option<Vec<String>>,
    preprocessors: Option<Vec<String>>,
    add_preprocessors: Option<Vec<String>>,
    remove_preprocessors: Option<Vec<String>>,
    preprocessor_output: Option<PathBuf>,
    processor: Option<String>,
) -> Result<()> {
    debug!(
        "Updating template '{}' (fields provided: type={:?}, file={:?}, output={:?})",
        template_name, template_type, template_file, output
    );
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(index) = manifest
        .templates
        .iter()
        .position(|t| t.name == template_name)
    {
        let template = &mut manifest.templates[index];

        template.template_type = template_type.unwrap_or(template.template_type.clone());
        template.output = output.or(template.output.clone());
        template.template_file = template_file.or(template.template_file.clone());
        if let Some(filters) = filters {
            template.filters = Some(filters);
        } else if let Some(add_filters) = add_filters {
            if add_filters.iter().any(|filter| filter.is_empty()) {
                return Err(eyre!(
                    "Cannot add an empty filter to the template '{}'.",
                    template_name
                ));
            }

            if let Some(filters) = &mut template.filters {
                filters.extend(add_filters);
            } else {
                template.filters = Some(add_filters);
            }
        } else if let Some(remove_filters) = remove_filters {
            if remove_filters.iter().any(|filter| filter.is_empty()) {
                return Err(eyre!(
                    "Cannot remove an empty filter from the template '{}'.",
                    template_name
                ));
            }

            if let Some(filters) = &mut template.filters {
                filters.retain(|filter| !remove_filters.contains(filter));
            }
        }

        if let Some(preprocessor_output) = preprocessor_output {
            if let Some(preprocessors) = &mut template.preprocessors {
                preprocessors.combined_output = PathBuf::from(preprocessor_output);
            } else {
                template.preprocessors = Some(PreProcessors {
                    preprocessors: vec![],
                    combined_output: PathBuf::from(preprocessor_output),
                });
            }
        }

        if let Some(preprocessors) = preprocessors {
            if let Some(template_preprocessors) = &mut template.preprocessors {
                template_preprocessors.preprocessors = preprocessors;
            } else {
                return Err(eyre!(
                    "Preprocessor cannot be set as no combined output is set for the template '{}'. Please set a combined output first.",
                    template_name
                ));
            }
        } else if let Some(add_preprocessors) = add_preprocessors {
            if add_preprocessors.iter().any(|filter| {
                manifest
                    .custom_processors
                    .preprocessors
                    .iter()
                    .all(|p| p.name != *filter)
            }) {
                return Err(eyre!(
                    "Preprocessor '{}' cannot be added as it does not exist or is invalid.",
                    template_name
                ));
            }

            if let Some(preprocessors) = &mut template.preprocessors {
                preprocessors.preprocessors.extend(add_preprocessors);
            } else {
                return Err(eyre!(
                    "Preprocessor cannot be set as no combined output is set for the template '{}'. Please set a combined output first.",
                    template_name
                ));
            }
        } else if let Some(remove_preprocessors) = remove_preprocessors {
            if remove_preprocessors.iter().any(|filter| filter.is_empty()) {
                return Err(eyre!(
                    "Cannot remove an empty preprocessor from the template '{}'.",
                    template_name
                ));
            }

            if let Some(preprocessors) = &mut template.preprocessors {
                preprocessors
                    .preprocessors
                    .retain(|filter| !remove_preprocessors.contains(filter));
            }
        }

        template.processor = processor.or(template.processor.clone());
    } else {
        return Err(eyre!(
            "Template with name '{}' does not exist.",
            template_name
        ));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;
    debug!("Template '{}' updated and manifest saved.", template_name);

    Ok(())
}

/// Updates the globally managed settings of a TiefDown project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `smart_clean` - Whether to enable smart cleaning.
/// * `smart_clean_threshold` - The threshold for smart cleaning.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn update_manifest(
    project: Option<PathBuf>,
    smart_clean: Option<bool>,
    smart_clean_threshold: Option<u32>,
) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(smart_clean_value) = smart_clean {
        let smart_clean_value = if smart_clean_value { Some(true) } else { None };
        manifest.smart_clean = smart_clean_value;
    }

    if let Some(smart_clean_threshold) = smart_clean_threshold {
        manifest.smart_clean_threshold = Some(smart_clean_threshold);
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

/// Adds a preprocessor to the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the preprocessor.
/// * `combined_output` - The output file for the preprocessor.
/// * `extension_filter` - The file extension the preprocessor should be applied to.
/// * `cli` - The program to call as the preprocessor.
/// * `cli_args` - The arguments for the preprocessor.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_preprocessor(
    project: Option<PathBuf>,
    name: String,
    extension_filter: Option<String>,
    cli: Option<String>,
    cli_args: Vec<String>,
) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let preprocessor = PreProcessor {
        name,
        extension_filter,
        cli,
        cli_args,
    };
    manifest.custom_processors.preprocessors.push(preprocessor);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

/// Removes a preprocessor from the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the preprocessor to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_preprocessor(project: Option<PathBuf>, name: String) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(pos) = manifest
        .custom_processors
        .preprocessors
        .iter()
        .position(|p| p.name == name)
    {
        manifest.custom_processors.preprocessors.remove(pos);
    } else {
        return Err(eyre!("Preprocessor with name '{}' does not exist.", name));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

/// Adds a processor to the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the processor.
/// * `processor_args` - The arguments for the processor.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_processor(
    project: Option<PathBuf>,
    name: String,
    processor_args: Vec<String>,
) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let processor = Processor {
        name,
        processor_args,
    };
    manifest.custom_processors.processors.push(processor);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

/// Removes a processor from the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the processor to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_processor(project: Option<PathBuf>, name: String) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(pos) = manifest
        .custom_processors
        .processors
        .iter()
        .position(|p| p.name == name)
    {
        manifest.custom_processors.processors.remove(pos);
    } else {
        return Err(eyre!("Processor with name '{}' does not exist.", name));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

/// Retrieves the list of processors from the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
///
/// # Returns
///
/// A Result containing either an error or a vector of Processor objects.
pub fn get_processors(project: Option<PathBuf>) -> Result<Vec<Processor>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    Ok(manifest.custom_processors.processors)
}

/// Adds a profile to the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the profile.
/// * `templates` - A vector of template names.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn add_profile(project: Option<PathBuf>, name: String, templates: Vec<String>) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let profile = Profile { name, templates };

    if manifest.profiles.is_none() {
        manifest.profiles = Some(vec![]);
    }

    manifest.profiles.as_mut().unwrap().push(profile);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

/// Removes a profile from the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
/// * `name` - The name of the profile to remove.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn remove_profile(project: Option<PathBuf>, name: String) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(profiles) = &mut manifest.profiles {
        if let Some(pos) = profiles.iter().position(|p| p.name == name) {
            profiles.remove(pos);
        } else {
            return Err(eyre!("Profile with name '{}' does not exist.", name));
        }
    } else {
        return Err(eyre!("Profile with name '{}' does not exist.", name));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

/// Retrieves the list of templates from the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
///
/// # Returns
///
/// A Result containing either an error or a vector of TemplateMapping objects.
pub fn get_templates(project: Option<PathBuf>) -> Result<Vec<Template>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    Ok(manifest.templates)
}

/// Retrieves the list of profiles from the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
///
/// # Returns
///
/// A Result containing either an error or a vector of Profile objects.
pub fn get_profiles(project: Option<PathBuf>) -> Result<Vec<Profile>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let profiles = manifest.profiles;

    Ok(profiles.unwrap_or_default())
}

/// Retrieves the list of preprocessors from the project's manifest.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
///
/// # Returns
///
/// A Result containing either an error or a vector of PreProcessor objects.
pub fn get_preprocessors(project: Option<PathBuf>) -> Result<Vec<PreProcessor>> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    Ok(manifest.custom_processors.preprocessors)
}

/// Cleans the project's output directories.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn clean(project: Option<PathBuf>) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");
    let _ = load_and_convert_manifest(&manifest_path)?;

    run_smart_clean(&project, 0)?;

    Ok(())
}

/// Runs the smart clean operation on the project.
///
/// # Arguments
///
/// * `project` - The path to the project directory (relative or absolute).
///   * Defaults to the current directory if not provided.
pub fn smart_clean(project: Option<PathBuf>) -> Result<()> {
    let project = project.unwrap_or(PathBuf::from("."));
    let manifest_path = project.join("manifest.toml");
    let manifest = load_and_convert_manifest(&manifest_path)?;
    let smart_clean_threshold = manifest.smart_clean_threshold.unwrap_or(5);

    run_smart_clean(&project, smart_clean_threshold)?;

    Ok(())
}

pub(crate) fn run_smart_clean(project: &PathBuf, smart_clean_threshold: u32) -> Result<()> {
    debug!(
        "Running smart clean on project {} with threshold of {}.",
        project.display(),
        smart_clean_threshold
    );

    let regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}$")?;

    let mut dirs_to_delete: Vec<_> = fs::read_dir(project)?
        .filter_map(Result::ok)
        .map(|entry| entry.path())
        .filter(|path| {
            path.is_dir()
                && path
                    .file_name()
                    .and_then(|name| name.to_str())
                    .map_or(false, |name| regex.is_match(name))
        })
        .collect();

    dirs_to_delete.sort_by(|a, b| a.cmp(b));
    dirs_to_delete.truncate(
        dirs_to_delete
            .len()
            .saturating_sub(smart_clean_threshold as usize),
    );

    for dir in dirs_to_delete {
        debug!("Deleting directory: {}", dir.display());

        dir::remove(dir)?;
    }

    Ok(())
}

/// Checks if the dependencies are installed.
///
/// # Arguments
///
/// * `dependencies` - A vector of strings representing the dependencies to check.
///
/// # Returns
///
/// A Result containing either an error or nothing.
pub fn check_dependencies(dependencies: Vec<&str>) -> Result<()> {
    debug!("Checking dependencies: {:?}", dependencies);
    let errors = get_missing_dependencies(dependencies)?;

    if !errors.is_empty() {
        for error in errors {
            error!("{}", error);
        }
        return Err(eyre!("Some dependencies are missing."));
    }

    info!("All dependencies are installed.");
    Ok(())
}

pub(crate) fn get_missing_dependencies(dependencies: Vec<&str>) -> Result<Vec<String>> {
    let mut errors = Vec::new();

    for dependency in dependencies {
        debug!("Probing dependency: '{}' --version", dependency);
        let output = Command::new(dependency).arg("--version").output();

        if !output.is_ok() {
            errors.push(format!(
                "Could not call {}:\n{}",
                dependency,
                output.unwrap_err()
            ));
        }
    }

    Ok(errors)
}

pub(crate) fn load_and_convert_manifest(manifest_path: &std::path::PathBuf) -> Result<Manifest> {
    if !manifest_path.exists() {
        return Err(eyre!(
            "Manifest file does not exist. Please initialize a project before editing it."
        ));
    }

    let manifest_content = fs::read_to_string(manifest_path)?;
    debug!(
        "Loading manifest from '{}' ({} bytes).",
        manifest_path.display(),
        manifest_content.len()
    );

    let mut manifest: Table = toml::from_str(&manifest_content)?;

    let current_manifest_version: u32 = manifest
        .get("version")
        .unwrap_or(&Value::Integer(0))
        .as_integer()
        .unwrap_or(0)
        .try_into()?;

    if current_manifest_version < CURRENT_MANIFEST_VERSION {
        upgrade_manifest(&mut manifest, current_manifest_version)?;
        debug!("Manifest upgraded to version {}.", CURRENT_MANIFEST_VERSION,);
    } else if current_manifest_version > CURRENT_MANIFEST_VERSION {
        return Err(eyre!(
            "Manifest file is from a newer version of the program. Please update the program."
        ));
    }

    let manifest = &toml::to_string(&manifest)?;
    fs::write(manifest_path, manifest)?;
    debug!(
        "Manifest written back after potential upgrade ({} bytes).",
        manifest.len()
    );

    let manifest: Manifest = toml::from_str(manifest)?;

    Ok(manifest)
}

pub(crate) fn get_project_path(project: Option<PathBuf>) -> Result<PathBuf> {
    let project = project.unwrap_or(PathBuf::from("."));

    if !project.exists() {
        return Err(eyre!("Project path does not exist."));
    }

    Ok(project)
}

fn create_templates(project: &std::path::Path, templates: &Vec<Template>) -> Result<()> {
    for template in templates {
        let template_creator = template_management::get_template_creator(template.name.as_str())?;

        debug!(
            "Creating template '{}' of type {}...",
            template.name, template.template_type
        );
        template_creator(project, template)?;
    }

    Ok(())
}
