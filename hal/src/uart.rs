use core::fmt::Write;

/// Initialize the UART device (UART0)
pub fn init() {
    platform::init();
    println!("UART0 ACTIVE");
}

pub fn read_byte() -> Option<u8> {
    platform::read_byte()
}

pub struct Writer;
impl Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        for byte in s.bytes() {
            platform::write_byte(byte);
        }
        Ok(())
    }
}

/// UART wrapper that implements rmodem's expected Read and Write traits
pub struct UartDevice;

impl UartDevice {
    pub fn new() -> Self {
        // We're using the already initialized UART
        UartDevice
    }
}

// Create custom error type for no_std
#[derive(Debug)]
pub enum UartError {
    ReadError,
    WriteError,
}

// Platform-specific UART functions
#[cfg(feature = "qemu")]
mod platform {
    pub use crate::qemu::uart::{init, read_byte, write_byte};
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::bbb::uart::{init, write_byte};
    pub fn read_byte() -> Option<u8> {
        None
    }
}
