pub fn init() -> Result<(), MMCError> {
    platform::init()?;
    Ok(())
}

pub fn read_sector(sector: u32, buffer: &mut [u8; 512]) -> Result<(), MMCError> {
    platform::read_sector(sector, buffer)?;
    Ok(())
}

#[derive(Debug)]
pub enum MMCError {
    NoResponse,
    Timeout,
    BadCMD8Response,
}

#[cfg(feature = "qemu")]
mod platform {
    pub use crate::qemu::mmc::{init, read_sector};
}

#[cfg(feature = "bbb")]
mod platform {
    pub fn read_sector(sector: u32, buffer: &mut [u8; 512]) {}
    pub use crate::bbb::mmc::init;
}
