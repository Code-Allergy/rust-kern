#![allow(dead_code)]
use super::{
    cm::*,
    regs::{
        base::{CM_PER_BASE, CONTROL_MODULE_BASE},
        cm::*,
        tps::{DEVICE_VERSION_2_0, DEVICE_VERSION_2_1},
    },
};
use crate::hal::util::*;

pub const DRAM_START: usize = 0x8000_0000;
pub const DRAM_END: usize = 0x9FFF_FFFF;
pub const DRAM_SIZE: usize = DRAM_END - DRAM_START + 1;

pub const DDR_PHY_CTRL_BASE: u32 = CONTROL_MODULE_BASE + 0x2000;

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

pub const CONTROL_DDR_CKE_CTRL: u32 = 0x131c;
pub const SOC_EMIF_0_REGS: u32 = 0x4C000000;
pub const EMIF_DDR_PHY_CTRL_1: u32 = 0xE4;
pub const EMIF_DDR_PHY_CTRL_1_SHDW: u32 = 0xE8;
pub const EMIF_DDR_PHY_CTRL_2: u32 = 0xEC;

/* DDR_CKE_CTRL */
pub const CONTROL_DDR_CKE_CTRL_DDR_CKE_CTRL: u32 = 0x00000001;
pub const CONTROL_DDR_CKE_CTRL_DDR_CKE_CTRL_SHIFT: u32 = 0x00000000;

pub const CONTROL_DDR_CKE_CTRL_SMA1: u32 = 0xFFFFFFFE;
pub const CONTROL_DDR_CKE_CTRL_SMA1_SHIFT: u32 = 0x00000001;

pub const CMD0_SLAVE_RATIO_0: u32 = 0x1C;
pub const CMD0_SLAVE_FORCE_0: u32 = 0x20;
pub const CMD0_SLAVE_DELAY_0: u32 = 0x24;
pub const CMD0_LOCK_DIFF_0: u32 = 0x28;
pub const CMD0_INVERT_CLKOUT_0: u32 = 0x2C;
pub const CMD1_SLAVE_RATIO_0: u32 = 0x50;
pub const CMD1_SLAVE_FORCE_0: u32 = 0x54;
pub const CMD1_SLAVE_DELAY_0: u32 = 0x58;
pub const CMD1_LOCK_DIFF_0: u32 = 0x5C;
pub const CMD1_INVERT_CLKOUT_0: u32 = 0x60;
pub const CMD2_SLAVE_RATIO_0: u32 = 0x84;
pub const CMD2_SLAVE_FORCE_0: u32 = 0x88;
pub const CMD2_SLAVE_DELAY_0: u32 = 0x8C;
pub const CMD2_LOCK_DIFF_0: u32 = 0x90;
pub const CMD2_INVERT_CLKOUT_0: u32 = 0x94;
pub const DATA0_RD_DQS_SLAVE_RATIO_0: u32 = 0xC8;
pub const DATA0_RD_DQS_SLAVE_RATIO_1: u32 = 0xCC;
pub const DATA0_WR_DQS_SLAVE_RATIO_0: u32 = 0xDC;
pub const DATA0_WR_DQS_SLAVE_RATIO_1: u32 = 0xE0;
pub const DATA0_WRLVL_INIT_RATIO_0: u32 = 0xF0;
pub const DATA0_WRLVL_INIT_RATIO_1: u32 = 0xF4;
pub const DATA0_GATELVL_INIT_RATIO_0: u32 = 0xFC;
pub const DATA0_GATELVL_INIT_RATIO_1: u32 = 0x100;
pub const DATA0_FIFO_WE_SLAVE_RATIO_0: u32 = 0x108;
pub const DATA0_FIFO_WE_SLAVE_RATIO_1: u32 = 0x10C;
pub const DATA0_WR_DATA_SLAVE_RATIO_0: u32 = 0x120;
pub const DATA0_WR_DATA_SLAVE_RATIO_1: u32 = 0x124;
pub const DATA0_USE_RANK0_DELAYS_0: u32 = 0x134;
pub const DATA0_LOCK_DIFF_0: u32 = 0x138;
pub const DATA1_RD_DQS_SLAVE_RATIO_0: u32 = 0x16C;
pub const DATA1_RD_DQS_SLAVE_RATIO_1: u32 = 0x170;
pub const DATA1_WR_DQS_SLAVE_RATIO_0: u32 = 0x180;
pub const DATA1_WR_DQS_SLAVE_RATIO_1: u32 = 0x184;
pub const DATA1_WRLVL_INIT_RATIO_0: u32 = 0x194;
pub const DATA1_WRLVL_INIT_RATIO_1: u32 = 0x198;
pub const DATA1_GATELVL_INIT_RATIO_0: u32 = 0x1A0;
pub const DATA1_GATELVL_INIT_RATIO_1: u32 = 0x1A4;
pub const DATA1_FIFO_WE_SLAVE_RATIO_0: u32 = 0x1AC;
pub const DATA1_FIFO_WE_SLAVE_RATIO_1: u32 = 0x1B0;
pub const DATA1_WR_DATA_SLAVE_RATIO_0: u32 = 0x1C4;
pub const DATA1_WR_DATA_SLAVE_RATIO_1: u32 = 0x1C8;
pub const DATA1_USE_RANK0_DELAYS_0: u32 = 0x1D8;
pub const DATA1_LOCK_DIFF_0: u32 = 0x1DC;

