use color_eyre::eyre::{self, Result, eyre};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// The type of a template. Defines the conversion behavior of a template.
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemplateType {
    Tex = 0,
    Typst = 1,
    Epub = 2,
    CustomPreprocessors = 3,
    CustomProcessor = 4,
}

impl From<&str> for TemplateType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tex" => TemplateType::Tex,
            "typst" => TemplateType::Typst,
            "epub" => TemplateType::Epub,
            "custompreprocessors" => TemplateType::CustomPreprocessors,
            "customprocessor" => TemplateType::CustomProcessor,
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
            "custompreprocessors" => Ok(TemplateType::CustomPreprocessors),
            "customprocessor" => Ok(TemplateType::CustomProcessor),
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
            3 => TemplateType::CustomPreprocessors,
            4 => TemplateType::CustomProcessor,
            _ => panic!("Invalid template type index: {}", value),
        }
    }
}

impl TemplateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            TemplateType::Tex => "Tex",
            TemplateType::Typst => "Typst",
            TemplateType::Epub => "Epub",
            TemplateType::CustomPreprocessors => "CustomPreprocessors",
            TemplateType::CustomProcessor => "CustomProcessor",
        }
    }
}

impl Display for TemplateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
