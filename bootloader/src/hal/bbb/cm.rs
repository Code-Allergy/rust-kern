use super::regs::{
    base::{CM_PER_BASE, CM_WKUP_BASE, CONTROL_MODULE_BASE},
    cm::*,
};
use super::tps::get_opp_config;
use crate::hal::util::*;

pub fn get_device_version() -> u32 {
    unsafe { reg32_read(CONTROL_MODULE_BASE, CONTROL_DEVICE_ID) >> CONTROL_DEVICE_ID_DEVREV_SHIFT }
}

pub fn init_mpu_pll(pll_mult: u32) {
    unsafe {
        let mut reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_CLKMODE_DPLL_MPU,
            !CM_WKUP_CM_CLKMODE_DPLL_MPU_DPLL_EN,
        );
        reg_val |= CM_WKUP_CM_CLKMODE_DPLL_MPU_DPLL_EN_DPLL_MN_BYP_MODE;
        reg32_write(CM_WKUP_BASE, CM_CLKMODE_DPLL_MPU, reg_val);

        // wait for it to enter bypass mode
        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_MPU,
            CM_WKUP_CM_IDLEST_DPLL_MPU_ST_MN_BYPASS,
        ) != CM_WKUP_CM_IDLEST_DPLL_MPU_ST_MN_BYPASS
        {}

        reg32_clear_bits(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKSEL_DPLL_MPU,
            CM_WKUP_CM_CLKSEL_DPLL_MPU_DPLL_MULT | CM_WKUP_CM_CLKSEL_DPLL_MPU_DPLL_DIV,
        );
        let update = (pll_mult << CM_WKUP_CM_CLKSEL_DPLL_MPU_DPLL_MULT_SHIFT)
            | (MPUPLL_N << CM_WKUP_CM_CLKSEL_DPLL_MPU_DPLL_DIV_SHIFT);
        reg32_write_masked(CM_WKUP_BASE, CM_WKUP_CM_CLKSEL_DPLL_MPU, update, update);

        reg_val = reg32_read(CM_WKUP_BASE, CM_WKUP_CM_DIV_M2_DPLL_MPU);
        reg_val = reg_val & !CM_WKUP_CM_DIV_M2_DPLL_MPU_DPLL_CLKOUT_DIV;
        reg_val = reg_val | MPUPLL_M2;
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_DIV_M2_DPLL_MPU, reg_val);

        /* LOCK the PLL */
        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKMODE_DPLL_MPU,
            CM_WKUP_CM_CLKMODE_DPLL_MPU_DPLL_EN,
            CM_WKUP_CM_CLKMODE_DPLL_MPU_DPLL_EN,
        );

        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_MPU,
            CM_WKUP_CM_IDLEST_DPLL_MPU_ST_DPLL_CLK,
        ) != CM_WKUP_CM_IDLEST_DPLL_MPU_ST_DPLL_CLK
        {}
    }
}

