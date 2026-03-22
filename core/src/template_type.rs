use color_eyre::eyre::{self, Result, eyre};
use serde::{Deserialize, Serialize};
use std::{
    fmt::{Display, Formatter},
    str::FromStr,
};

/// The type of a template. Defines the conversion behavior of a template.
///
/// # Examples
///
/// ```
/// use tiefdownlib::template_type::TemplateType;
/// use std::str::FromStr;
///
/// let t = TemplateType::from_str("tex").unwrap();
/// assert_eq!(t, TemplateType::Tex);
/// assert_eq!(t.to_string(), "Tex");
/// ```
#[derive(Deserialize, Serialize, Clone, Copy, Debug, PartialEq, Eq)]
pub enum TemplateType {
    Tex = 0,
    Typst = 1,
    Epub = 2,
    CustomPreprocessors = 3,
    CustomProcessor = 4,
}

impl From<&str> for TemplateType {
    /// Converts a string slice to a `TemplateType`.
    ///
    /// # Panics
    ///
    /// Panics if the string does not match a known template type (case-insensitive).
    ///
    /// # Examples
    ///
    /// ```
    /// use tiefdownlib::template_type::TemplateType;
    ///
    /// assert_eq!(TemplateType::from("typst"), TemplateType::Typst);
    /// assert_eq!(TemplateType::from("Epub"), TemplateType::Epub);
    /// ```
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

    /// Parses a string slice into a `TemplateType`.
    ///
    /// Case-insensitive. Returns an error if the value is not recognized.
    ///
    /// # Examples
    ///
    /// ```
    /// use tiefdownlib::template_type::TemplateType;
    /// use std::str::FromStr;
    ///
    /// assert_eq!(TemplateType::from_str("tex").unwrap(), TemplateType::Tex);
    /// assert_eq!(TemplateType::from_str("CustomProcessor").unwrap(), TemplateType::CustomProcessor);
    /// assert!(TemplateType::from_str("unknown").is_err());
    /// ```
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
    /// Converts a `usize` index to a `TemplateType`.
    ///
    /// # Panics
    ///
    /// Panics if the index is out of range.
    ///
    /// # Examples
    ///
    /// ```
    /// use tiefdownlib::template_type::TemplateType;
    ///
    /// assert_eq!(TemplateType::from(0usize), TemplateType::Tex);
    /// assert_eq!(TemplateType::from(1usize), TemplateType::Typst);
    /// assert_eq!(TemplateType::from(2usize), TemplateType::Epub);
    /// ```
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
    /// Returns the canonical string name of the template type.
    ///
    /// # Examples
    ///
    /// ```
    /// use tiefdownlib::template_type::TemplateType;
    ///
    /// assert_eq!(TemplateType::Tex.as_str(), "Tex");
    /// assert_eq!(TemplateType::Typst.as_str(), "Typst");
    /// assert_eq!(TemplateType::Epub.as_str(), "Epub");
    /// assert_eq!(TemplateType::CustomPreprocessors.as_str(), "CustomPreprocessors");
    /// assert_eq!(TemplateType::CustomProcessor.as_str(), "CustomProcessor");
    /// ```
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
