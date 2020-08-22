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
