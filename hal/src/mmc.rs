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
    Unimplemented,
}

#[cfg(feature = "qemu")]
mod platform {
    pub use crate::qemu::mmc::{init, read_sector};
}

#[cfg(feature = "bbb")]
mod platform {
    use super::MMCError;
    pub use crate::bbb::mmc::init;
    pub fn read_sector(_sector: u32, _buffer: &mut [u8; 512]) -> Result<(), MMCError> {
        todo!()
    }
}
