use super::cm::*;
use crate::hal::util::*;

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
