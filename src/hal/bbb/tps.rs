use crate::hal::util::{reg32_read_masked, reg32_write};

use super::{
    cm::{CONTROL_EFUSE_SMA, CONTROL_MODULE_BASE, get_device_version},
    i2c::{self, I2C_INT_TRANSMIT_READY},
};

pub const MASK_ALL_BITS: u8 = 0xFF;

pub const PASSWORD_LOCK_FOR_WRITE: u8 = 0x00;
pub const PASSWORD_UNLOCK: u8 = 0x7D;
pub const PMIC_TPS65217_I2C_SLAVE_ADDR: u8 = 0x24;

pub const PROT_LEVEL_NONE: u32 = 0x00;
pub const PROT_LEVEL_1: u32 = 0x01;
pub const PROT_LEVEL_2: u32 = 0x02;

// Register offsets
pub const POWER_PATH: u8 = 0x01;
pub const STATUS: u8 = 0x0A;
pub const PASSWORD: u8 = 0x0B;
pub const DEFDCDC1: u8 = 0x0E;
pub const DEFDCDC2: u8 = 0x0F;
pub const DEFDCDC3: u8 = 0x10;

pub const DEFSLEW: u8 = 0x11;
pub const DEFLS1: u8 = 0x14;
pub const DEFLS2: u8 = 0x15;
pub const DCDC_GO: u8 = 0x80;
pub const LDO_VOLTAGE_OUT_3_3: u8 = 0x1F;
pub const LDO_MASK: u8 = 0x1F;

pub const USB_INPUT_CUR_LIMIT_MASK: u8 = 0x03;
pub const USB_INPUT_CUR_LIMIT_100MA: u8 = 0x00;
pub const USB_INPUT_CUR_LIMIT_500MA: u8 = 0x01;
pub const USB_INPUT_CUR_LIMIT_1300MA: u8 = 0x02;
pub const USB_INPUT_CUR_LIMIT_1800MA: u8 = 0x03;

pub const DCDC_VOLT_SEL_1275MV: u8 = 0x0F;
pub const DCDC_VOLT_SEL_1200MV: u8 = 0x0C;
pub const DCDC_VOLT_SEL_1100MV: u8 = 0x08;
pub const DCDC_VOLT_SEL_0950MV: u8 = 0x02;

// efuse
// BOARDINFO board_info;
pub const DEVICE_VERSION_1_0: u32 = 0;
pub const DEVICE_VERSION_2_0: u32 = 1;
pub const DEVICE_VERSION_2_1: u32 = 2;

// EFUSE OPP bit mask
pub const EFUSE_OPP_MASK: u32 = 0x00001FFF;

// EFUSE bit for OPP100 275Mhz - 1.1v
pub const EFUSE_OPP100_275_MASK: u32 = 0x00000001;
pub const EFUSE_OPP100_275: u32 = 0x0;

// EFUSE bit for OPP100 500Mhz - 1.1v
pub const EFUSE_OPP100_500_MASK: u32 = 0x00000002;
pub const EFUSE_OPP100_500: u32 = 0x1;

// EFUSE bit for OPP120 600Mhz - 1.2v
pub const EFUSE_OPP120_600_MASK: u32 = 0x00000004;
pub const EFUSE_OPP120_600: u32 = 0x2;

// EFUSE bit for OPP TURBO 720Mhz - 1.26v
pub const EFUSE_OPPTB_720_MASK: u32 = 0x00000008;
pub const EFUSE_OPPTB_720: u32 = 0x3;

// EFUSE bit for OPP50 300Mhz - 950mv
pub const EFUSE_OPP50_300_MASK: u32 = 0x00000010;
pub const EFUSE_OPP50_300: u32 = 0x4;

// EFUSE bit for OPP100 300Mhz - 1.1v
pub const EFUSE_OPP100_300_MASK: u32 = 0x00000020;
pub const EFUSE_OPP100_300: u32 = 0x5;

// EFUSE bit for OPP100 600Mhz - 1.1v
pub const EFUSE_OPP100_600_MASK: u32 = 0x00000040;
pub const EFUSE_OPP100_600: u32 = 0x6;

// EFUSE bit for OPP120 720Mhz - 1.2v
pub const EFUSE_OPP120_720_MASK: u32 = 0x00000050;
pub const EFUSE_OPP120_720: u32 = 0x7;

// EFUSE bit for OPP TURBO 800Mhz - 1.26v
pub const EFUSE_OPPTB_800_MASK: u32 = 0x00000100;
pub const EFUSE_OPPTB_800: u32 = 0x8;

// EFUSE bit for OPP NITRO 1000Mhz - 1.325v
pub const EFUSE_OPPNT_1000_MASK: u32 = 0x00000200;
pub const EFUSE_OPPNT_1000: u32 = 0x9;

