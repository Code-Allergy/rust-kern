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
    use crate::bbb::{cm::*, tps::*};
    pub fn init() {
        config_vdd_op_voltage();
        let opp_max_idx = boot_max_opp_get();
        set_vdd10p_voltage(OPP_TABLE[opp_max_idx as usize].volt_sel);
        init_plls();
    }
}
