#![no_std]
#![no_main]

mod hal;
mod panic;

use crate::hal::{asm, ccm, dram, i2c, mmu, uart};

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
    mmu::init();

    // loop through all pages and map them to the same physical address
    let tables = mmu::get_boot_tables();
    for (i, entry) in tables.iter_mut().enumerate() {
        entry.map_section(i as u32 * 0x100000, mmu::L1_ACCESS_RW_RW);
    }
    // dbg!(tables);
    // let first_page = mmu::get_boot_entry_at_virt(0xA0000000);
    // first_page.map_section(dram::DRAM_START as u32, mmu::L1_ACCESS_RW_RW);

    mmu::enable();
    println!("MMU enabled!");
    // try and write to 0xA0000000
    unsafe {
        let test_addr = 0xA0000000 as *mut u32;
        test_addr.write_volatile(0xDEADBEEF);
        println!("Wrote 0xDEADBEEF to 0xA0000000");
        let read_value = test_addr.read_volatile();
        println!("Read 0x{:x} from 0xA0000000", read_value);
    }

    panic!("End of main");
}