pub const DDR3_CMD0_SLAVE_RATIO_0: u32 = 0x80;
pub const DDR3_CMD0_INVERT_CLKOUT_0: u32 = 0x0;
pub const DDR3_CMD1_SLAVE_RATIO_0: u32 = 0x80;
pub const DDR3_CMD1_INVERT_CLKOUT_0: u32 = 0x0;
pub const DDR3_CMD2_SLAVE_RATIO_0: u32 = 0x80;
pub const DDR3_CMD2_INVERT_CLKOUT_0: u32 = 0x0;

pub const DDR3_DATA0_RD_DQS_SLAVE_RATIO_0: u32 = 0x38;
pub const DDR3_DATA0_WR_DQS_SLAVE_RATIO_0: u32 = 0x44;
pub const DDR3_DATA0_FIFO_WE_SLAVE_RATIO_0: u32 = 0x94;
pub const DDR3_DATA0_WR_DATA_SLAVE_RATIO_0: u32 = 0x7D;

pub const DDR3_DATA0_RD_DQS_SLAVE_RATIO_1: u32 = 0x38;
pub const DDR3_DATA0_WR_DQS_SLAVE_RATIO_1: u32 = 0x44;
pub const DDR3_DATA0_FIFO_WE_SLAVE_RATIO_1: u32 = 0x94;
pub const DDR3_DATA0_WR_DATA_SLAVE_RATIO_1: u32 = 0x7D;

pub const DDR3_CONTROL_DDR_CMD_IOCTRL_0: u32 = 0x18B;
pub const DDR3_CONTROL_DDR_CMD_IOCTRL_1: u32 = 0x18B;
pub const DDR3_CONTROL_DDR_CMD_IOCTRL_2: u32 = 0x18B;

pub const DDR3_CONTROL_DDR_DATA_IOCTRL_0: u32 = 0x18B;
pub const DDR3_CONTROL_DDR_DATA_IOCTRL_1: u32 = 0x18B;

pub const DDR3_CONTROL_DDR_IO_CTRL: u32 = 0xefffffff;

pub const CONTROL_DDR_CMD_IOCTRL_0: u32 = 0x1404;
pub const CONTROL_DDR_CMD_IOCTRL_1: u32 = 0x1408;
pub const CONTROL_DDR_CMD_IOCTRL_2: u32 = 0x140C;

