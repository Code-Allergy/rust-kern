use crate::hal::bbb::cm::*;
use crate::hal::util::{reg32_read, reg32_read_masked, reg32_write, reg32_write_masked};

const GPIO1_BASE: u32 = 0x4804c000;
const GPIO_CTRL_OFF: u32 = 0x130;
const GPIO_OE_OFF: u32 = 0x134;
const GPIO_SETDATAOUT_OFF: u32 = 0x194;
const GPIO_CLEARDATAOUT_OFF: u32 = 0x190;

const UART0_BASE: u32 = 0x44E09000;

const UART_THR_OFF: u32 = 0x00;
const UART_SYSC_OFF: u32 = 0x54;
const UART_SYSS_OFF: u32 = 0x58;
const UART_LCR_OFF: u32 = 0x0C;
const UART_EFR_OFF: u32 = 0x08;
const UART_MCR_OFF: u32 = 0x10;
const UART_FCR_OFF: u32 = 0x08;
const UART_TLR_OFF: u32 = 0x1C;
const UART_SCR_OFF: u32 = 0x40;
const UART_MDR1_OFF: u32 = 0x20;
const UART_IER_UART_OFF: u32 = 0x04;
const UART_DLL_OFF: u32 = 0x00;
const UART_DLH_OFF: u32 = 0x04;
const UART_LSR_UART_OFF: u32 = 0x14;

