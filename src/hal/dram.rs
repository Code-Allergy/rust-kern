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
    use crate::{
        dbg,
        hal::bbb::{
            cm::{get_device_version, init_core_pll, init_mpu_pll, init_plls},
            tps::*,
        },
        println,
    };
    pub fn init() {
        // let device_version = get_device_version();
        config_vdd_op_voltage();
        let opp_max_idx = boot_max_opp_get();
        set_vdd10p_voltage(OPP_TABLE[opp_max_idx as usize].volt_sel);
        init_mpu_pll(OPP_TABLE[opp_max_idx as usize].mpupll_m);
        init_core_pll();
        println!("Done MPUPLL");
        // init_plls();

        init_emif();
        init_vtp();
    }
}
