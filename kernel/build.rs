use std::env;

const KERNEL_LDSCRIPT: &str = "kernel.ld";

fn set_ld_script() {
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").unwrap();
    println!(
        "cargo:rustc-link-arg=-T{}/{}",
        manifest_dir, KERNEL_LDSCRIPT
    );
}

fn main() {
    set_ld_script();
}
