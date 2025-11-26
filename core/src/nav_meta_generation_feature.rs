use color_eyre::eyre::{self, Result, eyre};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Featureset of Navigation Metadata Generation
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum NavMetaGenerationFeature {
    None = 0,
    Full = 1,
}

impl From<&str> for NavMetaGenerationFeature {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => NavMetaGenerationFeature::None,
            "full" => NavMetaGenerationFeature::Full,
            _ => panic!("Invalid nav meta generation feature: {}", s),
        }
    }
}

impl FromStr for NavMetaGenerationFeature {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "none" => Ok(NavMetaGenerationFeature::None),
            "full" => Ok(NavMetaGenerationFeature::Full),
            _ => Err(eyre!("Invalid nav meta generation feature: {}", s)),
        }
    }
}

impl From<usize> for NavMetaGenerationFeature {
    fn from(value: usize) -> Self {
        match value {
            0 => NavMetaGenerationFeature::None,
            1 => NavMetaGenerationFeature::Full,
            _ => panic!("Invalid nav meta generation feature index: {}", value),
        }
    }
}

impl NavMetaGenerationFeature {
    pub fn as_str(&self) -> &'static str {
        match self {
            NavMetaGenerationFeature::None => "None",
            NavMetaGenerationFeature::Full => "Full",
        }
    }
}

impl Display for NavMetaGenerationFeature {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
