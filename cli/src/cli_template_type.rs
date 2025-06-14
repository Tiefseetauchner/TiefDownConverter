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
    CustomPandoc = 3,
}

impl From<&str> for CliTemplateType {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "tex" => CliTemplateType::Tex,
            "typst" => CliTemplateType::Typst,
            "epub" => CliTemplateType::Epub,
            "custompandoc" => CliTemplateType::CustomPandoc,
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
            "custompandoc" => Ok(CliTemplateType::CustomPandoc),
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
            3 => CliTemplateType::CustomPandoc,
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
            CliTemplateType::CustomPandoc => TemplateType::CustomPandoc,
        }
    }
}

impl From<TemplateType> for CliTemplateType {
    fn from(value: TemplateType) -> Self {
        match value {
            TemplateType::Tex => CliTemplateType::Tex,
            TemplateType::Typst => CliTemplateType::Typst,
            TemplateType::Epub => CliTemplateType::Epub,
            TemplateType::CustomPandoc => CliTemplateType::CustomPandoc,
        }
    }
}

impl CliTemplateType {
    pub fn as_str(&self) -> &'static str {
        match self {
            CliTemplateType::Tex => "Tex",
            CliTemplateType::Typst => "Typst",
            CliTemplateType::Epub => "Epub",
            CliTemplateType::CustomPandoc => "CustomPandoc",
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