pub fn init_core_pll() {
    unsafe {
        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKMODE_DPLL_CORE,
            CM_WKUP_CM_CLKMODE_DPLL_CORE_DPLL_EN,
            CM_WKUP_CM_CLKMODE_DPLL_CORE_DPLL_EN_DPLL_MN_BYP_MODE,
        );

        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_CORE,
            CM_WKUP_CM_IDLEST_DPLL_CORE_ST_MN_BYPASS,
        ) != CM_WKUP_CM_IDLEST_DPLL_CORE_ST_MN_BYPASS
        {}

        // set the multiplier and divider
        reg32_write(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKSEL_DPLL_CORE,
            (COREPLL_M << CM_WKUP_CM_CLKSEL_DPLL_CORE_DPLL_MULT_SHIFT)
                | (COREPLL_N << CM_WKUP_CM_CLKSEL_DPLL_CORE_DPLL_DIV_SHIFT),
        );

        /* M4 divider */
        let mut reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_DIV_M4_DPLL_CORE,
            !CM_WKUP_CM_DIV_M4_DPLL_CORE_HSDIVIDER_CLKOUT1_DIV,
        );
        reg_val |= COREPLL_HSD_M4 << CM_WKUP_CM_DIV_M4_DPLL_CORE_HSDIVIDER_CLKOUT1_DIV_SHIFT;
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_DIV_M4_DPLL_CORE, reg_val);

        /* M5 divider */
        reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_DIV_M5_DPLL_CORE,
            !CM_WKUP_CM_DIV_M5_DPLL_CORE_HSDIVIDER_CLKOUT2_DIV,
        );
        reg_val |= COREPLL_HSD_M5 << CM_WKUP_CM_DIV_M5_DPLL_CORE_HSDIVIDER_CLKOUT2_DIV_SHIFT;
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_DIV_M5_DPLL_CORE, reg_val);

        /* M6 divider */
        reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_DIV_M6_DPLL_CORE,
            !CM_WKUP_CM_DIV_M6_DPLL_CORE_HSDIVIDER_CLKOUT3_DIV,
        );
        reg_val |= COREPLL_HSD_M6 << CM_WKUP_CM_DIV_M6_DPLL_CORE_HSDIVIDER_CLKOUT3_DIV_SHIFT;
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_DIV_M6_DPLL_CORE, reg_val);

        /* LOCK the PLL */
        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKMODE_DPLL_CORE,
            CM_WKUP_CM_CLKMODE_DPLL_CORE_DPLL_EN,
            CM_WKUP_CM_CLKMODE_DPLL_CORE_DPLL_EN,
        );

        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_CORE,
            CM_WKUP_CM_IDLEST_DPLL_CORE_ST_DPLL_CLK,
        ) != CM_WKUP_CM_IDLEST_DPLL_CORE_ST_DPLL_CLK
        {}
    }
}

pub fn init_per_pll() {
    unsafe {
        let mut reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKMODE_DPLL_PER,
            !CM_WKUP_CM_CLKMODE_DPLL_PER_DPLL_EN,
        );
        reg_val |= CM_WKUP_CM_CLKMODE_DPLL_PER_DPLL_EN_DPLL_MN_BYP_MODE;
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_CLKMODE_DPLL_PER, reg_val);

        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_PER,
            CM_WKUP_CM_IDLEST_DPLL_PER_ST_MN_BYPASS,
        ) != CM_WKUP_CM_IDLEST_DPLL_PER_ST_MN_BYPASS
        {}

        reg32_clear_bits(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKSEL_DPLL_PERIPH,
            CM_WKUP_CM_CLKSEL_DPLL_PERIPH_DPLL_MULT | CM_WKUP_CM_CLKSEL_DPLL_PERIPH_DPLL_DIV,
        );

        /* Set the multipler and divider values for the PLL */
        reg32_write(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKSEL_DPLL_PERIPH,
            (PERPLL_M << CM_WKUP_CM_CLKSEL_DPLL_PERIPH_DPLL_MULT_SHIFT)
                | (PERPLL_N << CM_WKUP_CM_CLKSEL_DPLL_PERIPH_DPLL_DIV_SHIFT),
        );

        reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_DIV_M2_DPLL_PER,
            !CM_WKUP_CM_DIV_M2_DPLL_PER_DPLL_CLKOUT_DIV,
        );
        reg_val |= PERPLL_M2;
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_DIV_M2_DPLL_PER, reg_val);

        /* LOCK the PLL */
        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKMODE_DPLL_PER,
            CM_WKUP_CM_CLKMODE_DPLL_PER_DPLL_EN,
            CM_WKUP_CM_CLKMODE_DPLL_PER_DPLL_EN,
        );

        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_PER,
            CM_WKUP_CM_IDLEST_DPLL_PER_ST_DPLL_CLK,
        ) != CM_WKUP_CM_IDLEST_DPLL_PER_ST_DPLL_CLK
        {}
    }
}

