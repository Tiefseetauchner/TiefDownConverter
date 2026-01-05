use clap::{
    ValueEnum,
    builder::{EnumValueParser, ValueParserFactory},
};
use color_eyre::eyre::{self, Result, eyre};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use tiefdownlib::meta_generation_feature::MetaGenerationFeature;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum CliMetaGenerationFeature {
    None = 0,
    Full = 1,
    NavOnly = 2,
    MetadataOnly = 3,
}

impl From<&str> for CliMetaGenerationFeature {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => CliMetaGenerationFeature::None,
            "full" => CliMetaGenerationFeature::Full,
            "navonly" => CliMetaGenerationFeature::NavOnly,
            "metadataonly" => CliMetaGenerationFeature::MetadataOnly,
            _ => panic!("Invalid template type: {}", s),
        }
    }
}

impl FromStr for CliMetaGenerationFeature {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "none" => Ok(CliMetaGenerationFeature::None),
            "full" => Ok(CliMetaGenerationFeature::Full),
            "navonly" => Ok(CliMetaGenerationFeature::NavOnly),
            "metadataonly" => Ok(CliMetaGenerationFeature::MetadataOnly),
            _ => Err(eyre!("Invalid template type: {}", s)),
        }
    }
}

impl From<usize> for CliMetaGenerationFeature {
    fn from(value: usize) -> Self {
        match value {
            0 => CliMetaGenerationFeature::None,
            1 => CliMetaGenerationFeature::Full,
            2 => CliMetaGenerationFeature::NavOnly,
            3 => CliMetaGenerationFeature::MetadataOnly,
            _ => panic!("Invalid template type index: {}", value),
        }
    }
}

impl From<CliMetaGenerationFeature> for MetaGenerationFeature {
    fn from(value: CliMetaGenerationFeature) -> Self {
        match value {
            CliMetaGenerationFeature::None => MetaGenerationFeature::None,
            CliMetaGenerationFeature::Full => MetaGenerationFeature::Full,
            CliMetaGenerationFeature::NavOnly => MetaGenerationFeature::NavOnly,
            CliMetaGenerationFeature::MetadataOnly => MetaGenerationFeature::MetadataOnly,
        }
    }
}

impl From<MetaGenerationFeature> for CliMetaGenerationFeature {
    fn from(value: MetaGenerationFeature) -> Self {
        match value {
            MetaGenerationFeature::None => CliMetaGenerationFeature::None,
            MetaGenerationFeature::Full => CliMetaGenerationFeature::Full,
            MetaGenerationFeature::NavOnly => CliMetaGenerationFeature::NavOnly,
            MetaGenerationFeature::MetadataOnly => CliMetaGenerationFeature::MetadataOnly,
        }
    }
}

impl CliMetaGenerationFeature {
    pub fn as_str(&self) -> &'static str {
        match self {
            CliMetaGenerationFeature::None => "None",
            CliMetaGenerationFeature::Full => "Full",
            CliMetaGenerationFeature::NavOnly => "NavOnly",
            CliMetaGenerationFeature::MetadataOnly => "MetadataOnly",
        }
    }
}

impl Display for CliMetaGenerationFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ValueParserFactory for CliMetaGenerationFeature {
    type Parser = EnumValueParser<Self>;

    fn value_parser() -> Self::Parser {
        EnumValueParser::new()
    }
}
