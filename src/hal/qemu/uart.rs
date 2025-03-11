use super::regs::{base::UART0_BASE, uart::*};
use crate::hal::util::reg32_write;

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

// On qemu, reads are buffered and we don't need to busywait for a signal to write next
pub fn write_byte(byte: u8) {
    unsafe {
        reg32_write(UART0_BASE, RBR_THR_DLL, byte as u32);
    }
}

// On qemu, reads are buffered and we don't need to busywait for a signal to read next