pub fn init_ddr_pll() {
    unsafe {
        // set bypass mode
        let mut reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKMODE_DPLL_DDR,
            !CM_WKUP_CM_CLKMODE_DPLL_DDR_DPLL_EN,
        );
        reg_val |= CM_WKUP_CM_CLKMODE_DPLL_DDR_DPLL_EN_DPLL_MN_BYP_MODE;
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_CLKMODE_DPLL_DDR, reg_val);

        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_DDR,
            CM_WKUP_CM_IDLEST_DPLL_DDR_ST_MN_BYPASS,
        ) != CM_WKUP_CM_IDLEST_DPLL_DDR_ST_MN_BYPASS
        {}

        // set the multiplier and divider
        reg32_clear_bits(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKSEL_DPLL_DDR,
            CM_WKUP_CM_CLKSEL_DPLL_DDR_DPLL_MULT | CM_WKUP_CM_CLKSEL_DPLL_DDR_DPLL_DIV,
        );

        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKSEL_DPLL_DDR,
            CM_WKUP_CM_CLKSEL_DPLL_DDR_DPLL_MULT | CM_WKUP_CM_CLKSEL_DPLL_DDR_DPLL_DIV,
            (DDRPLL_M_DDR3 << CM_WKUP_CM_CLKSEL_DPLL_DDR_DPLL_MULT_SHIFT)
                | (DDRPLL_N << CM_WKUP_CM_CLKSEL_DPLL_DDR_DPLL_DIV_SHIFT),
        );

        reg_val = reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_DIV_M2_DPLL_DDR,
            !CM_WKUP_CM_DIV_M2_DPLL_DDR_DPLL_CLKOUT_DIV,
        );
        reg_val |= DDRPLL_M2;

        // set the CLKOUT2 divider
        reg32_write(CM_WKUP_BASE, CM_WKUP_CM_DIV_M2_DPLL_DDR, reg_val);

        // lock the PLL
        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_CLKMODE_DPLL_DDR,
            CM_WKUP_CM_CLKMODE_DPLL_DDR_DPLL_EN,
            CM_WKUP_CM_CLKMODE_DPLL_DDR_DPLL_EN,
        );

        while reg32_read_masked(
            CM_WKUP_BASE,
            CM_WKUP_CM_IDLEST_DPLL_DDR,
            CM_WKUP_CM_IDLEST_DPLL_DDR_ST_DPLL_CLK,
        ) != CM_WKUP_CM_IDLEST_DPLL_DDR_ST_DPLL_CLK
        {}
    }
}

pub fn init_interface_clk() {
    unsafe {
        // L3
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L3_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        while reg32_read_masked(CM_PER_BASE, CM_PER_L3_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        // L4LS
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L4LS_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        while reg32_read_masked(CM_PER_BASE, CM_PER_L4LS_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        // L4FW
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L4FW_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        while reg32_read_masked(CM_PER_BASE, CM_PER_L4FW_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        // L4WKUP
        reg32_write_masked(
            CM_WKUP_BASE,
            CM_WKUP_L4WKUP_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        while reg32_read_masked(CM_WKUP_BASE, CM_WKUP_L4WKUP_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        // L3 instr
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L3_INSTR_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        while reg32_read_masked(CM_PER_BASE, CM_PER_L3_INSTR_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}

        // L4HS
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L4HS_CLKCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );

        while reg32_read_masked(CM_PER_BASE, CM_PER_L4HS_CLKCTRL, CLKCTRL_MODULEMODE)
            != CLKCTRL_MODULEMODE_ENABLE
        {}
    }
}

pub fn init_power_domain_transition() {
    unsafe {
        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L3_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );

        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L4LS_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );

        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L4FW_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );

        reg32_write_masked(
            CM_PER_BASE,
            CM_WKUP_CLKSTCTRL,
            CLKTRCTRL_SW_WKUP,
            CLKTRCTRL_SW_WKUP,
        );

        reg32_write_masked(
            CM_PER_BASE,
            CM_PER_L3S_CLKSTCTRL,
            CLKCTRL_MODULEMODE_ENABLE,
            CLKCTRL_MODULEMODE_ENABLE,
        );
    }
}

pub fn init_plls() {
    let opp_config = get_opp_config();
    init_mpu_pll(opp_config.mpupll_m);
    init_core_pll();
    init_per_pll();
    init_ddr_pll();
    init_interface_clk();
    init_power_domain_transition();
}
