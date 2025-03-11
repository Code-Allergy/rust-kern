#![no_std]
#![cfg_attr(test, no_main)]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]
pub mod asm;
// register module

// macro implementations
#[macro_use]
pub mod macros;

// component modules
pub mod board;
pub mod ccm;
pub mod dram;
pub mod i2c;
pub mod mmc;
pub mod mmu;
pub mod uart;

// utilities
pub use uart::Writer;
pub mod util;

#[cfg(feature = "bbb")]
pub mod bbb;

#[cfg(feature = "qemu")]
pub mod qemu;

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
fn panic(_info: &PanicInfo) -> ! {
    loop {} // Halt the system on panic
}
