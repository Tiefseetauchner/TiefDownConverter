use clap::{
    ValueEnum,
    builder::{EnumValueParser, ValueParserFactory},
};
use color_eyre::eyre::{self, Result, eyre};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    path::PathBuf,
    str::FromStr,
    sync::LazyLock,
};
use toml::Table;

use crate::{consts::CURRENT_MANIFEST_VERSION, template_management::get_template_type_from_path};

#[derive(Deserialize, Serialize)]
pub(crate) struct Manifest {
    pub version: u32,
    pub markdown_dir: Option<String>,
    pub templates: Vec<TemplateMapping>,
    pub custom_processors: Processors,
    pub smart_clean: Option<bool>,
    pub smart_clean_threshold: Option<u32>,
    pub profiles: Option<Vec<Profile>>,
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

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum TemplateType {
    Tex = 0,
    Typst = 1,
    Epub = 2,
    CustomPandoc = 3,
}

impl From<&str> for TemplateType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tex" => TemplateType::Tex,
            "typst" => TemplateType::Typst,
            "epub" => TemplateType::Epub,
            "custompandoc" => TemplateType::CustomPandoc,
            _ => panic!("Invalid template type: {}", s),
        }
    }
}

impl FromStr for TemplateType {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "tex" => Ok(TemplateType::Tex),
            "typst" => Ok(TemplateType::Typst),
            "epub" => Ok(TemplateType::Epub),
            "custompandoc" => Ok(TemplateType::CustomPandoc),
            _ => Err(eyre!("Invalid template type: {}", s)),
        }
    }
}

impl From<usize> for TemplateType {
    fn from(value: usize) -> Self {
        match value {
            0 => TemplateType::Tex,
            1 => TemplateType::Typst,
            2 => TemplateType::Epub,
            3 => TemplateType::CustomPandoc,
            _ => panic!("Invalid template type index: {}", value),
        }
    }
}

impl Display for TemplateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let text = match self {
            TemplateType::Tex => "Tex",
            TemplateType::Typst => "Typst",
            TemplateType::Epub => "Epub",
            TemplateType::CustomPandoc => "CustomPandoc",
        };
        write!(f, "{}", text)
    }
}

impl ValueParserFactory for TemplateType {
    type Parser = EnumValueParser<Self>;

    fn value_parser() -> Self::Parser {
        EnumValueParser::new()
    }
}

pub(crate) fn upgrade_manifest(manifest: &mut Table, current_version: u32) -> Result<()> {
    if current_version != CURRENT_MANIFEST_VERSION {
        let mut updated_version = current_version;

        while updated_version < CURRENT_MANIFEST_VERSION {
            if current_version == 0 {
                upgrade_manifest_v0_to_v1(manifest)?
            } else if current_version == 1 {
                upgrade_manifest_v1_to_v2(manifest)?
            } else if current_version == 2 {
                upgrade_manifest_v2_to_v3(manifest)?
            } else {
                return Err(eyre!(
                    "Manifest version {} is not supported for upgrades.",
                    current_version
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

    manifest["custom_processors"]
        .as_table_mut()
        .unwrap()
        .insert("processors".to_string(), toml::Value::Array(Vec::new()));

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
