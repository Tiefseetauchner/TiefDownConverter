use std::{fs, path::PathBuf};

use color_eyre::eyre::{Result, eyre};
use toml::{Table, Value};

use crate::{
    consts::CURRENT_MANIFEST_VERSION,
    manifest_model::{Manifest, TemplateMapping, TemplateType, upgrade_manifest},
    template_management::{self, get_template_path, get_template_type_from_path},
};

pub fn init(
    project: Option<String>,
    template_names: Option<Vec<String>>,
    no_templates: bool,
    force: bool,
    markdown_dir: Option<String>,
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

    let markdown_dir_path =
        project_path.join(markdown_dir.clone().unwrap_or("Markdown".to_string()));
    if !markdown_dir_path.exists() {
        std::fs::create_dir(&markdown_dir_path)?;
        std::fs::write(
            markdown_dir_path.join("Chapter 1 - Introduction.md"),
            r#"# Test Document
This is a simple test document for you to edit or overwrite."#,
        )?;
    }

    let manifest: Manifest = Manifest {
        version: CURRENT_MANIFEST_VERSION,
        markdown_dir,
        templates: templates.clone(),
    };

    std::fs::write(manifest_path, toml::to_string(&manifest)?)?;

    create_templates(project_path, &templates)?;

    Ok(())
}

fn get_template_mapping_for_preset(template: &String) -> Result<TemplateMapping> {
    // NOTE: As this is just the preset templates, we set the minimal implementation.
    Ok(TemplateMapping {
        name: template.clone(),
        template_type: get_template_type_from_path(template)?,
        output: None,
        template_file: None,
        filters: None,
    })
}

pub(crate) fn add_template(
    project: Option<String>,
    template_name: String,
    template_type: Option<TemplateType>,
    template_file: Option<PathBuf>,
    output: Option<PathBuf>,
    filters: Option<Vec<String>>,
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

    let template = TemplateMapping {
        name: template_name.clone(),
        template_type,
        output,
        template_file,
        filters,
    };

    manifest.templates.extend([template.clone()]);

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    create_templates(project_path, &vec![template.clone()])?;

    Ok(())
}
pub(crate) fn remove_template(project: Option<String>, template_name: String) -> Result<()> {
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

pub(crate) fn update_template(
    project: Option<String>,
    template_name: String,
    template_type: Option<TemplateType>,
    template_file: Option<PathBuf>,
    output: Option<PathBuf>,
    filters: Option<Vec<String>>,
    add_filters: Option<Vec<String>>,
    remove_filters: Option<Vec<String>>,
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

pub(crate) fn update_manifest(project: Option<String>, markdown_dir: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let mut manifest = load_and_convert_manifest(&manifest_path)?;

    manifest.markdown_dir = markdown_dir;

    let manifest_content = toml::to_string(&manifest)?;
    std::fs::write(&manifest_path, manifest_content)?;

    Ok(())
}

pub(crate) fn list_templates(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let templates = manifest.templates;

    for template in templates {
        println!("{}:", template.name);
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
    }

    Ok(())
}

pub(crate) fn validate(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let manifest_path = project_path.join("manifest.toml");

    let manifest = load_and_convert_manifest(&manifest_path)?;

    let mut errors: Vec<Result<()>> = Vec::new();

    let templates = manifest.templates;
    let markdown_dir = project_path.join(manifest.markdown_dir.unwrap_or("Markdown".to_string()));

    for template in templates {
        let template_path = project_path
            .join("template")
            .join(get_template_path(template.template_file, &template.name));
        if template_path.exists() {
            let template_should_be_dir = match template.template_type {
                TemplateType::Tex => false,
                TemplateType::Typst => false,
                TemplateType::Epub => true,
            };
            if template_should_be_dir && !template_path.is_dir() {
                errors.push(Err(eyre!(
                    "Template '{}' is of type 'Epub' but not a directory.",
                    template.name
                )));
            }

            let template_should_be_file = match template.template_type {
                TemplateType::Tex => true,
                TemplateType::Typst => true,
                TemplateType::Epub => false,
            };
            if template_should_be_file && !template_path.is_file() {
                errors.push(Err(eyre!(
                    "Template '{}' is of type 'Tex' but is a directory.",
                    template.name
                )));
            }
        } else {
            errors.push(Err(eyre!("Template '{}' does not exist.", template.name)));
        }

        if let Some(filters) = &template.filters {
            for filter in filters {
                if !project_path.join(filter).exists() {
                    errors.push(Err(eyre!("Filter(s) '{}' do not exist.", filter)));
                }
            }
        }
    }

    if !markdown_dir.exists() || !markdown_dir.is_dir() {
        errors.push(Err(eyre!(
            "Markdown directory '{}' does not exist.",
            markdown_dir.display()
        )));
    }

    if errors.is_empty() {
        println!("Manifest is valid.");
    } else {
        for error in errors {
            println!("{}", error.unwrap_err());
        }

        return Err(eyre!("Manifest is invalid."));
    }

    Ok(())
}

pub(crate) fn clean(project: Option<String>) -> Result<()> {
    let project = project.as_deref().unwrap_or(".");
    let project_path = std::path::Path::new(&project);
    let regex = regex::Regex::new(r"^\d{4}-\d{2}-\d{2}_\d{2}-\d{2}-\d{2}$")?;

    // Delete all folders matching '2025-03-10_08-40-18'

    let mut files_to_delete = Vec::new();
    for entry in fs::read_dir(project_path)? {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            let dir_name = path.file_name().unwrap().to_str().unwrap();
            if regex.is_match(dir_name) {
                files_to_delete.push(path);
            }
        }
    }

    for file in files_to_delete {
        fs::remove_dir_all(file)?;
    }

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
        println!("Manifest upgraded to version {}.", CURRENT_MANIFEST_VERSION,);
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
