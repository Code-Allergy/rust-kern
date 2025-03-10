#![no_std]
#![no_main]

mod hal;
mod panic;

use crate::hal::{ccm, dram, i2c, mmc, mmu, uart};

fn get_boot_entry() -> usize {
    unsafe extern "C" {
        static __init: u8;
    }
    let init_addr = unsafe { &__init as *const u8 as usize };
    init_addr
}
#[derive(Debug)]
struct BootInfo {
    pub boot_entry: usize,
    pub boot_size: usize,
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart::init();
    i2c::init();
    ccm::init();
    dram::init();
    mmu::init();
    mmc::init();

    println!(
        "Initialization Complete!\nloaded at 0x{:x}",
        get_boot_entry()
    );

    mmu::enable();
    mmu::test_kernel_entry();

    panic!("End of main");
}
