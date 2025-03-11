use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=c_src/fat32.c");
    println!("cargo:rerun-if-changed=c_src/fat32.h");

    // Get output directory from cargo
    let out_dir = std::env::var("OUT_DIR").expect("Failed to get OUT_DIR");

    // build fat32 static library (C)
    let status = std::process::Command::new("make")
        .args(&[format!("BUILD_DIR={}", out_dir)])
        .status()
        .expect("Failed to build libfat32");
    eprintln!("Expected build at: {}", out_dir);
    if !status.success() {
        panic!("Building libfat32 failed with status: {}", status);
    }

    // Tell cargo where to find our library
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=fat32");

    // create bindings for the fat32 library
    let bindings = bindgen::Builder::default()
        .header("c_src/wrapper.h")
        .parse_callbacks(Box::new(bindgen::CargoCallbacks::new()))
        .clang_arg("-mcpu=cortex-a8") // Use your actual target CPU
        .clang_arg("--target=arm-none-eabi")
        .use_core()
        .generate()
        .expect("Unable to generate bindings");
    let out_path = PathBuf::from(env::var("OUT_DIR").unwrap());
    bindings
        .write_to_file(out_path.join("bindings.rs"))
        .expect("Couldn't write bindings!");
}