pub const CONTROL_DDR_DATA_IOCTRL_0: u32 = 0x1440;
pub const CONTROL_DDR_DATA_IOCTRL_1: u32 = 0x1444;

/* DDR_IO_CTRL */
pub const CONTROL_DDR_IO_CTRL_DDR3_RST_DEF_VAL: u32 = 0x80000000;
pub const CONTROL_DDR_IO_CTRL_DDR3_RST_DEF_VAL_SHIFT: u32 = 0x0000001F;

pub const CONTROL_DDR_IO_CTRL_DDR_WUCLK_DISABLE: u32 = 0x40000000;
pub const CONTROL_DDR_IO_CTRL_DDR_WUCLK_DISABLE_SHIFT: u32 = 0x0000001E;

pub const CONTROL_DDR_IO_CTRL_MDDR_SEL: u32 = 0x10000000;
pub const CONTROL_DDR_IO_CTRL_MDDR_SEL_SHIFT: u32 = 0x0000001C;

pub const CONTROL_DDR_IO_CTRL_RSVD2: u32 = 0x20000000;
pub const CONTROL_DDR_IO_CTRL_RSVD2_SHIFT: u32 = 0x0000001D;

pub const CONTROL_DDR_IO_CTRL: u32 = 0xe04;

// emif final setup
pub const DDR3_EMIF_DDR_PHY_CTRL_1: u32 = 0x06;
pub const DDR3_EMIF_DDR_PHY_CTRL_1_DY_PWRDN: u32 = 0x00100000;
pub const DDR3_EMIF_DDR_PHY_CTRL_1_SHDW: u32 = 0x06;
pub const DDR3_EMIF_DDR_PHY_CTRL_1_SHDW_DY_PWRDN: u32 = 0x00100000;
pub const DDR3_EMIF_DDR_PHY_CTRL_2: u32 = 0x06;

pub const DDR3_EMIF_SDRAM_TIM_1: u32 = 0x0AAAD4DB;
pub const DDR3_EMIF_SDRAM_TIM_1_SHDW: u32 = 0x0AAAD4DB;

pub const DDR3_EMIF_SDRAM_TIM_2: u32 = 0x266B7FDA;
pub const DDR3_EMIF_SDRAM_TIM_2_SHDW: u32 = 0x266B7FDA;

pub const DDR3_EMIF_SDRAM_TIM_3: u32 = 0x501F867F;
pub const DDR3_EMIF_SDRAM_TIM_3_SHDM: u32 = 0x501F867F;

pub const DDR3_EMIF_SDRAM_REF_CTRL_VAL1: u32 = 0x00000C30;
pub const DDR3_EMIF_SDRAM_REF_CTRL_SHDW_VAL1: u32 = 0x00000C30;

pub const DDR3_EMIF_ZQ_CONFIG_VAL: u32 = 0x50074BE4;
pub const DDR3_EMIF_SDRAM_CONFIG: u32 = 0x61C04BB2;

pub const EMIF_SDRAM_TIM_1: u32 = 0x18;
pub const EMIF_SDRAM_TIM_1_SHDW: u32 = 0x1C;
pub const EMIF_SDRAM_TIM_2: u32 = 0x20;
pub const EMIF_SDRAM_TIM_2_SHDW: u32 = 0x24;
pub const EMIF_SDRAM_TIM_3: u32 = 0x28;
pub const EMIF_SDRAM_TIM_3_SHDW: u32 = 0x2C;
pub const EMIF_SDRAM_REF_CTRL: u32 = 0x10;
pub const EMIF_SDRAM_REF_CTRL_SHDW: u32 = 0x14;
pub const EMIF_ZQ_CONFIG: u32 = 0xC8;
pub const EMIF_SDRAM_CONFIG: u32 = 0x8;
pub const CONTROL_SECURE_EMIF_SDRAM_CONFIG: u32 = 0x110;
pub const CONTROL_SECURE_EMIF_SDRAM_CONFIG_2: u32 = 0x114;

