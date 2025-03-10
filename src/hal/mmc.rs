pub fn init() {
    platform::init();
}

pub fn read_sector(sector: u32, buffer: &mut [u8; 512]) {
    platform::read_sector(sector, buffer);
}

#[cfg(feature = "qemu")]
mod platform {
    pub use crate::hal::qemu::mmc::{init, read_sector};
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::mmc::init;
}
