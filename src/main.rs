#![no_std]
#![no_main]

mod hal;
mod panic;

use crate::hal::{ccm, dram, i2c, mmc, mmu, uart};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

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

unsafe extern "C" fn read_sector(sector: u32, buffer: *mut u8) -> i32 {
    if buffer.is_null() {
        return -1;
    }

    let buffer_slice: &mut [u8; 512] = unsafe { &mut *(buffer as *mut [u8; 512]) };
    hal::mmc::read_sector(sector, buffer_slice);
    0
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
    panic!("End of BBB main");

    // let mut fat32_fs_t;
    let mut fat32_diskio_t: fat32_diskio_t = fat32_diskio_t {
        read_sector: Some(read_sector),
    };
    let mut fat32_fs_t: fat32_fs_t = unsafe { core::mem::zeroed() };
    let mut fat32_file_t: fat32_file_t = unsafe { core::mem::zeroed() };
    unsafe {
        let filename = "MLO\0";
        let filename_ptr = filename.as_ptr();
        fat32_mount(&mut fat32_fs_t, &mut fat32_diskio_t);
        fat32_open(&mut fat32_fs_t, filename_ptr, &mut fat32_file_t);
        dbg!(fat32_file_t);
    }

    // fat32_diskio_t.read_sector = Some(hal::mmc::read_sector);

    panic!("End of main");
}
