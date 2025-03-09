#![no_std]
#![no_main]
#![feature(asm)]

use core::arch::asm;
use core::panic::PanicInfo;
use core::ptr;

use core::fmt::{self, Write};

type Reg32 = u32;

#[repr(C)]
struct UartRegisters {
    uart_thr_rhr_dll: Reg32, // 0x0
    uart_ier_dlh: Reg32,     // 0x4
    uart_iir_fcr: Reg32,     // 0x8
    uart_lcr: Reg32,         // 0xC
    uart_mcr: Reg32,         // 0x10
    uart_lsr: Reg32,         // 0x14
    uart_msr: Reg32,         // 0x18
    uart_spr: Reg32,         // 0x1C
    uart_mdr1: Reg32,        // 0x20
    uart_mdr2: Reg32,        // 0x24
    uart_sflsr: Reg32,       // 0x28
    uart_resv1: Reg32,       // 0x2CUART_SYSS_OFF
    uart_rxfll: Reg32,       // 0x30
    uart_rxflh: Reg32,       // 0x34
    uart_blr: Reg32,         // 0x38
    uart_acreg: Reg32,       // 0x3C
    uart_scr: Reg32,         // 0x40
    uart_ssr: Reg32,         // 0x44
    uart_eblr: Reg32,        // 0x48
    __pad0: Reg32,           // 0x4C
    uart_mvr: Reg32,         // 0x50
    uart_sysc: Reg32,        // 0x54
    uart_syss: Reg32,        // 0x58
}

const CONTROL_MODULE_BASE: u32 = 0x44E10000;
const CONTROL_MODULE_CONF_UART0_RXD: u32 = 0x970;
const CONTROL_MODULE_CONF_UART0_TXD: u32 = 0x974;

const CM_PER_BASE: u32 = 0x44E00000;
const CM_PER_GPIO1_CLKCTRL: u32 = 0xAC;

const CM_WKUP_BASE: u32 = 0x44E00400;
const CM_WKUP_UART0_CLKCTRL: u32 = 0xB4;
const CM_WKUP_L4WKUP_CLKCTRL: u32 = 0x0C;

const GPIO1_BASE: u32 = 0x4804c000;
const GPIO_CTRL_OFF: u32 = 0x130;
const GPIO_OE_OFF: u32 = 0x134;
const GPIO_SETDATAOUT_OFF: u32 = 0x194;
const GPIO_CLEARDATAOUT_OFF: u32 = 0x190;

const UART0_BASE: u32 = 0x44E09000;
const UART0_REGS: *mut UartRegisters = UART0_BASE as *mut UartRegisters; // TODO

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

const LED_PINS: u32 = (0x8 << 21);

unsafe fn reg32_write_masked(base: u32, offset: u32, mask: u32, value: u32) {
    // Calculate the address of the register by adding the base and offset
    let addr = base + offset;

    // Read the current value of the register
    let current_value = reg32_read(base, offset);

    // Mask the current value and the new value
    let new_value = (current_value & !mask) | (value & mask);

    // Write the new value back to the register
    reg32_write(base, offset, new_value);
}

unsafe fn reg32_read_masked(base: u32, offset: u32, mask: u32) -> u32 {
    // Read the current value of the register
    let current_value = reg32_read(base, offset);

    // Apply the mask to the current value and return it
    current_value & mask
}

// Unsafe function to write to a 32-bit register at a specific address
unsafe fn reg32_write(base: u32, offset: u32, value: u32) {
    let addr = base + offset;
    ptr::write_volatile(addr as *mut u32, value);
}

// Unsafe function to read from a 32-bit register at a specific address
unsafe fn reg32_read(base: u32, offset: u32) -> u32 {
    let addr = base + offset;
    ptr::read_volatile(addr as *const u32)
}

// GPIO initialization
pub unsafe fn gpio_init() {
    // 1. Enable the clock for GPIO1
    reg32_write(CM_PER_BASE, CM_PER_GPIO1_CLKCTRL, 0x2);

    // 2. Wait for the clock to be enabled
    while (reg32_read(CM_PER_BASE, CM_PER_GPIO1_CLKCTRL) & (0x3 << 16)) != 0x0 {}

    // 3. Configure the GPIO1 module clock gating to disabled (module not gated)
    reg32_write(GPIO1_BASE, GPIO_CTRL_OFF, 0x0);

    // 4. Configure GPIO1 pins 21-24 as outputs (LEDs)
    // Set the Output Enable (OE) register to set pins 21-24 as outputs
    // let pin_mask = 0xF << 21; // Mask for pins 21-24
    // reg32_write(GPIO1_BASE, GPIO_OE_OFF, pin_mask);
}