pub fn init() {
    unsafe {
        let stop_bit_en = 1;
        let num_stop_bits = 0;
        let parity_en = 0;
        let parity_type = 0;
        let char_length = 8;

        // Enable UART0 module clock
        reg32_write_masked(CM_WKUP_BASE, CM_WKUP_UART0_CLKCTRL, 0x3, 0x2);
        while (reg32_read(CM_WKUP_BASE, CM_WKUP_UART0_CLKCTRL) & (0x3 << 16)) > 0 {} // Wait for fully enabled

        // Enable UART0 interface clock (l4_wkup)
        reg32_write_masked(CM_WKUP_BASE, CM_WKUP_L4WKUP_CLKCTRL, 0x3, 0x2);
        while (reg32_read(CM_WKUP_BASE, CM_WKUP_L4WKUP_CLKCTRL) & (0x3 << 16)) > 0 {} // Wait for fully enabled

        // // mux pins to UART0
        reg32_write(CONTROL_MODULE_BASE, CONTROL_MODULE_CONF_UART0_RXD, 0x30);
        reg32_write(CONTROL_MODULE_BASE, CONTROL_MODULE_CONF_UART0_TXD, 0x10);

        // /* Now the steps described in the TRM (19.4.1.1)*/
        // // uart reset
        reg32_write_masked(UART0_BASE, UART_SYSC_OFF, 0x2, 0x2);
        while (reg32_read(UART0_BASE, UART_SYSS_OFF) & 0x1) != 1 {} // Wait for reset to complete
        reg32_write(UART0_BASE, UART_SYSC_OFF, 0x8);

        /*-------------- 19.4.1.1.2 FIFOs and DMA Settings --------------- */
        // 1. Save LCR and switch to register configuration mode B
        let lcr = reg32_read(UART0_BASE, UART_LCR_OFF);
        reg32_write(UART0_BASE, UART_LCR_OFF, 0xBF);

        // 2. Enable register submode TCR_TLR to access the UARTi.UART_TLR register (part 1 of 2):
        let mut efr_bit4 = reg32_read_masked(UART0_BASE, UART_EFR_OFF, 0x10);
        reg32_write_masked(UART0_BASE, UART_EFR_OFF, 0x10, 0x10); // ENHANCEDEN = 1

        // switch to register configure mode A to access the UARTi.UART_MCR register
        reg32_write(UART0_BASE, UART_LCR_OFF, 0x80);

        // 4. Enable register submode TCR_TLR to access the UARTi.UART_TLR register (part 2 of 2)
        let mcr_bit6 = reg32_read_masked(UART0_BASE, UART_MCR_OFF, 0x40);
        reg32_write_masked(UART0_BASE, UART_MCR_OFF, 0x40, 0x40); // TCR_TLR = 1

        // enable the fifo, load the new fifo triggers (1/3) and the new dma mode (1/2)
        reg32_write(UART0_BASE, UART_FCR_OFF, 0x07);

        // 6. Switch to register configuration mode B to access the UARTi.UART_EFR register
        reg32_write(UART0_BASE, UART_LCR_OFF, 0xBF);

        // 7. Load the new FIFO triggers (part 2 of 3)
        reg32_write(UART0_BASE, UART_TLR_OFF, 0x00);

        // 8. Load the new FIFO triggers (part 3 of 3) and the new DMA mode (part 2 of 2)
        reg32_write(UART0_BASE, UART_SCR_OFF, 0x00);

        // 9. Restore the UARTi.UART_EFR[4] ENHANCED_EN value saved in Step 2a
        reg32_write_masked(UART0_BASE, UART_EFR_OFF, 0x10, efr_bit4);

        // 10. Switch to register configuration mode A to access the UARTi.UART_MCR register
        reg32_write(UART0_BASE, UART_LCR_OFF, 0x80);

        // 11. Restore the UARTi.UART_MCR[6] TCR_TLR value saved in Step 4a
        reg32_write_masked(UART0_BASE, UART_MCR_OFF, 0x40, mcr_bit6);

        // 12. Restore the UARTi.UART_LCR value saved in Step 1a
        reg32_write(UART0_BASE, UART_LCR_OFF, lcr);

        /* -------------- 19.4.1.1.3 Protocol, Baud Rate, and Interrupt Settings -----------*/
        // 1. Disable UART to access the UARTi.UART_DLL and UARTi.UART_DLH registers
        reg32_write_masked(UART0_BASE, UART_MDR1_OFF, 0x7, 0x7); // Set MODE_SELECT = 0x7 (disable UART)

        // 2. Switch to register configuration mode B to access the UARTi.UART_EFR register
        reg32_write(UART0_BASE, UART_LCR_OFF, 0xBF);

        // 3. Enable access to the UARTi.UART_IER[7:4] bit field
        efr_bit4 = reg32_read_masked(UART0_BASE, UART_EFR_OFF, 0x10);
        reg32_write_masked(UART0_BASE, UART_EFR_OFF, 0x10, 0x10); // Set ENHANCED_EN = 1

        // 4. Switch to register operational mode to access the UARTi.UART_IER register
        reg32_write(UART0_BASE, UART_LCR_OFF, 0x00);

        // 5. Clear the UARTi.UART_IER register (set the UARTi.UART_IER[4] SLEEP_MODE bit to 0 to change
        //    the UARTi.UART_DLL and UARTi.UART_DLH registers). Set the UARTi.UART_IER register value to 0x0000
        reg32_write(UART0_BASE, UART_IER_UART_OFF, 0x00);

        // 6. Switch to register configuration mode B to access the UARTi.UART_DLL and UARTi.UART_DLH registers
        reg32_write(UART0_BASE, UART_LCR_OFF, 0xBF);

        // 7. Load the new divisor value
        // Baud rate = (UART module clock) / (16 * (DLL + DLH/256))
        // For 115200 baud rate, DLL = 0x1A, DLH = 0x00
        reg32_write(UART0_BASE, UART_DLL_OFF, 0x1A); // DLL = 0x1A
        reg32_write(UART0_BASE, UART_DLH_OFF, 0x00); // DLH = 0x00

        // 8. Switch to register operational mode to access the UARTi.UART_IER register
        reg32_write(UART0_BASE, UART_LCR_OFF, 0x00);

        // 9. Load the new interrupt configuration (0: Disable the interrupt; 1: Enable the interrupt)
        // Enable receive holding register interrupt
        reg32_write(UART0_BASE, UART_IER_UART_OFF, 0x01); // [0] RHRIT = 1 (Receive holding register interrupt)
        // [1] THRIT = 0 (Tranmission holding register interrupt)
        // [2] LINESTIT = 0 (receiver line status interrupt)
        // [3] MODEMSTSIT = 0 (modem status register interrupt)
        // [4] SLEEPMODE = 0 (Disables sleep mode)
        // [5] XOFFIT = 0 (XOFF interrupt)
        // [6] RTSIT = 0 (RTS (active-low) interrup)
        // [7] CTSIT = 0 (CTS (active-low) interrupt)

        // 10. Switch to register configuration mode B to access the UARTi.UART_EFR register
        reg32_write(UART0_BASE, UART_LCR_OFF, 0xBF);

        // 11. Restore the UARTi.UART_EFR[4] ENHANCED_EN value saved in Step 3a
        reg32_write_masked(UART0_BASE, UART_EFR_OFF, 0x10, efr_bit4);

        // 12. Load the new protocol formatting (parity, stop-bit, character length) and switch to register operational mode
        reg32_write(
            UART0_BASE,
            UART_LCR_OFF,
            (0 << 7) |                      // [7] DIV_EN = 0 (disable divisor latch access)
            (0 << 6) |                      // [6] BREAK_EN = 0 (disable break condition)
            (0 << 5) |                      // [5] PARITY_TYPE_2
            ((parity_type & 0x1) << 4) |    // [4] PARITY_TYPE_1
            ((parity_en & 0x1) << 3) |      // [3] PARITY_EN
            ((num_stop_bits & 0x1) << 2) |  // [2] NB_STOP
            ((char_length - 5) & 0x3), // [1:0] CHAR_LENGTH
        );

        // 13. Load the new UART mode
        reg32_write(UART0_BASE, UART_MDR1_OFF, 0x0); // UART 16x mode
    }
}

pub fn write_byte(c: u8) {
    unsafe {
        if c == b'\n' {
            write_byte('\r' as u8);
        }

        while (reg32_read(UART0_BASE, UART_LSR_UART_OFF) & 0x20) != 0x20 {}
        reg32_write(UART0_BASE, UART_THR_OFF, c as u32);
    }
}
