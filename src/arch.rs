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
