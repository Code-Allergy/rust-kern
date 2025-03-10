pub fn init() {
    platform::init();
}

#[cfg(feature = "qemu")]
mod platform {
    pub use crate::hal::qemu::mmc::init;
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::mmc::init;
}
