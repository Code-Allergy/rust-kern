use crate::util::reg32_read_masked;

use super::{
    cm::get_device_version,
    i2c,
    regs::{base::CONTROL_MODULE_BASE, cm::CONTROL_EFUSE_SMA, i2c::*, tps::*},
};

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
    i2c::master_int_clear_ex(I2C_INTERRUPT_FLAG_TO_CLR);
}

fn setup_reception(offset: u8, mut dcount: u32, buffer: &mut [u8]) -> u32 {
    let mut received = 0;

    i2c::set_data_count(1);
    cleanup_interrupt();
    i2c::master_control(I2C_CFG_MST_TX);
    i2c::master_start();
    while !i2c::master_bus_busy() {}

    i2c::master_data_put(offset);
    i2c::master_int_clear_ex(I2C_INT_TRANSMIT_READY);
    while (i2c::master_int_raw_status() & I2C_INT_ADRR_READY_ACESS) == 0 {}

    i2c::set_data_count(dcount);
    cleanup_interrupt();
    i2c::master_control(I2C_CFG_MST_RX);
    i2c::master_start();

    while dcount > 0 {
        while !i2c::master_int_raw_status_ex(I2C_INT_RECEIVE_READY) {}
        buffer[received] = i2c::master_data_get();
        i2c::master_int_clear_ex(I2C_INT_RECEIVE_READY);
        dcount -= 1;
        received += 1;
    }

    i2c::master_stop();
    while i2c::master_int_raw_status() & I2C_INT_STOP_CONDITION == 0 {}
    i2c::master_int_clear_ex(I2C_INT_STOP_CONDITION);

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

// TODO: verify this function
pub fn config_vdd_op_voltage() {
    i2c::master_slave_addr_set(PMIC_TPS65217_I2C_SLAVE_ADDR);
    let _pmic_status = read_reg(STATUS);

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
        let _count = setup_reception(offset, 1, &mut buffer);
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
