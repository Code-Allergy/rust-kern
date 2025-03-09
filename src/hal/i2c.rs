pub fn init() {
    platform::init(); // Platform-specific initialization
}

// Platform-specific UART functions
#[cfg(feature = "qemu")]
mod platform {
    pub fn init() {}
}

#[cfg(feature = "bbb")]
mod platform {
    use crate::hal::bbb::i2c::{init_clocks, mux_pins};
    pub fn init() {
        init_clocks();
        mux_pins(0);
    }
}
