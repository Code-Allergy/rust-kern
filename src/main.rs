#![no_std]
#![no_main]

mod hal;
mod panic;
use crate::hal::{ccm, dram, i2c, uart};

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

    i2c::init();
    println!("I2C initialized!");
    ccm::init();
    println!("CCM initialized!");
    dram::init();

    let board = hal::board::get_board_info();
    println!("Board Info: {}", board);

    println!("DRAM initialized!");
    // test if the dram works by writing to 0x80000000, then reading it back

    panic!("End of main");
}