pub const EFUSE_OPP_MAX: u32 = EFUSE_OPPNT_1000 + 1;

/* Types of Opp */
pub const OPP_NONE: u32 = 0;
pub const OPP_50: u32 = 1;
pub const OPP_100: u32 = 2;
pub const OPP_120: u32 = 3;
pub const SR_TURBO: u32 = 4;
pub const OPP_NITRO: u32 = 5;

pub const MPUPLL_M_275_MHZ: u32 = 275;
pub const MPUPLL_M_300_MHZ: u32 = 300;
pub const MPUPLL_M_500_MHZ: u32 = 500;
pub const MPUPLL_M_600_MHZ: u32 = 600;
pub const MPUPLL_M_720_MHZ: u32 = 720;
pub const MPUPLL_M_800_MHZ: u32 = 800;
pub const MPUPLL_M_1000_MHZ: u32 = 1000;

pub const PMIC_VOLT_SEL_0950MV: u8 = DCDC_VOLT_SEL_0950MV;
pub const PMIC_VOLT_SEL_1100MV: u8 = DCDC_VOLT_SEL_1100MV;
pub const PMIC_VOLT_SEL_1200MV: u8 = DCDC_VOLT_SEL_1200MV;
pub const PMIC_VOLT_SEL_1260MV: u8 = DCDC_VOLT_SEL_1275MV;
pub const PMIC_VOLT_SEL_1325MV: u8 = 0x11;

pub struct OppConfig {
    pub mpupll_m: u32,
    pub volt_sel: u8,
}

pub const OPP_TABLE: [OppConfig; 10] = [
    OppConfig {
        mpupll_m: MPUPLL_M_275_MHZ,
        volt_sel: PMIC_VOLT_SEL_1100MV,
    }, // OPP100 275Mhz - 1.1v
    OppConfig {
        mpupll_m: MPUPLL_M_500_MHZ,
        volt_sel: PMIC_VOLT_SEL_1100MV,
    }, // OPP100 500Mhz - 1.1v
    OppConfig {
        mpupll_m: MPUPLL_M_600_MHZ,
        volt_sel: PMIC_VOLT_SEL_1200MV,
    }, // OPP120 600Mhz - 1.2v
    OppConfig {
        mpupll_m: MPUPLL_M_720_MHZ,
        volt_sel: PMIC_VOLT_SEL_1260MV,
    }, // OPP TURBO 720Mhz - 1.26v
    OppConfig {
        mpupll_m: MPUPLL_M_300_MHZ,
        volt_sel: PMIC_VOLT_SEL_0950MV,
    }, // OPP50 300Mhz - 950mv
    OppConfig {
        mpupll_m: MPUPLL_M_300_MHZ,
        volt_sel: PMIC_VOLT_SEL_1100MV,
    }, // OPP100 300Mhz - 1.1v
    OppConfig {
        mpupll_m: MPUPLL_M_600_MHZ,
        volt_sel: PMIC_VOLT_SEL_1100MV,
    }, // OPP100 600Mhz - 1.1v
    OppConfig {
        mpupll_m: MPUPLL_M_720_MHZ,
        volt_sel: PMIC_VOLT_SEL_1200MV,
    }, // OPP120 720Mhz - 1.2v
    OppConfig {
        mpupll_m: MPUPLL_M_800_MHZ,
        volt_sel: PMIC_VOLT_SEL_1260MV,
    }, // OPP TURBO 800Mhz - 1.26v
    OppConfig {
        mpupll_m: MPUPLL_M_1000_MHZ,
        volt_sel: PMIC_VOLT_SEL_1325MV,
    }, // OPP NITRO 1000Mhz - 1.325v
];

pub fn get_opp_config() -> &'static OppConfig {
    let opp_max_idx = boot_max_opp_get();
    &OPP_TABLE[opp_max_idx as usize]
}

fn cleanup_interrupt() {
    i2c::master_int_clear_ex(i2c::I2C_INTERRUPT_FLAG_TO_CLR);
}

fn setup_reception(offset: u8, mut dcount: u32, buffer: &mut [u8]) -> u32 {
    let mut received = 0;

    i2c::set_data_count(1);
    cleanup_interrupt();
    i2c::master_control(i2c::I2C_CFG_MST_TX);
    i2c::master_start();
    while !i2c::master_bus_busy() {}

    i2c::master_data_put(offset);
    i2c::master_int_clear_ex(I2C_INT_TRANSMIT_READY);
    while (i2c::master_int_raw_status() & i2c::I2C_INT_ADRR_READY_ACESS) == 0 {}

    i2c::set_data_count(dcount);
    cleanup_interrupt();
    i2c::master_control(i2c::I2C_CFG_MST_RX);
    i2c::master_start();

    while dcount > 0 {
        while !i2c::master_int_raw_status_ex(i2c::I2C_INT_RECEIVE_READY) {}
        buffer[received] = i2c::master_data_get();
        i2c::master_int_clear_ex(i2c::I2C_INT_RECEIVE_READY);
        dcount -= 1;
        received += 1;
    }

    i2c::master_stop();
    while i2c::master_int_raw_status() & i2c::I2C_INT_STOP_CONDITION == 0 {}
    i2c::master_int_clear_ex(i2c::I2C_INT_STOP_CONDITION);

    received as u32
}

