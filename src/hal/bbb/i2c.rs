use super::regs::{
    base::{CM_PER_BASE, CM_WKUP_BASE, CONTROL_MODULE_BASE, I2C_BASE_ADDR},
    cm::*,
    i2c::*,
};
use crate::hal::util::*;

pub fn master_disable() {
    unsafe {
        reg32_clear_bits(I2C_BASE_ADDR, I2C_CON, I2C_CON_I2C_EN);
    }
}

pub fn soft_reset() {
    unsafe {
        reg32_write_masked(I2C_BASE_ADDR, I2C_SYSC, I2C_SYSC_SRST, I2C_SYSC_SRST);
    }
}

pub fn auto_idle_disable() {
    unsafe {
        reg32_clear_bits(I2C_BASE_ADDR, I2C_SYSC, I2C_SYSS_RDONE);
    }
}

pub fn master_init_clock(sys_clock: u32, internal_clock: u32, output_clock: u32) {
    unsafe {
        let prescaler = (sys_clock / internal_clock) - 1;
        reg32_write(I2C_BASE_ADDR, I2C_PSC, prescaler);
        let divider = (internal_clock / output_clock) / 2;
        reg32_write(I2C_BASE_ADDR, I2C_SCLL, divider - 7);
        reg32_write(I2C_BASE_ADDR, I2C_SCLH, divider - 5);
    }
}

pub fn master_enable() {
    unsafe {
        reg32_write_masked(I2C_BASE_ADDR, I2C_CON, I2C_CON_I2C_EN, I2C_CON_I2C_EN);
    }
}

pub fn master_slave_addr_set(addr: u8) {
    unsafe {
        reg32_write(I2C_BASE_ADDR, I2C_SA, addr as u32);
    }
}

pub fn master_int_disable_ex(int_flag: u32) {
    unsafe { reg32_write(I2C_BASE_ADDR, I2C_IRQENABLE_CLR, int_flag) }
}

pub fn master_data_put(data: u8) {
    unsafe {
        reg32_write(I2C_BASE_ADDR, I2C_DATA, data as u32);
    }
}

pub fn master_data_get() -> u8 {
    unsafe { reg32_read(I2C_BASE_ADDR, I2C_DATA) as u8 }
}

pub fn master_int_raw_status_ex(int_flag: u32) -> bool {
    unsafe { reg32_read_masked(I2C_BASE_ADDR, I2C_IRQSTATUS_RAW, int_flag) == int_flag }
}

pub fn master_int_raw_status() -> u32 {
    unsafe { reg32_read(I2C_BASE_ADDR, I2C_IRQSTATUS_RAW) }
}

pub fn master_stop() {
    unsafe {
        reg32_write_masked(I2C_BASE_ADDR, I2C_CON, I2C_CON_STP, I2C_CON_STP);
    }
}

pub fn set_data_count(count: u32) {
    unsafe {
        reg32_write(I2C_BASE_ADDR, I2C_CNT, count);
    }
}

pub fn master_int_clear_ex(int_flag: u32) {
    unsafe { reg32_write(I2C_BASE_ADDR, I2C_IRQSTATUS, int_flag) }
}

pub fn master_control(cmd: u32) {
    unsafe {
        reg32_write(I2C_BASE_ADDR, I2C_CON, cmd | I2C_CON_I2C_EN);
    }
}

pub fn master_start() {
    unsafe {
        reg32_write_masked(I2C_BASE_ADDR, I2C_CON, I2C_CON_STT, I2C_CON_STT);
    }
}

pub fn master_bus_busy() -> bool {
    unsafe {
        reg32_read_masked(I2C_BASE_ADDR, I2C_IRQSTATUS_RAW, I2C_IRQSTATUS_RAW_BB)
            == I2C_IRQSTATUS_RAW_BB
    }
}

pub fn system_status_ready() -> bool {
    unsafe { reg32_read_masked(I2C_BASE_ADDR, I2C_SYSS, I2C_SYSS_RDONE) == I2C_SYSS_RDONE }
}

