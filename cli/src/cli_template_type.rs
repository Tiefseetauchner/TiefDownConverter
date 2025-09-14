use clap::{
    ValueEnum,
    builder::{EnumValueParser, ValueParserFactory},
};
use color_eyre::eyre::{self, Result, eyre};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use tiefdownlib::template_type::TemplateType;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum CliTemplateType {
    Tex = 0,
    Typst = 1,
    Epub = 2,
    CustomPreprocessors = 3,
    CustomProcessor = 4,
}

impl From<&str> for CliTemplateType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tex" => CliTemplateType::Tex,
            "typst" => CliTemplateType::Typst,
            "epub" => CliTemplateType::Epub,
            "custompreprocessors" => CliTemplateType::CustomPreprocessors,
            "customprocessor" => CliTemplateType::CustomProcessor,
            _ => panic!("Invalid template type: {}", s),
        }
    }
}

impl FromStr for CliTemplateType {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "tex" => Ok(CliTemplateType::Tex),
            "typst" => Ok(CliTemplateType::Typst),
            "epub" => Ok(CliTemplateType::Epub),
            "custompreprocessors" => Ok(CliTemplateType::CustomPreprocessors),
            "customprocessor" => Ok(CliTemplateType::CustomProcessor),
            _ => Err(eyre!("Invalid template type: {}", s)),
        }
    }
}

impl From<usize> for CliTemplateType {
    fn from(value: usize) -> Self {
        match value {
            0 => CliTemplateType::Tex,
            1 => CliTemplateType::Typst,
            2 => CliTemplateType::Epub,
            3 => CliTemplateType::CustomPreprocessors,
            4 => CliTemplateType::CustomProcessor,
            _ => panic!("Invalid template type index: {}", value),
        }
    }
}

impl From<CliTemplateType> for TemplateType {
    fn from(value: CliTemplateType) -> Self {
        match value {
            CliTemplateType::Tex => TemplateType::Tex,
            CliTemplateType::Typst => TemplateType::Typst,
            CliTemplateType::Epub => TemplateType::Epub,
            CliTemplateType::CustomPreprocessors => TemplateType::CustomPreprocessors,
            CliTemplateType::CustomProcessor => TemplateType::CustomProcessor,
        }
    }
}

impl From<TemplateType> for CliTemplateType {
    fn from(value: TemplateType) -> Self {
        match value {
            TemplateType::Tex => CliTemplateType::Tex,
            TemplateType::Typst => CliTemplateType::Typst,
            TemplateType::Epub => CliTemplateType::Epub,
            TemplateType::CustomPreprocessors => CliTemplateType::CustomPreprocessors,
            TemplateType::CustomProcessor => CliTemplateType::CustomProcessor,
        }
    }
}

impl CliTemplateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CliTemplateType::Tex => "Tex",
            CliTemplateType::Typst => "Typst",
            CliTemplateType::Epub => "Epub",
            CliTemplateType::CustomPreprocessors => "CustomPreprocessors",
            CliTemplateType::CustomProcessor => "CustomProcessor",
        }
    }
}

impl Display for CliTemplateType {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ValueParserFactory for CliTemplateType {
    type Parser = EnumValueParser<Self>;

    fn value_parser() -> Self::Parser {
        EnumValueParser::new()
    }
}
