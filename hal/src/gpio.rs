// const CONTROL_MODULE_BASE: u32 = 0x44E10000;
// const CONTROL_MODULE_CONF_UART0_RXD: u32 = 0x970;
// const CONTROL_MODULE_CONF_UART0_TXD: u32 = 0x974;

// const CM_PER_BASE: u32 = 0x44E00000;
// const CM_PER_GPIO1_CLKCTRL: u32 = 0xAC;

// const CM_WKUP_BASE: u32 = 0x44E00400;
// const CM_WKUP_UART0_CLKCTRL: u32 = 0xB4;
// const CM_WKUP_L4WKUP_CLKCTRL: u32 = 0x0C;

// const GPIO1_BASE: u32 = 0x4804c000;
// const GPIO_CTRL_OFF: u32 = 0x130;
// const GPIO_OE_OFF: u32 = 0x134;
// const GPIO_SETDATAOUT_OFF: u32 = 0x194;
// const GPIO_CLEARDATAOUT_OFF: u32 = 0x190;

// const UART0_BASE: u32 = 0x44E09000;
// const UART0_REGS: *mut UartRegisters = UART0_BASE as *mut UartRegisters; // TODO

// const UART_THR_OFF: u32 = 0x00;
// const UART_SYSC_OFF: u32 = 0x54;
// const UART_SYSS_OFF: u32 = 0x58;
// const UART_LCR_OFF: u32 = 0x0C;
// const UART_EFR_OFF: u32 = 0x08;
// const UART_MCR_OFF: u32 = 0x10;
// const UART_FCR_OFF: u32 = 0x08;
// const UART_TLR_OFF: u32 = 0x1C;
// const UART_SCR_OFF: u32 = 0x40;
// const UART_MDR1_OFF: u32 = 0x20;
// const UART_IER_UART_OFF: u32 = 0x04;
// const UART_DLL_OFF: u32 = 0x00;
// const UART_DLH_OFF: u32 = 0x04;
// const UART_LSR_UART_OFF: u32 = 0x14;

// const LED_PINS: u32 = (0x8 << 21);

// // GPIO initialization
// pub unsafe fn gpio_init() {
//     // 1. Enable the clock for GPIO1
//     reg32_write(CM_PER_BASE, CM_PER_GPIO1_CLKCTRL, 0x2);

//     // 2. Wait for the clock to be enabled
//     while (reg32_read(CM_PER_BASE, CM_PER_GPIO1_CLKCTRL) & (0x3 << 16)) != 0x0 {}

//     // 3. Configure the GPIO1 module clock gating to disabled (module not gated)
//     reg32_write(GPIO1_BASE, GPIO_CTRL_OFF, 0x0);

//     // 4. Configure GPIO1 pins 21-24 as outputs (LEDs)
//     // Set the Output Enable (OE) register to set pins 21-24 as outputs
//     // let pin_mask = 0xF << 21; // Mask for pins 21-24
//     // reg32_write(GPIO1_BASE, GPIO_OE_OFF, pin_mask);
// }

// // Set the direction (input/output) of a specific GPIO pin
// pub unsafe fn gpio_set_pin_mode(gpio_base: u32, pin_mask: u32, dir: u32) {
//     // Set the direction for the specific GPIO pins
//     reg32_write(gpio_base, GPIO_OE_OFF, dir & pin_mask);
// }

// // Set GPIO pins high
// pub unsafe fn gpio_set(gpio_base: u32, pins: u32) {
//     reg32_write(gpio_base, GPIO_SETDATAOUT_OFF, pins);
// }

// // Clear GPIO pins (set them low)
// pub unsafe fn gpio_clear(gpio_base: u32, pins: u32) {
//     reg32_write(gpio_base, GPIO_CLEARDATAOUT_OFF, pins);
// }