pub fn voltage_update(dc_cntrl_reg: u8, volt_sel: u8) {
    write_reg(PROT_LEVEL_2, dc_cntrl_reg, volt_sel, MASK_ALL_BITS);
    write_reg(PROT_LEVEL_2, DEFSLEW, DCDC_GO, DCDC_GO);
}

pub fn get_opp_data() -> u32 {
    return unsafe { reg32_read_masked(CONTROL_MODULE_BASE, CONTROL_EFUSE_SMA, EFUSE_OPP_MASK) };
}

pub fn boot_max_opp_get() -> u32 {
    let opp_support = get_opp_data();
    let device_version = get_device_version();

    match device_version {
        DEVICE_VERSION_1_0 => EFUSE_OPPTB_720,
        DEVICE_VERSION_2_0 => EFUSE_OPPTB_800,
        DEVICE_VERSION_2_1 => {
            if opp_support & EFUSE_OPPNT_1000_MASK != 0 {
                EFUSE_OPPNT_1000
            } else if opp_support & EFUSE_OPPTB_800_MASK != 0 {
                EFUSE_OPPTB_800
            } else if opp_support & EFUSE_OPP120_720_MASK != 0 {
                EFUSE_OPP120_720
            } else if opp_support & EFUSE_OPP100_600_MASK != 0 {
                EFUSE_OPP100_600
            } else if opp_support & EFUSE_OPP100_300_MASK != 0 {
                EFUSE_OPP100_300
            } else {
                EFUSE_OPP50_300
            }
        }
        _ => return OPP_NONE,
    }
}

pub fn set_vdd10p_voltage(vol_selector: u8) {
    voltage_update(DEFDCDC1, vol_selector);
}

pub fn config_vdd_op_voltage() {
    let mut pmic = 0;

    i2c::master_slave_addr_set(PMIC_TPS65217_I2C_SLAVE_ADDR);
    let pmic_status = read_reg(STATUS);
    dbg!(pmic_status);

    // set usb current limit to 1300mA
    write_reg(
        PROT_LEVEL_NONE,
        POWER_PATH,
        USB_INPUT_CUR_LIMIT_1300MA,
        USB_INPUT_CUR_LIMIT_MASK,
    );
    voltage_update(DEFDCDC2, DCDC_VOLT_SEL_1275MV);
    write_reg(PROT_LEVEL_2, DEFLS1, LDO_VOLTAGE_OUT_3_3, LDO_MASK);
    write_reg(PROT_LEVEL_2, DEFLS2, LDO_VOLTAGE_OUT_3_3, LDO_MASK);
    println!("Done!");
}

pub fn read_reg(offset: u8) -> u8 {
    let mut buffer = [0u8; 1];
    setup_reception(offset, 1, &mut buffer);
    buffer[0]
}

pub fn write_reg(port_level: u32, offset: u8, mut dest_val: u8, mask: u8) {
    let mut buffer = [0u8; 1];
    let mut xor_reg = 0;

    if mask != MASK_ALL_BITS {
        let count = setup_reception(offset, 1, &mut buffer);
        let mut received = buffer[0];
        received &= !mask;
        received |= dest_val & mask;
        dest_val = received;
    }

    if port_level > 0 {
        xor_reg = offset ^ PASSWORD_UNLOCK;
        let mut buffer = [0u8; 2];
        buffer[0] = PASSWORD;
        buffer[1] = xor_reg;
        i2c::device_write(PMIC_TPS65217_I2C_SLAVE_ADDR, &buffer);
    }

    let mut buffer = [0u8; 2];
    buffer[0] = offset;
    buffer[1] = dest_val as u8;
    i2c::device_write(PMIC_TPS65217_I2C_SLAVE_ADDR, &buffer);

    if port_level == PROT_LEVEL_2 {
        let mut buffer = [0u8; 2];
        buffer[0] = PASSWORD;
        buffer[1] = xor_reg;
        i2c::device_write(PMIC_TPS65217_I2C_SLAVE_ADDR, &buffer);

        let mut buffer = [0u8; 2];
        buffer[0] = offset;
        buffer[1] = dest_val as u8;
        i2c::device_write(PMIC_TPS65217_I2C_SLAVE_ADDR, &buffer);
    }
}
