// use crate::println;
/// Initialize the DRAM
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
    pub use crate::hal::bbb::dram::{init_emif, init_vtp};
    pub fn init() {
        // let device_version = get_device_version();
        // config_vdd_op_voltage();

        init_emif();
        init_vtp();
    }
}
