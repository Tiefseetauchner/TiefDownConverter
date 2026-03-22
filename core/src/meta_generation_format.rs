use color_eyre::eyre::{self, Result, eyre};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// Format of Metadata Generation
///
/// # Examples
///
/// ```
/// use tiefdownlib::meta_generation_format::MetaGenerationFormat;
/// use std::str::FromStr;
///
/// let f = MetaGenerationFormat::from_str("json").unwrap();
/// assert_eq!(f, MetaGenerationFormat::Json);
/// assert_eq!(f.to_string(), "Json");
/// ```
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum MetaGenerationFormat {
    None = 0,
    Json = 1,
}

impl From<&str> for MetaGenerationFormat {
    /// Converts a string slice to a `MetaGenerationFormat`.
    ///
    /// # Panics
    ///
    /// Panics if the string does not match a known variant (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use tiefdownlib::meta_generation_format::MetaGenerationFormat;
    ///
    /// assert_eq!(MetaGenerationFormat::from("json"), MetaGenerationFormat::Json);
    /// assert_eq!(MetaGenerationFormat::from("None"), MetaGenerationFormat::None);
    /// ```
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

    /// Parses a string slice into a `MetaGenerationFormat`.
    ///
    /// Case-insensitive. Returns an error for unrecognized values.
    ///
    /// # Examples
    ///
    /// ```
    /// use tiefdownlib::meta_generation_format::MetaGenerationFormat;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(MetaGenerationFormat::from_str("json").unwrap(), MetaGenerationFormat::Json);
    /// assert!(MetaGenerationFormat::from_str("xml").is_err());
    /// ```
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
    /// Returns the canonical string name of this format variant.
    ///
    /// # Examples
    ///
    /// ```
    /// use tiefdownlib::meta_generation_format::MetaGenerationFormat;
    ///
    /// assert_eq!(MetaGenerationFormat::None.as_str(), "None");
    /// assert_eq!(MetaGenerationFormat::Json.as_str(), "Json");
    /// ```
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
