use crate::hal::util::*;

use super::cm::*;

pub const CONTROL_VTP_CTRL: u32 = 0xe0c;
pub const CONTROL_VREF_CTRL: u32 = 0xe14;
/* VTP_CTRL */
pub const CONTROL_VTP_CTRL_CLRZ: u32 = 0x00000001;
pub const CONTROL_VTP_CTRL_CLRZ_SHIFT: u32 = 0x00000000;

pub const CONTROL_VTP_CTRL_ENABLE: u32 = 0x00000040;
pub const CONTROL_VTP_CTRL_ENABLE_SHIFT: u32 = 0x00000006;

pub const CONTROL_VTP_CTRL_FILTER: u32 = 0x0000000E;
pub const CONTROL_VTP_CTRL_FILTER_SHIFT: u32 = 0x00000001;

pub const CONTROL_VTP_CTRL_LOCK: u32 = 0x00000010;
pub const CONTROL_VTP_CTRL_LOCK_SHIFT: u32 = 0x00000004;

pub const CONTROL_VTP_CTRL_NCIN: u32 = 0x00007F00;
pub const CONTROL_VTP_CTRL_NCIN_SHIFT: u32 = 0x00000008;

pub const CONTROL_VTP_CTRL_PCIN: u32 = 0x007F0000;
pub const CONTROL_VTP_CTRL_PCIN_SHIFT: u32 = 0x00000010;

pub const CONTROL_VTP_CTRL_READY: u32 = 0x00000020;
pub const CONTROL_VTP_CTRL_READY_SHIFT: u32 = 0x00000005;

pub const CONTROL_VTP_CTRL_RSVD2: u32 = 0x00008000;
pub const CONTROL_VTP_CTRL_RSVD2_SHIFT: u32 = 0x0000000F;

pub const CONTROL_VTP_CTRL_RSVD3: u32 = 0xFF800000;
pub const CONTROL_VTP_CTRL_RSVD3_SHIFT: u32 = 0x00000017;

pub fn init_emif() {
    unsafe {
        let mut reg_val = reg32_read(CM_PER_BASE, CM_PER_EMIF_FW_CLKCTRL) & !(CLKCTRL_MODULEMODE);

        reg_val |= CLKCTRL_MODULEMODE_ENABLE;
        reg32_write(CM_PER_BASE, CM_PER_EMIF_FW_CLKCTRL, reg_val);

        reg_val = reg32_read(CM_PER_BASE, CM_PER_EMIF_CLKCTRL) & !(CLKCTRL_MODULEMODE);
        reg_val |= CLKCTRL_MODULEMODE_ENABLE;
        reg32_write(CM_PER_BASE, CM_PER_EMIF_CLKCTRL, reg_val);

        while ((reg32_read(CM_PER_BASE, CM_PER_L3_CLKSTCTRL)
            & (CM_PER_L3_CLKSTCTRL_CLKACTIVITY_EMIF_GCLK
                | CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK))
            != (CM_PER_L3_CLKSTCTRL_CLKACTIVITY_EMIF_GCLK
                | CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK))
        {}
    }
}

pub fn init_vtp() {
    unsafe {
        reg32_write_masked(
            CONTROL_MODULE_BASE,
            CONTROL_VTP_CTRL,
            CONTROL_VTP_CTRL_ENABLE,
            CONTROL_VTP_CTRL_ENABLE,
        );

        reg32_write_masked(
            CONTROL_MODULE_BASE,
            CONTROL_VTP_CTRL,
            CONTROL_VTP_CTRL_CLRZ,
            CONTROL_VTP_CTRL_CLRZ,
        );

        while reg32_read(CONTROL_MODULE_BASE, CONTROL_VTP_CTRL) & CONTROL_VTP_CTRL_READY
            != CONTROL_VTP_CTRL_READY
        {} // Wait for VTP to be ready
    }
}