// Set the direction (input/output) of a specific GPIO pin
pub unsafe fn gpio_set_pin_mode(gpio_base: u32, pin_mask: u32, dir: u32) {
    // Set the direction for the specific GPIO pins
    reg32_write(gpio_base, GPIO_OE_OFF, dir & pin_mask);
}

// Set GPIO pins high
pub unsafe fn gpio_set(gpio_base: u32, pins: u32) {
    reg32_write(gpio_base, GPIO_SETDATAOUT_OFF, pins);
}

// Clear GPIO pins (set them low)
pub unsafe fn gpio_clear(gpio_base: u32, pins: u32) {
    reg32_write(gpio_base, GPIO_CLEARDATAOUT_OFF, pins);
}

pub unsafe fn uart_init() {
    let stop_bit_en = 1;
    let num_stop_bits = 0;
    let parity_en = 0;
    let parity_type = 0;
    let char_length = 8;

    // Enable UART0 module clock
    reg32_write_masked(CM_WKUP_BASE, CM_WKUP_UART0_CLKCTRL, 0x3, 0x2);
    while ((reg32_read(CM_WKUP_BASE, CM_WKUP_UART0_CLKCTRL) & (0x3 << 16)) > 0) {} // Wait for fully enabled

    // Enable UART0 interface clock (l4_wkup)
    reg32_write_masked(CM_WKUP_BASE, CM_WKUP_L4WKUP_CLKCTRL, 0x3, 0x2);
    while (reg32_read(CM_WKUP_BASE, CM_WKUP_L4WKUP_CLKCTRL) & (0x3 << 16) > 0) {} // Wait for fully enabled

    // // mux pins to UART0
    reg32_write(CONTROL_MODULE_BASE, CONTROL_MODULE_CONF_UART0_RXD, 0x30);
    reg32_write(CONTROL_MODULE_BASE, CONTROL_MODULE_CONF_UART0_TXD, 0x10);

    // /* Now the steps described in the TRM (19.4.1.1)*/
    // // uart reset
    reg32_write_masked(UART0_BASE, UART_SYSC_OFF, 0x2, 0x2);
    while ((reg32_read(UART0_BASE, UART_SYSS_OFF) & 0x1) != 1) {} // Wait for reset to complete
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

fn uart_putc(c: u8) {
    unsafe {
        if (c == b'\n') {
            uart_putc('\r' as u8);
        }

        while ((reg32_read(UART0_BASE, UART_LSR_UART_OFF) & 0x20) != 0x20) {}
        reg32_write(UART0_BASE, UART_THR_OFF, c as u32);
    }
}

struct UartWriter;

impl Write for UartWriter {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        // Iterate over the string and write each character using uart_write
        for byte in s.bytes() {
            uart_putc(byte);
        }
        Ok(())
    }
}

#[macro_export]
macro_rules! uart_print {
    ($($arg:tt)*) => ({
        let mut writer = UartWriter;
        write!(writer, $($arg)*).unwrap();
    });
}

#[macro_export]
macro_rules! uart_println {
    ($($arg:tt)*) => ({
        let mut writer = UartWriter;
        writeln!(writer, $($arg)*).unwrap();
    });
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        uart_init();
        uart_println!("Hello, world!");
        uart_println!("Answer of life: {}", 42);
        gpio_init();
        gpio_set_pin_mode(GPIO1_BASE, 0xF << 21, 0);
        gpio_set(GPIO1_BASE, 1 << 21);

        loop {
            gpio_set(GPIO1_BASE, LED_PINS);
            for _ in 0..10 {
                asm!("nop");
            }
            gpio_clear(GPIO1_BASE, LED_PINS);
            for _ in 0..10 {
                asm!("nop");
            }
        }
    }
}

/// Panic handler (required for `no_std`)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
