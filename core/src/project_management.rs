use crate::{
    consts::CURRENT_MANIFEST_VERSION,
    manifest_model::{
        Manifest, MarkdownProject, PreProcessor, Processor, Processors, Profile, TemplateMapping,
        upgrade_manifest,
    },
    template_management::{self, add_lix_filters, get_template_path, get_template_type_from_path},
    template_type::TemplateType,
};
use color_eyre::eyre::{Result, eyre};
use fs_extra::dir;
use log::{debug, error, info};
use std::{fs, path::PathBuf, process::Command};
use toml::{Table, Value};

pub fn init(
    project: Option<String>,
    template_names: Option<Vec<String>>,
    no_templates: bool,
    force: bool,
    markdown_dir: Option<String>,
    smart_clean: bool,
    smart_clean_threshold: Option<u32>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);

    if project_path.exists() && force {
        if project == "." {
            return Err(eyre!(
                "Cannot force initialization in the current directory."
            ));
        }
        std::fs::remove_dir_all(project_path)?;
    }

    if !project_path.exists() {
        std::fs::create_dir(project_path)?;
    }

    let manifest_path = project_path.join("manifest.toml");
    if manifest_path.exists() {
        return Err(eyre!(
            "Manifest file already exists. Please remove it before initializing a new project or use the --force flag."
        ));
    }

    let mut templates: Vec<TemplateMapping> = Vec::new();

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

    let markdown_dir_path = project_path.join(&markdown_dir);
    if !markdown_dir_path.exists() {
        std::fs::create_dir(&markdown_dir_path)?;
        std::fs::write(
            markdown_dir_path.join("Chapter 1 - Introduction.md"),
            r#"# Test Document
This is a simple test document for you to edit or overwrite."#,
        )?;
    }

    let smart_clean_value = if smart_clean { Some(true) } else { None };

    create_templates(project_path, &templates)?;

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

    std::fs::write(manifest_path, toml::to_string(&manifest)?)?;

    Ok(())
}

fn get_template_mapping_for_preset(template: &String) -> Result<TemplateMapping> {
    // NOTE: As this is just the preset templates, we set the minimal implementation.
    let mut template = TemplateMapping {
        name: template.clone(),
        template_type: get_template_type_from_path(template)?,
        output: None,
        template_file: None,
        filters: None,
        preprocessor: None,
        processor: None,
    };

    add_lix_filters(&mut template);

    Ok(template)
}

