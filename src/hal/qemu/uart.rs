use crate::hal::util::reg32_write;
const UART0_BASE: u32 = 0x01C28000;
const RBR_THR_DLL: u32 = 0x00;
const IER_DLH: u32 = 0x04;
const IIR_FCR: u32 = 0x08;
const LCR: u32 = 0x0C;
const MCR: u32 = 0x10;
const LSR: u32 = 0x14;
const MSR: u32 = 0x18;
const SCR: u32 = 0x1C;

pub fn init() {
    unsafe {
        reg32_write(UART0_BASE, IER_DLH, 0x0);
        reg32_write(UART0_BASE, LCR, 0x80);
        reg32_write(UART0_BASE, RBR_THR_DLL, 13);
        reg32_write(UART0_BASE, IER_DLH, 0x0);
        reg32_write(UART0_BASE, LCR, 0x3);
        reg32_write(UART0_BASE, IIR_FCR, 0x1);
    }
}

pub fn write_byte(byte: u8) {
    unsafe {
        // while (reg32_read(UART0_BASE, LSR) & 0x20) == 0 {}
        reg32_write(UART0_BASE, RBR_THR_DLL, byte as u32);
    }
}
