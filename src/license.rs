use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum License {
    BSD,
    MIT,
    Apache,
    GPL(GPLVersion),
    LGPL,
    Unsupported(String),
}

#[derive(Debug, PartialEq)]
pub enum GPLVersion {
    V2,
    V3,
    Undefined,
}

impl Default for License {
    fn default() -> Self {
        License::Unsupported("".to_string())
    }
}

impl Display for License {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            License::BSD => "bsd",
            License::MIT => "mit",
            License::Apache => "apache",
            License::GPL(GPLVersion::V2) => "gplv2",
            License::GPL(GPLVersion::V3) => "gplv3",
            License::GPL(GPLVersion::Undefined) => "gpl",
            License::LGPL => "lgpl",
            License::Unsupported(s) => s.as_str()
        })
    }
}

impl License {
    pub fn parse(s: String) -> Self {
        match s.trim().to_lowercase().as_str() {
            "mit" => License::MIT,
            "apache" => License::Apache,
            "bsd" => License::BSD,
            "lgpl" => License::LGPL,
            "gpl" => License::GPL(GPLVersion::Undefined),
            "gplv2" | "gpl2" | "gpl-2" => License::GPL(GPLVersion::V2),
            "gplv3" | "gpl3" | "gpl-3" => License::GPL(GPLVersion::V3),

            other => License::Unsupported(other.to_owned()),
        }
    }
}
