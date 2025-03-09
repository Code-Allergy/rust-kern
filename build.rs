use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    println!("cargo:rerun-if-changed=src/boot.S");

    // Get output directory from cargo
    let out_dir = env::var("OUT_DIR").expect("Failed to get OUT_DIR");

    // Specify the object file and archive paths
    let obj_path = Path::new(&out_dir).join("boot.o");
    let lib_path = Path::new(&out_dir).join("libboot.a");

    // Step 1: Assemble the file
    println!("Running arm-none-eabi-gcc to assemble with preprocessing...");
    let status = Command::new("arm-none-eabi-gcc")
        .arg("-c")
        .arg("-x")
        .arg("assembler-with-cpp")
        .arg("-mcpu=cortex-a8")
        .arg("-march=armv7-a")
        .arg("src/boot.S")
        .arg("-o")
        .arg(&obj_path)
        .status()
        .expect("Failed to run arm-none-eabi-gcc");

    if !status.success() {
        panic!("Assembler failed with status: {}", status);
    }

    // Step 2: Create a static library
    println!("Creating static library...");
    let status = Command::new("arm-none-eabi-ar")
        .arg("crs")
        .arg(&lib_path)
        .arg(&obj_path)
        .status()
        .expect("Failed to create static library");

    if !status.success() {
        panic!("Creating library failed with status: {}", status);
    }

    // Tell cargo where to find our library
    println!("cargo:rustc-link-search=native={}", out_dir);
    println!("cargo:rustc-link-lib=static=boot");

    // Force inclusion of __init symbol
    println!("cargo:rustc-link-arg=-u__init");

    // Add necessary linker flags
    println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-nostdlib");
    println!("cargo:rustc-link-arg=-Tlinker.ld");
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=-mcpu=cortex-a8");
    println!("cargo:rustc-link-arg=-march=armv7-a");
}
