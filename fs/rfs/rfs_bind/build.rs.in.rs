extern crate bindgen;
// use std::fs;
// use std::path;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let base_dir = "${PROJECT_SOURCE_DIR}";
    // let archive_path = format!("{}/build/user_ddriver/libddriver.a", base_dir);
    // if path::Path::new(&archive_path).exists() {
    //     fs::copy(archive_path, format!("{}/rfs_bind/libddriver.a", base_dir))?;
    // }

    // Tell cargo to look for shared libraries in the specified directory
    println!("cargo:rustc-link-search={}/build/user_ddriver", base_dir);

    // Tell cargo to tell rustc to link the system bzip2
    // shared library.
    println!("cargo:rustc-link-lib=ddriver");

    let head_file = format!("{}/user_ddriver/include/ddriver.hpp", base_dir);

    println!("head_file = {}", head_file);

    // Tell cargo to invalidate the built crate whenever the wrapper changes
    println!("cargo:rerun-if-changed={}", head_file);

    // The bindgen::Builder is the main entry point
    // to bindgen, and lets you build up options for
    // the resulting bindings.
    let bindings = bindgen::Builder::default()
        // The input header we would like to generate
        // bindings for.
        .header(head_file)
        // Tell cargo to invalidate the built crate whenever any of the
        // included header files changed.
        .parse_callbacks(Box::new(bindgen::CargoCallbacks))
        // Finish the builder and generate the bindings.
        .generate()
        // Unwrap the Result and panic on failure.
        .expect("Unable to generate bindings");

    let out_path = format!("{}/rfs_bind/bindings.rs", base_dir);
    println!("out_path = {}", out_path);
    bindings
        .write_to_file(out_path)
        .expect("Couldn't write bindings!");
    Ok(())
}
