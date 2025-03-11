use std::env;
use std::path::PathBuf;

fn build_c_lib() {
    cc::Build::new()
        .file("c_src/fat32.c")
        .compiler("arm-none-eabi-gcc")
        .warnings(true)
        .extra_warnings(true)
        .warnings_into_errors(true)
        .no_default_flags(true)
        .flag("-c")
        .flag("-g")
        .flag("-O3")
        .flag("-Wpedantic")
        .flag("-nostdlib")
        .flag("-nostartfiles")
        .flag("-ffreestanding")
        .flag("-fno-builtin")
        .flag("-mcpu=cortex-a8")
        .flag("-mfloat-abi=soft")
        .flag("-marm")
        .flag("-ffunction-sections")
        .flag("-fdata-sections")
        .flag("-fno-omit-frame-pointer")
        .include("c_src")
        .compile("fat32");

    println!("cargo:rustc-link-lib=static=fat32");
}

fn build_rust_bindings() {
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

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=c_src/fat32.c");
    println!("cargo:rerun-if-changed=c_src/fat32.h");

    build_c_lib();
    build_rust_bindings();
}
