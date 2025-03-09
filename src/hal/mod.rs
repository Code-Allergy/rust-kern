pub mod uart;
pub mod util;

pub use uart::Writer;

#[cfg(feature = "bbb")]
pub mod bbb;

#[cfg(feature = "qemu")]
pub mod qemu;
