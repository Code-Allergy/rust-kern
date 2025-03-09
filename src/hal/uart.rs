use core::fmt;

pub trait Uart {
    fn write_byte(&self, byte: u8);
    fn init(&self);
}

// Global writer (no dynamic memory)
pub struct UartWriter {
    _private: (), // Prevent external initialization
}

impl fmt::Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            platform::write_byte(byte); // Platform-specific function
        }
        Ok(())
    }
}

// Platform-specific implementation (selected at compile time)
#[cfg(feature = "qemu")]
mod platform {
    pub use crate::hal::qemu::uart::{init, write_byte};
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::uart::{init, write_byte};
}

// Global instance (initialized at compile time)
static UART: UartWriter = UartWriter { _private: () };

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
