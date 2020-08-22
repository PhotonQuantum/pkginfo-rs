#[derive(Debug)]
pub enum License {
    BSD,
    MIT,
    Apache,
    GPLv3,
    GPLv2,
    LGPL,
    Unsupported(String),
}

impl Default for License {
    fn default() -> Self {
        License::Unsupported("".to_string())
    }
}

impl License {
    pub fn parse(s: String) -> Self {
        match s.trim().to_lowercase().as_str() {
            "mit" => License::MIT,
            "apache" => License::Apache,
            "bsd" => License::BSD,
            "lgpl" => License::LGPL,
            "gplv2" | "gpl2" | "gpl-2" => License::GPLv2,
            "gplv3" | "gpl3" | "gpl-3" => License::GPLv3,

            other => License::Unsupported(other.to_owned()),
        }
    }
}
