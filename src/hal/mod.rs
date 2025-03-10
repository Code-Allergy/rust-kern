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
pub mod mmu;
pub mod uart;

// utilities
pub use uart::Writer;
pub mod util;

#[cfg(feature = "bbb")]
pub mod bbb;

#[cfg(feature = "qemu")]
pub mod qemu;