pub fn init_emif() {
    unsafe {
        let mut reg_val = reg32_read(CM_PER_BASE, CM_PER_EMIF_FW_CLKCTRL) & !(CLKCTRL_MODULEMODE);

        reg_val |= CLKCTRL_MODULEMODE_ENABLE;
        reg32_write(CM_PER_BASE, CM_PER_EMIF_FW_CLKCTRL, reg_val);

        reg_val = reg32_read(CM_PER_BASE, CM_PER_EMIF_CLKCTRL) & !(CLKCTRL_MODULEMODE);
        reg_val |= CLKCTRL_MODULEMODE_ENABLE;
        reg32_write(CM_PER_BASE, CM_PER_EMIF_CLKCTRL, reg_val);

        while (reg32_read(CM_PER_BASE, CM_PER_L3_CLKSTCTRL)
            & (CM_PER_L3_CLKSTCTRL_CLKACTIVITY_EMIF_GCLK | CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK))
            != (CM_PER_L3_CLKSTCTRL_CLKACTIVITY_EMIF_GCLK | CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK)
        {
        }
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

pub fn init_ddr_phys() {
    unsafe {
        // CMD0
        reg32_write(
            DDR_PHY_CTRL_BASE,
            CMD0_SLAVE_RATIO_0,
            DDR3_CMD0_SLAVE_RATIO_0,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            CMD0_INVERT_CLKOUT_0,
            DDR3_CMD0_INVERT_CLKOUT_0,
        );

        // CMD1
        reg32_write(
            DDR_PHY_CTRL_BASE,
            CMD1_SLAVE_RATIO_0,
            DDR3_CMD1_SLAVE_RATIO_0,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            CMD1_INVERT_CLKOUT_0,
            DDR3_CMD1_INVERT_CLKOUT_0,
        );

        // CMD2
        reg32_write(
            DDR_PHY_CTRL_BASE,
            CMD2_SLAVE_RATIO_0,
            DDR3_CMD2_SLAVE_RATIO_0,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            CMD2_INVERT_CLKOUT_0,
            DDR3_CMD2_INVERT_CLKOUT_0,
        );

        // data macro config (0)
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA0_RD_DQS_SLAVE_RATIO_0,
            DDR3_DATA0_RD_DQS_SLAVE_RATIO_0,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA0_WR_DQS_SLAVE_RATIO_0,
            DDR3_DATA0_WR_DQS_SLAVE_RATIO_0,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA0_FIFO_WE_SLAVE_RATIO_0,
            DDR3_DATA0_FIFO_WE_SLAVE_RATIO_0,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA0_WR_DATA_SLAVE_RATIO_0,
            DDR3_DATA0_WR_DATA_SLAVE_RATIO_0,
        );

        // data macro config (1)
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA1_RD_DQS_SLAVE_RATIO_1,
            DDR3_DATA0_RD_DQS_SLAVE_RATIO_1,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA1_WR_DQS_SLAVE_RATIO_1,
            DDR3_DATA0_WR_DQS_SLAVE_RATIO_1,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA1_FIFO_WE_SLAVE_RATIO_1,
            DDR3_DATA0_FIFO_WE_SLAVE_RATIO_1,
        );
        reg32_write(
            DDR_PHY_CTRL_BASE,
            DATA1_WR_DATA_SLAVE_RATIO_1,
            DDR3_DATA0_WR_DATA_SLAVE_RATIO_1,
        );
    }
}

