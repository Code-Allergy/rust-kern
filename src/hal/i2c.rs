pub fn init() {
    platform::init(); // Platform-specific initialization
}

pub fn mux_pins(channel: u32) {
    platform::mux_pins(channel);
}

// Platform-specific UART functions
#[cfg(feature = "qemu")]
mod platform {
    pub fn init() {}
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::i2c::*;
    pub fn init() {
        init_clocks();
        mux_pins(0);

        master_disable();
        soft_reset();

        auto_idle_disable();
        master_init_clock(I2C_SYSTEM_CLOCK, I2C_INTERNAL_CLOCK, I2C_OUTPUT_CLOCK);

        master_enable();
        while !system_status_ready() {}
    }
}
