[![Build Status](https://drone.jojii.de/api/badges/jojii/pkginfo-rs/status.svg)](https://drone.jojii.de/jojii/pkginfo-rs)

# pkginfo-rs
Rust implementation for reading Arch linux packages informations

# Usage
Add following to your Cargo.toml
```toml
pkginfo = "0.1.2"
```

### Example:
```rust
use pkginfo;

fn main() -> Result<(), pkginfo::errors::Error> {
    let pinfo = pkginfo::new("your-package.pkg.tar.xz")?; // pkg.tar.zst is supported too!

    println!("{:#?}", pinfo);

    Ok(())
}
```
