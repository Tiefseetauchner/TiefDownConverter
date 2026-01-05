use color_eyre::eyre::{self, Result, eyre};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Featureset of Metadata Generation
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetaGenerationFeature {
    None = 0,
    Full = 1,
    NavOnly = 2,
    MetadataOnly = 3,
}

impl From<&str> for MetaGenerationFeature {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => MetaGenerationFeature::None,
            "full" => MetaGenerationFeature::Full,
            "navonly" => MetaGenerationFeature::NavOnly,
            "metadataonly" => MetaGenerationFeature::MetadataOnly,
            _ => panic!("Invalid nav meta generation feature: {}", s),
        }
    }
}

impl FromStr for MetaGenerationFeature {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "none" => Ok(MetaGenerationFeature::None),
            "full" => Ok(MetaGenerationFeature::Full),
            "navonly" => Ok(MetaGenerationFeature::NavOnly),
            "metadataonly" => Ok(MetaGenerationFeature::MetadataOnly),
            _ => Err(eyre!("Invalid nav meta generation feature: {}", s)),
        }
    }
}

impl From<usize> for MetaGenerationFeature {
    fn from(value: usize) -> Self {
        match value {
            0 => MetaGenerationFeature::None,
            1 => MetaGenerationFeature::Full,
            2 => MetaGenerationFeature::NavOnly,
            3 => MetaGenerationFeature::MetadataOnly,
            _ => panic!("Invalid nav meta generation feature index: {}", value),
        }
    }
}

impl MetaGenerationFeature {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetaGenerationFeature::None => "None",
            MetaGenerationFeature::Full => "Full",
            MetaGenerationFeature::NavOnly => "NavOnly",
            MetaGenerationFeature::MetadataOnly => "MetadataOnly",
        }
    }
}

impl Display for MetaGenerationFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
