// OUTER BUILD: build.rs
// BUILD SDCARD IMAGE FROM SUBMODULES
use std::env;
use std::io::Write;
use std::path::PathBuf;
use std::process::{Command, Stdio};

const IMG_SIZE_MB: u32 = 100;
const BOOT_PART_SIZE_MB: u32 = 50;

const OUTPUT_IMAGE: &str = "sdimg.img";
const KERNEL_IMG: &str = "test.txt";
const BOOTLOADER_MLO: &str = "bootloader/target/deploy/MLO";

fn build_disk_img() {
    println!("cargo:rerun-if-changed=bootloader/Cargo.toml");
    println!("cargo:rerun-if-changed=bootloader/src");

    let out_dir = PathBuf::from(env::var("OUT_DIR").expect("OUT_DIR not set"));
    let out_file = out_dir.join(OUTPUT_IMAGE);

    eprintln!("Creating main image");
    Command::new("dd")
        .args(&[
            "if=/dev/zero",
            &format!("of={}", out_file.to_str().unwrap()),
            "bs=1M",
            &format!("count={}", IMG_SIZE_MB),
        ])
        .status()
        .expect("Failed to create disk image");

    eprintln!("Creating boot partition image");
    Command::new("dd")
        .args(&[
            "if=/dev/zero",
            &format!("of={}.boot", out_file.to_str().unwrap()),
            "bs=1M",
            &format!("count={}", BOOT_PART_SIZE_MB),
        ])
        .status()
        .expect("Failed to create boot partition image");
    // Partition the disk image
    eprintln!("Partitioning output disk image");
    let mut fdisk = Command::new("fdisk")
        .arg(OUTPUT_IMAGE)
        .stdin(Stdio::piped())
        .spawn()
        .expect("Failed to spawn fdisk");

    let input = format!("o\nn\np\n1\n\n+{}M\nt\nc\na\nw\n", BOOT_PART_SIZE_MB);
    if let Some(stdin) = fdisk.stdin.as_mut() {
        stdin
            .write_all(input.as_bytes())
            .expect("Failed to write to fdisk");
    }
    fdisk.wait().expect("Failed to wait for fdisk");

    eprintln!("Formatting boot partition image as FAT32");
    Command::new("mkfs.vfat")
        .args(&["-F", "32", &format!("{}.boot", out_file.to_str().unwrap())])
        .status()
        .expect("Failed to format boot partition image");
    // panic!("Done");
    // Now use mtools to copy files into this boot partition
    eprintln!("Setting up mtools configuration");
    let mtools_conf =
        "drive c: file=\"".to_string() + &format!("{}.boot", out_file.to_str().unwrap()) + "\"";
    std::fs::write("mtools.conf", mtools_conf).expect("Failed to write mtools configuration");
    unsafe { std::env::set_var("MTOOLSRC", "mtools.conf") };

    eprintln!("Copying MLO file");
    Command::new("mcopy")
        .args(&["-o", BOOTLOADER_MLO, "c:/MLO"])
        .status()
        .expect("Failed to copy MLO file");

    eprintln!("Creating boot folder");
    Command::new("mdir")
        .args(&["c:/boot"])
        .status()
        .expect("Failed to create boot folder");

    eprintln!("Copying kernel image");
    Command::new("mcopy")
        .args(&["-o", KERNEL_IMG, "c:/boot/kernel.bin"])
        .status()
        .expect("Failed to copy kernel image");

    // TODO verify that the start is at 2048, for now assume it is
    let offset = 2048 * 512;
    // Copy the boot partition into the disk image at the right offset
    eprintln!("Copying boot partition into disk image");
    Command::new("dd")
        .args(&[
            &format!("if={}.boot", out_file.to_str().unwrap()),
            &format!("of={}", out_file.to_str().unwrap()),
            "bs=512",
            &format!("seek={}", offset),
            "conv=notrunc",
        ])
        .status()
        .expect("Failed to copy boot partition into disk image");

    // Clean up
    std::fs::remove_file("mtools.conf").ok();
    std::fs::remove_file(&format!("{}.boot", out_file.to_str().unwrap())).ok();
    panic!("Done");
    eprintln!("Disk image created successfully at {}", OUTPUT_IMAGE);
}

fn main() {
    build_disk_img();
}
