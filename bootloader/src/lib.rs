#![no_std]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

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

#[derive(Debug)]
pub struct BootInfoHeader {
    pub boot_entry: usize,
    pub boot_size: usize,
}

#[derive(Debug)]
pub struct KernelHeader {
    pub kernel_entry: usize,
    pub kernel_size: usize,
}
