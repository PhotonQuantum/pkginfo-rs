use std::fmt::{Display, Formatter, Result};

#[derive(Debug, PartialEq)]
pub enum Architecture {
    X86_64,
    I686,
    Pentium4,
    Arm,
    Armv7h,
    Armv6h,
    Aarch64,
    Any,
    Unsupported(String),
}

impl Default for Architecture {
    fn default() -> Self {
        Architecture::Unsupported("".to_owned())
    }
}

impl Display for Architecture{
    fn fmt(&self, f: &mut Formatter<'_>) -> Result {
        write!(f, "{}", match self {
            Architecture::X86_64 => "x86_64",
            Architecture::I686 => "i686",
            Architecture::Pentium4 => "pentium4",
            Architecture::Arm => "arm",
            Architecture::Armv7h => "armv7h",
            Architecture::Armv6h => "armv6h",
            Architecture::Aarch64 =>"aarch64",
            Architecture::Any => "any",
            Architecture::Unsupported(s) => s.as_str()
        })
    }
}

impl Architecture {
    pub fn parse(s: String) -> Self {
        match s.trim().to_lowercase().as_str() {
            "i686" => Architecture::I686,
            "pentium4" => Architecture::Pentium4,
            "x86_64" => Architecture::X86_64,
            "arm" => Architecture::Arm,
            "armv7h" => Architecture::Armv7h,
            "armv6h" => Architecture::Armv6h,
            "aarch64" => Architecture::Aarch64,
            "any" => Architecture::Any,
            other => Architecture::Unsupported(other.to_owned()),
        }
    }
}
