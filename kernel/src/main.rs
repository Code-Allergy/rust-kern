#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

use bootloader_types::BootInfoHeader;
use hal::{dbg, println};

#[unsafe(no_mangle)]
pub extern "C" fn _start(info: &mut BootInfoHeader) -> ! {
    println!("Hello, world!");
    println!("This is aasdfasdf test, sizeasdfs: {:x}", info.boot_size);
    dbg!(info);
    todo!("End of kernel main");
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

#[cfg(not(test))]
mod panic_handler {
    use core::panic::PanicInfo;
    /// Panic handler (required for `no_std`)
    #[panic_handler]
    fn panic(_info: &PanicInfo) -> ! {
        loop {}
    }
}
