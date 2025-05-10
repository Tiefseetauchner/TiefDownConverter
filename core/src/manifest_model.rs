use crate::{
    consts::CURRENT_MANIFEST_VERSION, template_management::get_template_type_from_path,
    template_type::TemplateType,
};
use color_eyre::eyre::{Result, eyre};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::LazyLock};
use toml::Table;

#[derive(Deserialize, Serialize)]
pub(crate) struct Manifest {
    pub version: u32,
    pub markdown_projects: Option<Vec<MarkdownProject>>,
    pub templates: Vec<TemplateMapping>,
    pub custom_processors: Processors,
    pub smart_clean: Option<bool>,
    pub smart_clean_threshold: Option<u32>,
    pub shared_metadata: Option<Table>,
    pub metadata_settings: Option<MetadataSettings>,
    pub profiles: Option<Vec<Profile>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct MarkdownProject {
    pub name: String,
    pub path: PathBuf,
    pub output: PathBuf,
    pub metadata_fields: Option<Table>,
    pub default_profile: Option<String>,
    pub resources: Option<Vec<PathBuf>>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Processors {
    pub preprocessors: Vec<PreProcessor>,
    pub processors: Vec<Processor>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct PreProcessor {
    pub name: String,
    pub pandoc_args: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Processor {
    pub name: String,
    pub processor_args: Vec<String>,
}

pub(crate) static DEFAULT_TEX_PREPROCESSOR: LazyLock<PreProcessor> =
    LazyLock::new(|| PreProcessor {
        name: "default_tex_preprocessor".to_string(),
        pandoc_args: vec!["-o", "output.tex", "-t", "latex"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    });

pub(crate) static DEFAULT_TYPST_PREPROCESSOR: LazyLock<PreProcessor> =
    LazyLock::new(|| PreProcessor {
        name: "default_typst_preprocessor".to_string(),
        pandoc_args: vec!["-o", "output.typ", "-t", "typst"]
            .iter()
            .map(|s| s.to_string())
            .collect(),
    });

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct MetadataSettings {
    pub metadata_prefix: Option<String>,
}

impl MetadataSettings {
    pub fn default() -> Self {
        Self {
            metadata_prefix: None,
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct Profile {
    pub name: String,
    pub templates: Vec<String>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct TemplateMapping {
    pub name: String,
    pub template_type: TemplateType,
    pub template_file: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub filters: Option<Vec<String>>,
    pub preprocessor: Option<String>,
    pub processor: Option<String>,
}

pub(crate) fn upgrade_manifest(manifest: &mut Table, current_version: u32) -> Result<()> {
    if current_version != CURRENT_MANIFEST_VERSION {
        let mut updated_version = current_version;

        while updated_version < CURRENT_MANIFEST_VERSION {
            if updated_version == 0 {
                upgrade_manifest_v0_to_v1(manifest)?
            } else if updated_version == 1 {
                upgrade_manifest_v1_to_v2(manifest)?
            } else if updated_version == 2 {
                upgrade_manifest_v2_to_v3(manifest)?
            } else if updated_version == 3 {
                upgrade_manifest_v3_to_v4(manifest)?
            } else {
                return Err(eyre!(
                    "Manifest version {} is not supported for upgrades.",
                    updated_version
                ));
            }

            updated_version += 1;
        }
    }

    Ok(())
}

fn upgrade_manifest_v0_to_v1(manifest: &mut Table) -> Result<()> {
    manifest.insert("version".to_string(), toml::Value::Integer(1));

    if let Some(templates) = manifest.get("templates") {
        manifest.insert(
            "templates".to_string(),
            toml::Value::Array(
                templates
                    .as_array()
                    .unwrap_or(&Vec::new())
                    .iter()
                    .filter(|t| t.is_str())
                    .map(|template| {
                        let template_name = template.as_str().unwrap();
                        let template_type =
                            get_template_type_from_path(template_name).unwrap_or(TemplateType::Tex);
                        let mut table = Table::new();
                        table.insert(
                            "name".to_string(),
                            toml::Value::String(template_name.to_string()),
                        );

                        table.insert(
                            "template_type".to_string(),
                            toml::Value::String(template_type.to_string()),
                        );

                        toml::Value::Table(table)
                    })
                    .collect(),
            ),
        );
    }

    Ok(())
}

fn upgrade_manifest_v1_to_v2(manifest: &mut Table) -> Result<()> {
    manifest.insert("version".to_string(), toml::Value::Integer(2));

    manifest.insert(
        "custom_processors".to_string(),
        toml::Value::Table(Table::new()),
    );
    manifest["custom_processors"]
        .as_table_mut()
        .unwrap()
        .insert("preprocessors".to_string(), toml::Value::Array(Vec::new()));

    Ok(())
}

fn upgrade_manifest_v2_to_v3(manifest: &mut Table) -> Result<()> {
    manifest.insert("version".to_string(), toml::Value::Integer(3));

    manifest.insert(
        "metadata_fields".to_string(),
        toml::Value::Table(Table::new()),
    );
    manifest.insert(
        "metadata_settings".to_string(),
        toml::Value::Table(Table::new()),
    );
    manifest["custom_processors"]
        .as_table_mut()
        .unwrap()
        .insert("processors".to_string(), toml::Value::Array(Vec::new()));

    Ok(())
}

fn upgrade_manifest_v3_to_v4(manifest: &mut Table) -> Result<()> {
    manifest.insert("version".to_string(), toml::Value::Integer(4));

    let metadata_fields = manifest["metadata_fields"].clone();
    if metadata_fields.is_table() && metadata_fields.as_table().unwrap().len() > 0 {
        manifest.insert("shared_metadata".to_string(), metadata_fields);
    }
    manifest.remove("metadata_fields");

    if let Some(markdown_dir) = manifest.get("markdown_dir") {
        if let Some(markdown_dir) = markdown_dir.as_str() {
            let mut markdown_project = Table::new();
            markdown_project.insert(
                "name".to_string(),
                toml::Value::String(markdown_dir.to_string()),
            );
            markdown_project.insert(
                "path".to_string(),
                toml::Value::String(markdown_dir.to_string()),
            );
            markdown_project.insert("output".to_string(), toml::Value::String(".".to_string()));

            manifest.insert(
                "markdown_projects".to_string(),
                toml::Value::Array(vec![toml::Value::Table(markdown_project)]),
            );
        }

        manifest.remove("markdown_dir");
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use rstest::rstest;

    use super::*;

    #[rstest]
    fn test_upgrade_manifest_v0_to_v1() {
        let manifest_content = r#"
markdown_dir = "Custom Markdown Directory"
templates = ["template1.tex", "template2.typ"]"#;
        let mut manifest = toml::from_str(manifest_content).unwrap();

        let result = upgrade_manifest_v0_to_v1(&mut manifest);

        assert!(result.is_ok());

        let expected_manifest = r#"markdown_dir = "Custom Markdown Directory"
version = 1

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

        let actual_manifest = toml::to_string(&manifest).unwrap();
        assert_eq!(expected_manifest, actual_manifest);
    }

    #[rstest]
    fn test_upgrade_manifest_v1_to_v2() {
        let manifest_content = r#"
markdown_dir = "Custom Markdown Directory"
version = 1

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

        let mut manifest = toml::from_str(manifest_content).unwrap();
        let result = upgrade_manifest_v1_to_v2(&mut manifest);
        assert!(result.is_ok());

        let expected_manifest = r#"markdown_dir = "Custom Markdown Directory"
version = 2

[custom_processors]
preprocessors = []

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

        let actual_manifest = toml::to_string(&manifest).unwrap();
        assert_eq!(expected_manifest, actual_manifest);
    }

    #[rstest]
    fn test_upgrade_manifest_v2_to_v3() {
        let manifest_content = r#"markdown_dir = "Custom Markdown Directory"
version = 2

[custom_processors]
preprocessors = []

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

        let mut manifest = toml::from_str(manifest_content).unwrap();
        let result = upgrade_manifest_v2_to_v3(&mut manifest);
        assert!(result.is_ok());

        let expected_manifest = r#"markdown_dir = "Custom Markdown Directory"
version = 3

[custom_processors]
preprocessors = []
processors = []

[metadata_fields]

[metadata_settings]

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

        let actual_manifest = toml::to_string(&manifest).unwrap();
        assert_eq!(expected_manifest, actual_manifest);
    }

    #[rstest]
    fn test_upgrade_manifest_v3_to_v4() {
        let manifest_content = r#"markdown_dir = "Custom Markdown Directory"
version = 3

[custom_processors]
preprocessors = []
processors = []

[metadata_fields]
author = "Author Name"
title = "Document Title"

[metadata_settings]
metadata_prefix = "supermeta"

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

        let mut manifest = toml::from_str(manifest_content).unwrap();
        let result = upgrade_manifest_v3_to_v4(&mut manifest);
        assert!(result.is_ok());

        let expected_manifest = r#"version = 4

[custom_processors]
preprocessors = []
processors = []

[[markdown_projects]]
name = "Custom Markdown Directory"
output = "."
path = "Custom Markdown Directory"

[metadata_settings]
metadata_prefix = "supermeta"

[shared_metadata]
author = "Author Name"
title = "Document Title"

[[templates]]
name = "template1.tex"
template_type = "Tex"

[[templates]]
name = "template2.typ"
template_type = "Typst"
"#;

        let actual_manifest = toml::to_string(&manifest).unwrap();
        assert_eq!(expected_manifest, actual_manifest);
    }
}