pub fn init_clocks() {
    unsafe {
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L3_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );
        while reg32_read_masked(CM_PER_BASE, CM_PER_L3_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L3_INSTR_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );
        while reg32_read_masked(CM_PER_BASE, CM_PER_L3_INSTR_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L3_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );
        while reg32_read_masked(CM_PER_BASE, CM_PER_L3_CLKSTCTRL, CLKTRCTRL) != CLKTRCTRL_SW_WKUP {}

        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_OCPWP_L3_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );
        while reg32_read_masked(CM_PER_BASE, CM_PER_OCPWP_L3_CLKSTCTRL, CLKTRCTRL)
            != CLKTRCTRL_SW_WKUP
        {}

        reg32_write_masked(
            CM_WKUP_BASE,
            CM_PER_L3S_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );
        while reg32_read_masked(CM_PER_BASE, CM_PER_L3S_CLKSTCTRL, CLKTRCTRL) != CLKTRCTRL_SW_WKUP {
        }

        // done wakeups, check clock status
        //
        while reg32_read_masked(CM_PER_BASE, CM_PER_L3_CLKCTRL, CLKCTRL_IDLEST)
            != (CLKCTRL_IDLEST_FUNC << CLKCTRL_IDLEST_SHIFT)
        {}

        while reg32_read_masked(CM_PER_BASE, CM_PER_L3_INSTR_CLKCTRL, CLKCTRL_IDLEST)
            != (CLKCTRL_IDLEST_FUNC << CLKCTRL_IDLEST_SHIFT)
        {}

        while reg32_read_masked(
            CM_PER_BASE,
            CM_PER_L3_CLKSTCTRL,
            CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK,
        ) != CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK
        {}

        while reg32_read_masked(
            CM_PER_BASE,
            CM_PER_OCPWP_L3_CLKSTCTRL,
            CM_PER_OCPWP_L3_CLKSTCTRL_CLKACTIVITY_OCPWP_L3_GCLK,
        ) != CM_PER_OCPWP_L3_CLKSTCTRL_CLKACTIVITY_OCPWP_L3_GCLK
        {}

        while reg32_read_masked(
            CM_PER_BASE,
            CM_PER_L3S_CLKSTCTRL,
            CM_PER_L3S_CLKSTCTRL_CLKACTIVITY_L3S_GCLK,
        ) != CM_PER_L3S_CLKSTCTRL_CLKACTIVITY_L3S_GCLK
        {}

        /* Registers for wakeup region */
        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CONTROL_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );
        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_CONTROL_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );
        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_CLKSTCTRL, CLKTRCTRL) != CLKTRCTRL_SW_WKUP {}

        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_L3_AON_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );
        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_CM_L3_AON_CLKSTCTRL, CLKTRCTRL)
            != CLKTRCTRL_SW_WKUP
        {}

        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_I2C0_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );
        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_I2C0_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        // verify clocks are enabled
        //
        //

        /*
         ** Waiting for IDLEST field in CM_WKUP_CONTROL_CLKCTRL register to attain
         ** desired value.
         */
        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_CONTROL_CLKCTRL, CLKCTRL_IDLEST)
            != (CLKCTRL_IDLEST_FUNC << CLKCTRL_IDLEST_SHIFT)
        {}

        /*
         ** Waiting for CLKACTIVITY_L3_AON_GCLK field in CM_L3_AON_CLKSTCTRL
         ** register to attain desired value.
         */
        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_L3_AON_CLKSTCTRL,
            CM_WKUP_CM_L3_AON_CLKSTCTRL_CLKACTIVITY_L3_AON_GCLK,
        ) != CM_WKUP_CM_L3_AON_CLKSTCTRL_CLKACTIVITY_L3_AON_GCLK
        {}

        /*
         ** Waiting for IDLEST field in CM_WKUP_L4WKUP_CLKCTRL register to attain
         ** desired value.
         */
        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_L4WKUP_CLKCTRL, CLKCTRL_IDLEST)
            != (CLKCTRL_IDLEST_FUNC << CLKCTRL_IDLEST_SHIFT)
        {}

        /*
         ** Waiting for CLKACTIVITY_L4_WKUP_GCLK field in CM_WKUP_CLKSTCTRL register
         ** to attain desired value.
         */
        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CLKSTCTRL,
            CM_WKUP_CLKSTCTRL_CLKACTIVITY_L4_WKUP_GCLK,
        ) != CM_WKUP_CLKSTCTRL_CLKACTIVITY_L4_WKUP_GCLK
        {}

        /*
         ** Waiting for CLKACTIVITY_L4_WKUP_AON_GCLK field in CM_L4_WKUP_AON_CLKSTCTRL
         ** register to attain desired value.
         */
        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_L4_WKUP_AON_CLKSTCTRL,
            CM_WKUP_CM_L4_WKUP_AON_CLKSTCTRL_CLKACTIVITY_L4_WKUP_AON_GCLK,
        ) != CM_WKUP_CM_L4_WKUP_AON_CLKSTCTRL_CLKACTIVITY_L4_WKUP_AON_GCLK
        {}

        /*
         ** Waiting for CLKACTIVITY_I2C0_GFCLK field in CM_WKUP_CLKSTCTRL
         ** register to attain desired value.
         */
        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CLKSTCTRL,
            CM_WKUP_CLKSTCTRL_CLKACTIVITY_I2C0_GFCLK,
        ) != CM_WKUP_CLKSTCTRL_CLKACTIVITY_I2C0_GFCLK
        {}

        /*
         ** Waiting for IDLEST field in CM_WKUP_I2C0_CLKCTRL register to attain
         ** desired value.
         */
        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_I2C0_CLKCTRL, CLKCTRL_IDLEST)
            != (CLKCTRL_IDLEST_FUNC << CLKCTRL_IDLEST_SHIFT)
        {}
    }
}

