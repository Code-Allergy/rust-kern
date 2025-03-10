pub fn init() {
    platform::init();
}

#[cfg(feature = "qemu")]
mod platform {
    pub fn init() {}
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::mmc::init;
}
