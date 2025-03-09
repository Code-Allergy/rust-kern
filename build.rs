use std::env;
use std::path::Path;
use std::process::Command;

fn main() {
    // Detect Cargo features
    let is_bbb = env::var("CARGO_FEATURE_BBB").is_ok();
    let is_qemu = env::var("CARGO_FEATURE_QEMU").is_ok();

    // Ensure only one feature is enabled at a time
    if is_bbb && is_qemu {
        panic!("Cannot enable both 'bbb' and 'qemu' features at the same time.");
    }

    if is_bbb {
        println!("cargo:rustc-cfg=feature=\"bbb\"");
        println!("Building for BeagleBone Black...");
        println!("cargo:rustc-link-arg=-Tlinker_bbb.ld");
    } else if is_qemu {
        println!("cargo:rustc-cfg=feature=\"qemu\"");
        println!("Building for QEMU...");
        println!("cargo:rustc-link-arg=-Tlinker_qemu.ld");
        // Add QEMU-specific build steps here
    } else {
        panic!("Either the 'bbb' or 'qemu' feature must be enabled.");
    }

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
    println!("cargo:rustc-link-arg=-static");
    println!("cargo:rustc-link-arg=-mcpu=cortex-a8");
    println!("cargo:rustc-link-arg=-march=armv7-a");
}
