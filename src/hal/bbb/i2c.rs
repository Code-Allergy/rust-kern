use super::cm::*;
use crate::hal::util::*;

pub const I2C_BASE_ADDR: u32 = 0x44E0_B000;

pub const I2C_SYSC: u32 = 0x10;
pub const I2C_SYSS: u32 = 0x90;
pub const I2C_SA: u32 = 0xAC;
pub const I2C_CON: u32 = 0xA4;
pub const I2C_CNT: u32 = 0x98;
pub const I2C_DATA: u32 = 0x9C;
pub const I2C_PSC: u32 = 0xB0;
pub const I2C_SCLL: u32 = 0xB4;
pub const I2C_SCLH: u32 = 0xB8;

pub const I2C_IRQENABLE_CLR: u32 = 0x30;
pub const I2C_IRQSTATUS: u32 = 0x28;
pub const I2C_IRQSTATUS_RAW: u32 = 0x24;
pub const I2C_IRQSTATUS_RAW_BB: u32 = 0x00001000;

pub const I2C_CON_I2C_EN: u32 = 0x0000_8000;
pub const I2C_CON_STP: u32 = 0x0000_0002;
pub const I2C_CON_STT: u32 = 0x0000_0001;

pub const I2C_SYSC_SRST: u32 = 0x0000_0002;

pub const I2C_SYSS_RDONE: u32 = 0x0000_0001;
pub const I2C_SYSS_RDONE_SHIFT: u32 = 0x0000_0000;
pub const I2C_SYSS_RDONE_RSTCOMP: u32 = 0x1;
pub const I2C_SYSS_RDONE_RSTONGOING: u32 = 0x0;

pub const I2C_CON_TRX: u32 = 0x0000_0200;
pub const I2C_CON_MST: u32 = 0x0000_0400;

pub const I2C_CFG_MST_TX: u32 = I2C_CON_MST | I2C_CON_TRX;
pub const I2C_CFG_MST_RX: u32 = I2C_CON_MST;
pub const I2C_CFG_STOP: u32 = I2C_CON_STP;

pub const I2C_INT_TRANSMIT_READY: u32 = I2C_IRQSTATUS_XRDY;
pub const I2C_INT_RECEIVE_READY: u32 = I2C_IRQSTATUS_RRDY;
pub const I2C_INT_ADRR_READY_ACESS: u32 = I2C_IRQSTATUS_ARDY;

/* IRQSTATUS */
pub const I2C_IRQSTATUS_AAS: u32 = 0x0000_0200;
pub const I2C_IRQSTATUS_AAS_SHIFT: u32 = 0x0000_0009;
pub const I2C_IRQSTATUS_AAS_NO: u32 = 0x0;
pub const I2C_IRQSTATUS_AAS_RECOGNIZED: u32 = 0x1;

pub const I2C_IRQSTATUS_AERR: u32 = 0x0000_0080;
pub const I2C_IRQSTATUS_AERR_SHIFT: u32 = 0x0000_0007;
pub const I2C_IRQSTATUS_AERR_ERROR: u32 = 0x1;
pub const I2C_IRQSTATUS_AERR_NO: u32 = 0x0;

pub const I2C_IRQSTATUS_AL: u32 = 0x0000_0001;
pub const I2C_IRQSTATUS_AL_SHIFT: u32 = 0x0000_0000;
pub const I2C_IRQSTATUS_AL_LOST: u32 = 0x1;
pub const I2C_IRQSTATUS_AL_NORMAL: u32 = 0x0;

pub const I2C_IRQSTATUS_ARDY: u32 = 0x0000_0004;
pub const I2C_IRQSTATUS_ARDY_SHIFT: u32 = 0x0000_0002;
pub const I2C_IRQSTATUS_ARDY_BUSY: u32 = 0x0;
pub const I2C_IRQSTATUS_ARDY_READY: u32 = 0x1;

pub const I2C_IRQSTATUS_BB: u32 = 0x0000_1000;
pub const I2C_IRQSTATUS_BB_SHIFT: u32 = 0x0000_000C;
pub const I2C_IRQSTATUS_BB_FREE: u32 = 0x0;
pub const I2C_IRQSTATUS_BB_OCCUPIED: u32 = 0x1;

