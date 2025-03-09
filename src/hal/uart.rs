use core::fmt::Write;

/// Initialize the UART device (UART0)
pub fn init() {
    platform::init(); // Platform-specific initialization
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

// Platform-specific UART functions
#[cfg(feature = "qemu")]
mod platform {
    pub use crate::hal::qemu::uart::{init, write_byte};
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::uart::{init, write_byte};
}
