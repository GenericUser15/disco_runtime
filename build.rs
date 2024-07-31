use std::{env, error::Error, fs::{self, File}, io::Write, path::PathBuf};

fn main() -> Result<(), Box<dyn Error>> {
    // Build directory for this crate.
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    // Extend the library search path.
    println!("cargo:rustc-link-search={}", out_dir.display());

    // Put `Tlink.x` in the build directory.
    File::create(out_dir.join("Tlink.x"))?.write_all(include_bytes!("Tlink.x"))?;

    // Link to librt.a. This contains the hard fault trampoline.
    fs::copy("librt.a", out_dir.join("librt.a"))?;
    println!("cargo:rustc-link-lib=static=rt");

    // Rebuild is librt.a is changed.
    println!("cargo:rerun-if-changed=librt.a");

    Ok(())
}
