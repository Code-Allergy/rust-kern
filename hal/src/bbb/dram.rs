use super::{
    cm::*,
    regs::{
        base::{CM_PER_BASE, CONTROL_MODULE_BASE, DDR_PHY_CTRL_BASE},
        cm::*,
        dram::*,
        tps::{DEVICE_VERSION_2_0, DEVICE_VERSION_2_1},
    },
};

use crate::util::*;

pub const DRAM_START: usize = 0x8000_0000;
pub const DRAM_END: usize = 0x9FFF_FFFF;
pub const DRAM_SIZE: usize = DRAM_END - DRAM_START + 1;

pub fn init_emif() {
    unsafe {
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_EMIF_FW_CLKCTRL,
            CLKCTRL_MODULEMODE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_EMIF_CLKCTRL,
            CLKCTRL_MODULEMODE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        while reg32_read_masked(
            CM_PER_BASE,
            CM_PER_L3_CLKSTCTRL,
            CM_PER_L3_CLKSTCTRL_CLKACTIVITY_EMIF_GCLK | CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK,
        ) != (CM_PER_L3_CLKSTCTRL_CLKACTIVITY_EMIF_GCLK
            | CM_PER_L3_CLKSTCTRL_CLKACTIVITY_L3_GCLK)
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
        {}
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
        reg32_clear_bits(
            CONTROL_MODULE_BASE,
            CONTROL_DDR_IO_CTRL,
            !DDR3_CONTROL_DDR_IO_CTRL,
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
