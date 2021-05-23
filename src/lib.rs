pub mod arch;
pub mod errors;
pub mod license;

use std::fs::{File, OpenOptions};
use std::io;
use std::io::prelude::*;
use std::path::Path;

use crate::{arch::Architecture, errors::Error, license::License};

use itertools::join;
use tar::Archive;

/// PkgInfo represents the data from an Arch linux package.
#[derive(Debug, Default)]
pub struct PkgInfo {
    pub pkg_name: String,
    pub pkg_base: Option<String>,
    pub pkg_ver: String,
    pub pkg_desc: String,
    pub url: Option<String>,
    pub size: u32,
    pub arch: Architecture,
    pub license: Option<License>,
    pub conflict: Vec<String>,
    pub provides: Vec<String>,
    pub depend: Vec<String>,
    pub opt_depend: Vec<String>,
    pub make_depend: Vec<String>,
}

impl PkgInfo {
    /// Parses a file from a Reader R
    pub fn parse_file<R: Read>(entry: R) -> Result<Self, Error> {
        let reader = io::BufReader::new(entry);
        let mut pkg_info = PkgInfo::default();

        for line in reader.lines() {
            let line = line.map_err(Error::IoError)?;
            let line = line.trim();

            if line.starts_with('#') || !line.contains('=') {
                continue;
            }

            let mut splitted = line.split('=');
            let key = splitted.next().unwrap().trim();
            let value: String = {
                let v: String = join(splitted, "=");
                v.trim().into()
            };

            pkg_info.add_field(key, value);
        }

        Ok(pkg_info)
    }

    /// Adds a field to 'value' depending on the given key
    fn add_field(&mut self, key: &str, value: String) {
        if value.is_empty() {
            return;
        }

        match key {
            "pkgname" => self.pkg_name = value,
            "pkgbase" => self.pkg_base = Some(value),
            "pkgver" => self.pkg_ver = value,
            "pkgdesc" => self.pkg_desc = value,
            "url" => self.url = Some(value),
            "size" => self.size = value.parse().expect("Couldn't parse size"),
            "conflict" => self.conflict.push(value),
            "provides" => self.provides.push(value),
            "depend" => self.depend.push(value),
            "optdepend" => self.opt_depend.push(value),
            "makedepend" => self.make_depend.push(value),
            "arch" => self.arch = Architecture::parse(value),
            "license" => self.license = Some(License::parse(value)),
            _ => {}
        }
    }
}

/// Opens a file and converts the error to an crate::error::Error
fn open_file<P: AsRef<Path>>(path: P) -> Result<File, Error> {
    Ok(OpenOptions::new()
        .read(true)
        .open(&path)
        .map_err(Error::IoError)?)
}

/// Parses a newer zst package
///
/// The `path` specifies the file which is supposed to get parsed
fn new_from_zst<P: AsRef<Path>>(path: P) -> Result<PkgInfo, Error> {
    let mut a = Archive::new(zstd::Decoder::new(open_file(path)?).map_err(Error::IoError)?);

    for file in a.entries().map_err(Error::IoError)? {
        let file = file.map_err(Error::IoError)?;
        if file.header().path().unwrap().to_str().unwrap() == ".PKGINFO" {
            return PkgInfo::parse_file(file);
        }
    }

    Err(Error::InvalidPackageFormat)
}

/// Parses an old xz package. This function is only for backwards compatibility.
/// New packages are in zst format.
///
/// The `path` specifies the file which is supposed to get parsed
pub fn new_from_xz<P: AsRef<Path>>(path: P) -> Result<PkgInfo, Error> {
    let mut a = Archive::new(xz::read::XzDecoder::new(open_file(path)?));

    for file in a.entries().map_err(Error::IoError)? {
        let file = file.map_err(Error::IoError)?;
        if file.header().path().unwrap().to_str().unwrap() == ".PKGINFO" {
            return PkgInfo::parse_file(file);
        }
    }

    Err(Error::InvalidPackageFormat)
}

/// Parses an Arch linux package and automatically chooses the
/// required file format.
pub fn new<P: AsRef<str>>(path: P) -> Result<PkgInfo, Error> {
    if path.as_ref().ends_with(".xz") {
        return new_from_xz(Path::new(path.as_ref()));
    }
    new_from_zst(Path::new(path.as_ref()))
}
