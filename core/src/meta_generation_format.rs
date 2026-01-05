use color_eyre::eyre::{self, Result, eyre};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Format of Metadata Generation
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetaGenerationFormat {
    None = 0,
    Json = 1,
}

impl From<&str> for MetaGenerationFormat {
    fn from(s: &str) -> Self {
        match s.to_lowercase().as_str() {
            "none" => MetaGenerationFormat::None,
            "json" => MetaGenerationFormat::Json,
            _ => panic!("Invalid nav meta generation Format: {}", s),
        }
    }
}

impl FromStr for MetaGenerationFormat {
    type Err = eyre::Report;

    fn from_str(s: &str) -> Result<Self> {
        match s.to_lowercase().as_str() {
            "none" => Ok(MetaGenerationFormat::None),
            "json" => Ok(MetaGenerationFormat::Json),
            _ => Err(eyre!("Invalid nav meta generation Format: {}", s)),
        }
    }
}

impl From<usize> for MetaGenerationFormat {
    fn from(value: usize) -> Self {
        match value {
            0 => MetaGenerationFormat::None,
            1 => MetaGenerationFormat::Json,
            _ => panic!("Invalid nav meta generation Format index: {}", value),
        }
    }
}

impl MetaGenerationFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            MetaGenerationFormat::None => "None",
            MetaGenerationFormat::Json => "Json",
        }
    }
}

impl Display for MetaGenerationFormat {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