pub fn init_ddr_final() {
    unsafe {
        let version = get_device_version();
        // IOCTL
        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_CMD_IOCTRL_0,
            DDR3_CONTROL_DDR_CMD_IOCTRL_0,
        );
        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_CMD_IOCTRL_1,
            DDR3_CONTROL_DDR_CMD_IOCTRL_1,
        );
        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_CMD_IOCTRL_2,
            DDR3_CONTROL_DDR_CMD_IOCTRL_2,
        );

        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_DATA_IOCTRL_0,
            DDR3_CONTROL_DDR_DATA_IOCTRL_0,
        );
        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_DATA_IOCTRL_1,
            DDR3_CONTROL_DDR_DATA_IOCTRL_1,
        );

        /* IO to work for DDR3 */
        let reg = reg32_read(CONTROL_MODULE_BASE, CONTROL_DDR_IO_CTRL);
        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_IO_CTRL,
            reg & DDR3_CONTROL_DDR_IO_CTRL,
        );

        reg32_write_masked(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_CKE_CTRL,
            CONTROL_DDR_CKE_CTRL_DDR_CKE_CTRL,
            CONTROL_DDR_CKE_CTRL_DDR_CKE_CTRL,
        );
        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_DDR_PHY_CTRL_1,
            DDR3_EMIF_DDR_PHY_CTRL_1,
        );

        /* dynamic power down */
        if (DEVICE_VERSION_2_1 == version) || (DEVICE_VERSION_2_0 == version) {
            reg32_write_masked(
                SOC_EMIF_0_REGS,
                EMIF_DDR_PHY_CTRL_1,
                DDR3_EMIF_DDR_PHY_CTRL_1_DY_PWRDN,
                DDR3_EMIF_DDR_PHY_CTRL_1_DY_PWRDN,
            );
        }

        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_DDR_PHY_CTRL_1_SHDW,
            DDR3_EMIF_DDR_PHY_CTRL_1_SHDW,
        );

        /* dynamic power down */
        if (DEVICE_VERSION_2_1 == version) || (DEVICE_VERSION_2_0 == version) {
            reg32_write_masked(
                SOC_EMIF_0_REGS,
                EMIF_DDR_PHY_CTRL_1_SHDW,
                DDR3_EMIF_DDR_PHY_CTRL_1_SHDW_DY_PWRDN,
                DDR3_EMIF_DDR_PHY_CTRL_1_SHDW_DY_PWRDN,
            );
        }

        // final emif setup
        //

        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_DDR_PHY_CTRL_2,
            DDR3_EMIF_DDR_PHY_CTRL_2,
        );

        reg32_write(SOC_EMIF_0_REGS, EMIF_SDRAM_TIM_1, DDR3_EMIF_SDRAM_TIM_1);
        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_SDRAM_TIM_1_SHDW,
            DDR3_EMIF_SDRAM_TIM_1_SHDW,
        );

        reg32_write(SOC_EMIF_0_REGS, EMIF_SDRAM_TIM_2, DDR3_EMIF_SDRAM_TIM_2);
        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_SDRAM_TIM_2_SHDW,
            DDR3_EMIF_SDRAM_TIM_2_SHDW,
        );

        reg32_write(SOC_EMIF_0_REGS, EMIF_SDRAM_TIM_3, DDR3_EMIF_SDRAM_TIM_3);
        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_SDRAM_TIM_3_SHDW,
            DDR3_EMIF_SDRAM_TIM_3_SHDM,
        );

        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_SDRAM_REF_CTRL,
            DDR3_EMIF_SDRAM_REF_CTRL_VAL1,
        );
        reg32_write(
            SOC_EMIF_0_REGS,
            EMIF_SDRAM_REF_CTRL_SHDW,
            DDR3_EMIF_SDRAM_REF_CTRL_SHDW_VAL1,
        );

        reg32_write(SOC_EMIF_0_REGS, EMIF_ZQ_CONFIG, DDR3_EMIF_ZQ_CONFIG_VAL);
        reg32_write(SOC_EMIF_0_REGS, EMIF_SDRAM_CONFIG, DDR3_EMIF_SDRAM_CONFIG);

        reg32_write(
            CONTROL_MODULE_BASE,
            CONTROL_SECURE_EMIF_SDRAM_CONFIG,
            DDR3_EMIF_SDRAM_CONFIG,
        );
    }
}
