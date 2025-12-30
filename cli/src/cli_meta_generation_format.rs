use clap::{
    ValueEnum,
    builder::{EnumValueParser, ValueParserFactory},
};
use color_eyre::eyre::{self, Result, eyre};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use tiefdownlib::meta_generation_format::MetaGenerationFormat;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum CliMetaGenerationFormat {
    Json = 1,
}

impl From<&str> for CliMetaGenerationFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "json" => CliMetaGenerationFormat::Json,
            _ => panic!("Invalid template type: {}", s),
        }
    }
}

impl FromStr for CliMetaGenerationFormat {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "json" => Ok(CliMetaGenerationFormat::Json),
            _ => Err(eyre!("Invalid template type: {}", s)),
        }
    }
}

impl From<usize> for CliMetaGenerationFormat {
    fn from(value: usize) -> Self {
        match value {
            1 => CliMetaGenerationFormat::Json,
            _ => panic!("Invalid template type index: {}", value),
        }
    }
}

impl From<CliMetaGenerationFormat> for MetaGenerationFormat {
    fn from(value: CliMetaGenerationFormat) -> Self {
        match value {
            CliMetaGenerationFormat::Json => MetaGenerationFormat::Json,
        }
    }
}

impl From<MetaGenerationFormat> for CliMetaGenerationFormat {
    fn from(value: MetaGenerationFormat) -> Self {
        match value {
            MetaGenerationFormat::Json => CliMetaGenerationFormat::Json,
        }
    }
}

impl CliMetaGenerationFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            CliMetaGenerationFormat::Json => "Json",
        }
    }
}

impl Display for CliMetaGenerationFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ValueParserFactory for CliMetaGenerationFormat {
    type Parser = EnumValueParser<Self>;

    fn value_parser() -> Self::Parser {
        EnumValueParser::new()
    }
}
