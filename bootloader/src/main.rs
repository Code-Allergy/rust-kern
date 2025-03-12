#![no_std]
#![no_main]
#![feature(alloc_error_handler)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

// use alloc::vec;

mod panic;

use core::ffi::c_uchar;
pub use core::ffi::c_void;
use fat32::{Fat32Error, Fat32FileSystem};
use hal::{
    ccm, dbg,
    dram::{self, DRAM_START},
    i2c, mmc, mmu, println,
    uart::{self, read_byte},
};

// // Implementing rmodem compatible Read trait
// impl rmodem::Read for UartDevice {
//     fn read(&mut self, buf: &mut [u8]) -> Result<usize, rmodem::Error> {
//         let mut count = 0;
//         for byte in buf.iter_mut() {
//             match platform::read_byte() {
//                 Some(b) => {
//                     *byte = b;
//                     count += 1;
//                 }
//                 None => break, // No more bytes available
//             }
//         }
//         Ok(count)
//     }
// }

// // Implementing rmodem compatible Write trait
// impl rmodem::Write for UartDevice {
//     fn write(&mut self, buf: &[u8]) -> Result<usize, rmodem::Error> {
//         for &byte in buf {
//             platform::write_byte(byte);
//         }
//         Ok(buf.len())
//     }

//     fn flush(&mut self) -> Result<(), rmodem::Error> {
//         // Assuming the UART doesn't need explicit flushing
//         Ok(())
//     }
// }

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

pub fn get_kernel_entry() -> usize {
    let s = env!("KERNEL_ENTRY");
    usize::from_str_radix(s.trim_start_matches("0x"), 16).unwrap()
}

pub fn get_boot_entry() -> usize {
    unsafe extern "C" {
        static _init: u8;
    }
    let init_addr = unsafe { &_init as *const u8 as usize };
    init_addr
}

#[cfg(feature = "boot_mmc")]
fn boot_mmc() -> ! {
    mmc::init().expect("Failed to initialize MMC");
    println!("Initialized MMC controller");
    copy_kernel_to_phys().expect("Failed to copy kernel to memory");
    println!("Copied kernel, jumping to kernel");
    load_kernel();
}

#[cfg(feature = "boot_uart")]
fn boot_uart() -> ! {
    todo!("do this shit");
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    uart::init();
    i2c::init();
    ccm::init();
    dram::init();
    mmu::init(get_kernel_entry() as u32);
    mmu::enable();

    println!("Finished initializing hardware, enabling MMU");
    println!(
        "Initialization Complete!\nloaded at 0x{:x}",
        get_boot_entry()
    );

    #[cfg(feature = "boot_mmc")]
    boot_mmc();

    #[cfg(feature = "boot_uart")]
    boot_uart();

    #[cfg(not(feature = "boot_uart"))]
    unreachable!("End of bootloader main without jumping!");
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
