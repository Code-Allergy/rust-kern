#![no_std]
#![no_main]

mod hal;
mod panic;
use crate::hal::{dram, uart};

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
    println!("Uart initialized!");
    println!("Bootloader loaded at 0x{:x}", get_boot_entry());

    dram::init();
    println!("DRAM initialized!");
    println!("Time to hang!\n");
    loop {}
}
