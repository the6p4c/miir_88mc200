use std::{env, error::Error, fs::File, io::Write, path::PathBuf};

use cc::Build;

fn main() -> Result<(), Box<dyn Error>> {
    let out_dir = PathBuf::from(env::var_os("OUT_DIR").unwrap());

    println!("cargo:rustc-link-search={}", out_dir.display());

    Build::new().file("pre_init.s").compile("asm");
    println!("cargo:rerun-if-changed=pre_init.s");

    Ok(())
}
