use clap::{
    ValueEnum,
    builder::{EnumValueParser, ValueParserFactory},
};
use color_eyre::eyre::{self, Result, eyre};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};
use tiefdownlib::nav_meta_generation_feature::NavMetaGenerationFeature;

#[derive(Clone, Copy, Debug, PartialEq, Eq, ValueEnum)]
pub(crate) enum CliNavMetaGenerationFeature {
    None = 0,
    Basic = 1,
    Full = 2,
}

impl From<&str> for CliNavMetaGenerationFeature {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => CliNavMetaGenerationFeature::None,
            "basic" => CliNavMetaGenerationFeature::Basic,
            "full" => CliNavMetaGenerationFeature::Full,
            _ => panic!("Invalid template type: {}", s),
        }
    }
}

impl FromStr for CliNavMetaGenerationFeature {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "none" => Ok(CliNavMetaGenerationFeature::None),
            "basic" => Ok(CliNavMetaGenerationFeature::Basic),
            "full" => Ok(CliNavMetaGenerationFeature::Full),
            _ => Err(eyre!("Invalid template type: {}", s)),
        }
    }
}

impl From<usize> for CliNavMetaGenerationFeature {
    fn from(value: usize) -> Self {
        match value {
            0 => CliNavMetaGenerationFeature::None,
            1 => CliNavMetaGenerationFeature::Basic,
            2 => CliNavMetaGenerationFeature::Full,
            _ => panic!("Invalid template type index: {}", value),
        }
    }
}

impl From<CliNavMetaGenerationFeature> for NavMetaGenerationFeature {
    fn from(value: CliNavMetaGenerationFeature) -> Self {
        match value {
            CliNavMetaGenerationFeature::None => NavMetaGenerationFeature::None,
            CliNavMetaGenerationFeature::Basic => NavMetaGenerationFeature::Basic,
            CliNavMetaGenerationFeature::Full => NavMetaGenerationFeature::Full,
        }
    }
}

impl From<NavMetaGenerationFeature> for CliNavMetaGenerationFeature {
    fn from(value: NavMetaGenerationFeature) -> Self {
        match value {
            NavMetaGenerationFeature::None => CliNavMetaGenerationFeature::None,
            NavMetaGenerationFeature::Basic => CliNavMetaGenerationFeature::Basic,
            NavMetaGenerationFeature::Full => CliNavMetaGenerationFeature::Full,
        }
    }
}

impl CliNavMetaGenerationFeature {
    pub fn as_str(&self) -> &'static str {
        match self {
            CliNavMetaGenerationFeature::None => "None",
            CliNavMetaGenerationFeature::Basic => "Basic",
            CliNavMetaGenerationFeature::Full => "Full",
        }
    }
}

impl Display for CliNavMetaGenerationFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl ValueParserFactory for CliNavMetaGenerationFeature {
    type Parser = EnumValueParser<Self>;

    fn value_parser() -> Self::Parser {
        EnumValueParser::new()
    }
}