pub fn add_template(
    project: Option<String>,
    template_name: String,
    template_type: Option<TemplateType>,
    template_file: Option<PathBuf>,
    output: Option<PathBuf>,
    filters: Option<Vec<String>>,
    preprocessor: Option<String>,
    processor: Option<String>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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

    let mut template = TemplateMapping {
        name: template_name.clone(),
        template_type,
        output,
        template_file,
        filters,
        preprocessor,
        processor,
    };

    create_templates(project_path, &vec![template.clone()])?;
    add_lix_filters(&mut template);

    manifest.templates.extend([template.clone()]);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn remove_template(project: Option<String>, template_name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    if let Some(pos) = manifest
        .templates
        .iter()
        .position(|t| t.name == template_name)
    {
        let removed_template = manifest.templates.swap_remove(pos);

        let manifest_content = toml::to_string(&manifest)?;
        std::fs::write(&manifest_path, manifest_content)?;

        let template_dir = project_path.join("template");
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
    } else {
        return Err(eyre!(
            "Template {} could not be found in the project.",
            template_name
        ));
    }

    Ok(())
}

pub fn update_template(
    project: Option<String>,
    template_name: String,
    template_type: Option<TemplateType>,
    template_file: Option<PathBuf>,
    output: Option<PathBuf>,
    filters: Option<Vec<String>>,
    add_filters: Option<Vec<String>>,
    remove_filters: Option<Vec<String>>,
    preprocessor: Option<String>,
    processor: Option<String>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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
        template.preprocessor = preprocessor.or(template.preprocessor.clone());
        template.processor = processor.or(template.processor.clone());
    } else {
        return Err(eyre!(
            "Template with name '{}' does not exist.",
            template_name
        ));
    }

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn update_manifest(
    project: Option<String>,
    smart_clean: Option<bool>,
    smart_clean_threshold: Option<u32>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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

pub fn add_preprocessor(
    project: Option<String>,
    name: String,
    pandoc_args: Vec<String>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    let preprocessor = PreProcessor { name, pandoc_args };
    manifest.custom_processors.preprocessors.push(preprocessor);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub fn remove_preprocessor(project: Option<String>, name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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

pub fn add_processor(
    project: Option<String>,
    name: String,
    processor_args: Vec<String>,
) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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

pub fn remove_processor(project: Option<String>, name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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

pub fn list_processors(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let processors = manifest.custom_processors.processors;

    for processor in processors {
        info!("{}: {}", processor.name, processor.processor_args.join(" "));
    }

    Ok(())
}

pub fn add_profile(project: Option<String>, name: String, templates: Vec<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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

pub fn remove_profile(project: Option<String>, name: String) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

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

pub fn list_templates(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let templates = manifest.templates;

    for template in templates {
        info!("{}:", template.name);
        info!("  Template type: {}", &template.template_type);
        if let Some(file) = &template.template_file {
            info!("  Template file: {}", file.display());
        }
        if let Some(output) = &template.output {
            info!("  Output file: {}", output.display());
        }
        if let Some(filters) = &template.filters {
            info!("  Filters: {}", filters.join(", "));
        }
    }

    Ok(())
}

pub fn list_profiles(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let profiles = manifest.profiles;

    for profile in profiles.unwrap_or_default() {
        info!("{}:", profile.name);
        for template in profile.templates {
            info!("  {}", template);
        }
    }

    Ok(())
}

pub fn list_preprocessors(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let preprocessors = manifest.custom_processors.preprocessors;

    for preprocessor in preprocessors {
        info!(
            "{}: {}",
            preprocessor.name,
            preprocessor.pandoc_args.join(" ")
        );
    }

    Ok(())
}

pub fn validate(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let mut errors: Vec<Result<()>> = Vec::new();

    let templates = manifest.templates;

    for template in templates {
        if template.template_type != TemplateType::CustomPandoc {
            let template_path = project_path
                .join("template")
                .join(get_template_path(template.template_file, &template.name));
            if template_path.exists() {
                let template_should_be_dir = match template.template_type {
                    TemplateType::Tex => false,
                    TemplateType::Typst => false,
                    TemplateType::Epub => true,
                    TemplateType::CustomPandoc => false,
                };
                if template_should_be_dir && !template_path.is_dir() {
                    errors.push(Err(eyre!(
                        "Template '{}' is of type 'Epub' but not a directory.",
                        template.name
                    )));
                }

                let template_should_be_file = !template_should_be_dir;
                if template_should_be_file && !template_path.is_file() {
                    errors.push(Err(eyre!(
                        "Template '{}' is of type '{}' but is a directory.",
                        template.name,
                        template.template_type
                    )));
                }
            } else {
                errors.push(Err(eyre!(
                    "Template file '{}' does not exist.",
                    template.name
                )));
            }
        }

        if let Some(filters) = &template.filters {
            for filter in filters {
                if !project_path.join(filter).exists() {
                    errors.push(Err(eyre!("Filter(s) '{}' do not exist.", filter)));
                }
            }
        }
    }

    let markdown_projects = manifest.markdown_projects.unwrap_or(vec![MarkdownProject {
        name: "Default".to_string(),
        path: PathBuf::from("Markdown"),
        output: PathBuf::from("."),
        metadata_fields: None,
        default_profile: None,
        resources: None,
    }]);

    for markdown_project in markdown_projects {
        let markdown_project_path = project_path.join(&markdown_project.path);

        if !markdown_project_path.exists() || !markdown_project_path.is_dir() {
            errors.push(Err(eyre!(
                "Markdown project directory '{}' for project '{}' does not exist.",
                markdown_project.path.display(),
                markdown_project.name
            )));
        }
    }

    debug!("We have {} errors.", errors.len());

    if errors.is_empty() {
        info!("Manifest is valid.");
    } else {
        for error in errors {
            error!("{}", error.unwrap_err());
        }

        return Err(eyre!("Manifest is invalid."));
    }

    Ok(())
}

pub fn clean(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");
    let _ = load_and_convert_manifest(&manifest_path)?;

    run_smart_clean(project_path, 0)?;

    Ok(())
}

pub fn smart_clean(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");
    let manifest = load_and_convert_manifest(&manifest_path)?;
    let smart_clean_threshold = manifest.smart_clean_threshold.unwrap_or(5);

    run_smart_clean(project_path, smart_clean_threshold)?;

    Ok(())
}

pub fn run_smart_clean(project_path: &std::path::Path, smart_clean_threshold: u32) -> Result<()> {
    debug!(
        "Running smart clean on project {} with threshold of {}.",
        project_path.display(),
        smart_clean_threshold
    );

    let regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}$")?;

    let mut dirs_to_delete: Vec<_> = fs::read_dir(project_path)?
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

pub fn check_dependencies(dependencies: Vec<&str>) -> Result<()> {
    let mut errors = Vec::new();

    for dependency in dependencies {
        let output = Command::new(dependency).arg("--version").output();

        if !output.is_ok() {
            errors.push(format!(
                "Could not call {}:\n{}",
                dependency,
                output.unwrap_err()
            ));
        }
    }

    if !errors.is_empty() {
        for error in errors {
            error!("{}", error);
        }
        return Err(eyre!("Some dependencies are missing."));
    }

    info!("All dependencies are installed.");
    Ok(())
}

pub(crate) fn load_and_convert_manifest(manifest_path: &std::path::PathBuf) -> Result<Manifest> {
    if !manifest_path.exists() {
        return Err(eyre!(
            "Manifest file does not exist. Please initialize a project before editing it."
        ));
    }

    let manifest_content = fs::read_to_string(manifest_path)?;

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

    let manifest: Manifest = toml::from_str(manifest)?;

    Ok(manifest)
}

fn create_templates(
    project_path: &std::path::Path,
    templates: &Vec<TemplateMapping>,
) -> Result<()> {
    for template in templates {
        let template_creator = template_management::get_template_creator(template.name.as_str())?;

        template_creator(project_path, template)?;
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn check_dependencies_valid() {
        let dependencies = vec!["ls", "echo"];
        assert!(check_dependencies(dependencies).is_ok());
    }

    #[rstest]
    fn check_dependencies_invalid() {
        let dependencies = vec!["ls", "invalid_command_that_no_sane_person_would_have"];
        assert!(check_dependencies(dependencies).is_err());
    }
}
