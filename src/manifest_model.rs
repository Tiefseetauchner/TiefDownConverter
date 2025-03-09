use clap::{
    builder::{EnumValueParser, PossibleValue, ValueParserFactory},
    ValueEnum,
};
use color_eyre::eyre::{self, eyre, Result};
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, str::FromStr};
use toml::Table;

use crate::consts::CURRENT_MANIFEST_VERSION;

#[derive(Deserialize, Serialize)]
pub(crate) struct Manifest {
    pub version: u32,
    pub markdown_dir: Option<String>,
    pub templates: Vec<TemplateMapping>,
}

#[derive(Deserialize, Serialize, Clone)]
pub(crate) struct TemplateMapping {
    pub name: String,
    pub template_type: TemplateType,
    pub template_file: Option<PathBuf>,
    pub output: Option<PathBuf>,
    pub filters: Option<Vec<String>>,
}

#[derive(Deserialize, Serialize, Clone, Debug, PartialEq, Eq)]
pub(crate) enum TemplateType {
    Tex = 0,
    Typst = 1,
    Epub = 2,
}

impl From<&str> for TemplateType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tex" => TemplateType::Tex,
            "typst" => TemplateType::Typst,
            "epub" => TemplateType::Epub,
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
            _ => panic!("Invalid template type index: {}", value),
        }
    }
}

impl ValueEnum for TemplateType {
    fn to_possible_value(&self) -> Option<PossibleValue> {
        match self {
            TemplateType::Tex => Some(PossibleValue::new("tex")),
            TemplateType::Typst => Some(PossibleValue::new("typst")),
            TemplateType::Epub => Some(PossibleValue::new("epub")),
        }
    }

    fn value_variants<'a>() -> &'a [Self] {
        &[TemplateType::Tex, TemplateType::Typst, TemplateType::Epub]
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
            match current_version {
                0 => {
                    upgrade_manifest_v0_to_v1(manifest)?;
                }
                _ => {}
            }

            updated_version += 1;
        }
    }

    Ok(())
}

fn upgrade_manifest_v0_to_v1(manifest: &mut Table) -> Result<()> {
    manifest["version"] = toml::Value::Integer(1);

    // TODO: Convert from old template format to new template format

    Ok(())
}