pub fn mux_pins(instance: u32) {
    match instance {
        0 => unsafe {
            reg32_write(
                CONTROL_MODULE_BASE,
                CONTROL_CONF_I2C0_SDA,
                CONTROL_CONF_I2C0_SDA_CONF_I2C0_SDA_RXACTIVE
                    | CONTROL_CONF_I2C0_SDA_CONF_I2C0_SDA_SLEWCTRL
                    | CONTROL_CONF_I2C0_SDA_CONF_I2C0_SDA_PUTYPESEL,
            );
            reg32_write(
                CONTROL_MODULE_BASE,
                CONTROL_CONF_I2C0_SCL,
                CONTROL_CONF_I2C0_SCL_CONF_I2C0_SCL_RXACTIVE
                    | CONTROL_CONF_I2C0_SCL_CONF_I2C0_SCL_SLEWCTRL
                    | CONTROL_CONF_I2C0_SCL_CONF_I2C0_SCL_PUTYPESEL,
            );
        },
        _ => {}
    }
}

pub fn device_write(addr: u8, data: &[u8]) {
    let mut i = 0;
    let mut len = data.len();
    master_slave_addr_set(addr);
    set_data_count(len as u32);
    master_int_clear_ex(I2C_INTERRUPT_FLAG_TO_CLR);
    master_control(I2C_CFG_MST_TX);
    master_start();

    while !master_bus_busy() {}

    while (master_int_raw_status() & I2C_INT_TRANSMIT_READY == I2C_INT_TRANSMIT_READY) && len > 0 {
        master_data_put(data[i]);
        master_int_clear_ex(I2C_INT_TRANSMIT_READY);
        i += 1;
        len -= 1;
    }

    master_stop();

    while master_int_raw_status() & I2C_INT_STOP_CONDITION == 0 {}
    master_int_clear_ex(I2C_INT_STOP_CONDITION);
}
