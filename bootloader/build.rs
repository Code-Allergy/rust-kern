use std::env;

const BOOT_ASM: &str = "src/boot.S";
const LD_SCRIPT_DIR: &str = "ld_scripts";

enum Platform {
    Bbb,
    Qemu,
}

impl Platform {
    fn from_env() -> Self {
        if env::var("CARGO_FEATURE_BBB").is_ok() {
            Platform::Bbb
        } else if env::var("CARGO_FEATURE_QEMU").is_ok() {
            Platform::Qemu
        } else {
            panic!("Either the 'bbb' or 'qemu' feature must be enabled.");
        }
    }

    fn get_ld_script(&self) -> String {
        match self {
            Platform::Bbb => format!("{}/linker_bbb.ld", LD_SCRIPT_DIR),
            Platform::Qemu => format!("{}/linker_qemu.ld", LD_SCRIPT_DIR),
        }
    }

    fn set_ld_script(&self) {
        let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
        let ld_script = self.get_ld_script();
        println!("cargo:rustc-link-arg=-T{}/{}", manifest_dir, ld_script);
    }

    fn set_kernel_entry_addr(&self) {
        println!("cargo:rerun-if-env-changed=KERNEL_ENTRY");
        println!(
            "cargo:rustc-env=KERNEL_ENTRY={}",
            std::env::var("KERNEL_ENTRY").unwrap_or("0xA0000000".into())
        );
    }

    fn set_features(&self) {
        match self {
            Platform::Bbb => {
                println!("cargo:rustc-cfg=feature=\"bbb\"");
                println!("Building for BeagleBone Black...");
            }
            Platform::Qemu => {
                println!("cargo:rustc-cfg=feature=\"qemu\"");
                println!("Building for QEMU...");
            }
        }
    }
}

fn create_asm_entry_1() {
    cc::Build::new()
        .file(BOOT_ASM)
        .compiler("arm-none-eabi-gcc")
        .extra_warnings(true)
        .warnings_into_errors(true)
        .asm_flag("-c")
        .asm_flag("-x")
        .asm_flag("assembler-with-cpp")
        .asm_flag("-Wa,--fatal-warnings")
        .asm_flag("-falign-functions=4")
        .asm_flag("-falign-jumps=4")
        .asm_flag("-falign-loops=4")
        .asm_flag("-Wa,-adhln=output.lst")
        .asm_flag("-g")
        .compile("boot");
    println!("cargo:rustc-link-lib=static=boot");
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed={}", BOOT_ASM);

    let platform = Platform::from_env();
    platform.set_features();
    platform.set_ld_script();
    platform.set_kernel_entry_addr();

    // set linking flags
    // println!("cargo:rustc-link-arg=-nostartfiles");
    println!("cargo:rustc-link-arg=-nostdlib");

    create_asm_entry_1();
}
