#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

mod panic;

use core::ffi::c_uchar;
pub use core::ffi::c_void;
use fat32::{
    Fat32DiskIO, Fat32Error, Fat32FileSystem,
    raw::{fat32_diskio_t, fat32_file_t, fat32_fs_t, fat32_mount, fat32_open, fat32_read},
};
use hal::{
    ccm, dbg,
    dram::{self, DRAM_START},
    i2c, mmc, mmu, println, uart,
};

use bootloader_types::BootInfoHeader;

unsafe extern "C" fn read_sector(sector: u32, buffer: *mut u8) -> i32 {
    if buffer.is_null() {
        return -1;
    }

    let buffer_slice: &mut [u8; 512] = unsafe { &mut *(buffer as *mut [u8; 512]) };
    hal::mmc::read_sector(sector, buffer_slice).expect("Failed to read sector");
    0
}

fn copy_kernel_to_phys() -> Result<(), Fat32Error> {
    let mut fs = Fat32FileSystem::from_read_fn(read_sector)?;
    let file = fs
        .open_file("/boot/kernel.bin\0")
        .expect("Failed to open kernel.bin");
    let file_size = file.size();
    let start_of_memory = DRAM_START as *mut c_uchar;
    println!("Copying kernel to 0x{:x}", start_of_memory as usize);
    println!("Kernel size: {}", file_size);

    unsafe {
        let memory_slice: &mut [u8] =
            core::slice::from_raw_parts_mut(start_of_memory, file_size as usize);
        file.read(memory_slice)
            .expect("Failed to read file into memory");
    }
    Ok(())
}

#[unsafe(no_mangle)]
pub fn load_kernel() -> ! {
    unsafe {
        let kernel_entry = get_kernel_entry();
        assert!(kernel_entry % 4 == 0, "Kernel must be 4-byte aligned");

        let info = BootInfoHeader {
            boot_entry: get_boot_entry(),
            boot_size: 0xdeadbeef,
        };
        let info_ptr = &info as *const BootInfoHeader as usize;

        // for now, cast kernel_entry with transmutate into a function pointer that takes one argument and never returns
        let kernel_entry = core::mem::transmute::<usize, fn(usize) -> !>(kernel_entry);
        kernel_entry(info_ptr);
    }
}

fn get_kernel_entry() -> usize {
    let s = env!("KERNEL_ENTRY");
    usize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}

fn get_boot_entry() -> usize {
    unsafe extern "C" {
        static _init: u8;
    }
    let init_addr = unsafe { &_init as *const u8 as usize };
    init_addr
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart::init();
    i2c::init();
    ccm::init();
    dram::init();
    mmu::init();

    // for now, map the first 1MB of kernel space to dram
    let first_page = mmu::get_boot_entry_at_virt(get_kernel_entry() as u32);
    first_page.map_section(dram::DRAM_START as u32, mmu::L1_KERNEL_CODE_FLAGS);

    #[cfg(feature = "bbb")]
    todo!("End of rust_main for BBB, need mmc reads");

    #[allow(unreachable_code)] // BBB
    mmc::init().expect("Failed to initialize MMC");

    println!("Finished initializing hardware, enabling MMU");
    println!(
        "Initialization Complete!\nloaded at 0x{:x}",
        get_boot_entry()
    );

    mmu::enable();
    copy_kernel_to_phys().expect("Failed to copy kernel to memory");
    load_kernel();
}

// TODO testing
#[cfg(test)]
fn test_runner(tests: &[&dyn Fn()]) {
    for test in tests {
        test(); // Simply run each test
    }
}

#[cfg(test)]
use core::panic::PanicInfo;

#[cfg(test)]
#[panic_handler]
fn test_panic(_info: &PanicInfo) -> ! {
    loop {} // Halt the system on panic
}
