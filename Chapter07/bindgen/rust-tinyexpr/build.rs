use std::env;
use std::env::var;

use std::path::PathBuf;

const HEADER_FILE_NAME: &'static str = "../tinyexpr/tinyexpr.h";

fn main() {
    let project_dir = var("CARGO_MANIFEST_DIR").unwrap();

    println!("cargo:rustc-link-search={}/../tinyexpr/", project_dir);
    println!("cargo:rustc-link-lib=static=tinyexpr");

    if cfg!(target_env = "msvc") {
        println!("cargo:rustc-link-lib=static=legacy_stdio_definitions");
    }
    
    let bindings = bindgen::Builder::default()
        .header(HEADER_FILE_NAME)
        .generate()
        .expect("Error generating bindings");

    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Error writing bindings");
}