pub const I2C_IRQSTATUS_BF: u32 = 0x0000_0100;
pub const I2C_IRQSTATUS_BF_SHIFT: u32 = 0x0000_0008;
pub const I2C_IRQSTATUS_BF_FREE: u32 = 0x1;
pub const I2C_IRQSTATUS_BF_NO: u32 = 0x0;

pub const I2C_IRQSTATUS_GC: u32 = 0x0000_0020;
pub const I2C_IRQSTATUS_GC_SHIFT: u32 = 0x0000_0005;
pub const I2C_IRQSTATUS_GC_GENERALCALL: u32 = 0x1;
pub const I2C_IRQSTATUS_GC_NO: u32 = 0x0;

pub const I2C_IRQSTATUS_NACK: u32 = 0x0000_0002;
pub const I2C_IRQSTATUS_NACK_SHIFT: u32 = 0x0000_0001;
pub const I2C_IRQSTATUS_NACK_DETECTED: u32 = 0x1;
pub const I2C_IRQSTATUS_NACK_NOTDETECTED: u32 = 0x0;

pub const I2C_IRQSTATUS_RDR: u32 = 0x0000_2000;
pub const I2C_IRQSTATUS_RDR_SHIFT: u32 = 0x0000_000D;
pub const I2C_IRQSTATUS_RDR_ENABLED: u32 = 0x1;
pub const I2C_IRQSTATUS_RDR_INACTIVE: u32 = 0x0;

pub const I2C_IRQSTATUS_ROVR: u32 = 0x0000_0800;
pub const I2C_IRQSTATUS_ROVR_SHIFT: u32 = 0x0000_000B;
pub const I2C_IRQSTATUS_ROVR_NORMAL: u32 = 0x0;
pub const I2C_IRQSTATUS_ROVR_OVERRUN: u32 = 0x1;

pub const I2C_IRQSTATUS_RRDY: u32 = 0x0000_0008;
pub const I2C_IRQSTATUS_RRDY_SHIFT: u32 = 0x0000_0003;
pub const I2C_IRQSTATUS_RRDY_DATAREADY: u32 = 0x1;
pub const I2C_IRQSTATUS_RRDY_NODATA: u32 = 0x0;

pub const I2C_IRQSTATUS_STC: u32 = 0x0000_0040;
pub const I2C_IRQSTATUS_STC_SHIFT: u32 = 0x0000_0006;
pub const I2C_IRQSTATUS_STC_NO: u32 = 0x0;
pub const I2C_IRQSTATUS_STC_STARTCONDITION: u32 = 0x1;

pub const I2C_IRQSTATUS_XDR: u32 = 0x0000_4000;
pub const I2C_IRQSTATUS_XDR_SHIFT: u32 = 0x0000_000E;
pub const I2C_IRQSTATUS_XDR_ENABLED: u32 = 0x1;
pub const I2C_IRQSTATUS_XDR_INACTIVE: u32 = 0x0;

pub const I2C_IRQSTATUS_XRDY: u32 = 0x0000_0010;
pub const I2C_IRQSTATUS_XRDY_SHIFT: u32 = 0x0000_0004;
pub const I2C_IRQSTATUS_XRDY_DATAREADY: u32 = 0x1;
pub const I2C_IRQSTATUS_XRDY_ONGOING: u32 = 0x0;

pub const I2C_IRQSTATUS_XUDF: u32 = 0x0000_0400;
pub const I2C_IRQSTATUS_XUDF_SHIFT: u32 = 0x0000_000A;
pub const I2C_IRQSTATUS_XUDF_NORMAL: u32 = 0x0;
pub const I2C_IRQSTATUS_XUDF_UNDERFLOW: u32 = 0x1;

pub const I2C_INT_STOP_CONDITION: u32 = I2C_IRQSTATUS_BF;

// clocks
/// System clock fed to I2C module - 48Mhz
pub const I2C_SYSTEM_CLOCK: u32 = 48_000_000;
/// Internal clock used by I2C module - 12Mhz
pub const I2C_INTERNAL_CLOCK: u32 = 12_000_000;
/// I2C bus speed or frequency - 100Khz
pub const I2C_OUTPUT_CLOCK: u32 = 100_000;
pub const I2C_INTERRUPT_FLAG_TO_CLR: u32 = 0x7FF;

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
