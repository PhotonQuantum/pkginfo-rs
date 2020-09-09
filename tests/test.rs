const ZST_FILE: &str = "./tests/acpi_call-dkms.pkg.tar.zst";
const XZ_FILE: &str = "./tests/archey3.pkg.tar.xz";

// Read packages
#[test]
fn read_zst_file() {
    let info = pkginfo::new(ZST_FILE);

    assert!(info.is_ok());
}

#[test]
fn read_xz_file() {
    let info = pkginfo::new(XZ_FILE);

    assert!(info.is_ok());
}

// Read value tests

fn setup_info() -> pkginfo::PkgInfo {
    pkginfo::new(ZST_FILE).unwrap()
}

#[test]
fn pkg_name() {
    let info = setup_info();
    assert_eq!(info.pkg_name, "acpi_call-dkms");
}

#[test]
fn pkg_desc() {
    let info = setup_info();
    assert_eq!(info.pkg_desc, "A linux kernel module that enables calls to ACPI methods through /proc/acpi/call - module sources");
}

#[test]
fn pkg_ver() {
    let info = setup_info();
    assert_eq!(info.pkg_ver, "1.1.0-287");
}

#[test]
fn url() {
    let info = setup_info();
    assert!(info.url.is_some());
    assert_eq!(info.url.unwrap(), "https://github.com/mkottman/acpi_call");
}

#[test]
fn depend() {
    let info = setup_info();
    assert_eq!(info.depend, vec!["dkms"]);
}

#[test]
fn make_depend() {
    let info = setup_info();
    assert_eq!(info.make_depend, vec!["linux-headers"]);
}

#[test]
fn provides() {
    let info = setup_info();
    // FIXME
    assert_eq!(info.provides, vec!["acpi_call=1.1.0-287"]);
}

#[test]
fn conflict() {
    let info = setup_info();
    assert_eq!(info.conflict, vec!["acpi_call"]);
}

#[test]
fn size() {
    let info = setup_info();
    assert_eq!(info.size, 29029);
}

#[test]
fn arch() {
    let info = setup_info();
    assert_eq!(info.arch, pkginfo::arch::Architecture::X86_64);
}

#[test]
fn license() {
    let info = setup_info();
    assert!(info.license.is_some());
    // TODO add gpl
    //assert_eq!(info.license, pkginfo::License::GPLv2);
}
