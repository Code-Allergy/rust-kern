use core::fmt;

// Platform-specific implementation (selected at compile time)
#[cfg(feature = "qemu")]
mod platform {
    pub use crate::hal::qemu::uart::{init, write_byte};
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::uart::{init, write_byte};
}

// Public API
pub fn init() {
    platform::init(); // Platform-specific initialization
}

pub struct Writer;

impl core::fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> core::fmt::Result {
        unsafe {
            for byte in s.bytes() {
                platform::write_byte(byte);
            }
        }
        Ok(())
    }
}
