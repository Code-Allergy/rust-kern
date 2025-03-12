use super::regs::{base::UART0_BASE, uart::*};
use crate::util::{reg32_read, reg32_write};

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

pub fn read_byte() -> Option<u8> {
    unsafe {
        let lsr = reg32_read(UART0_BASE, LSR);
        if lsr & 0x1 != 0 {
            Some(reg32_read(UART0_BASE, RBR_THR_DLL) as u8)
        } else {
            None
        }
    }
}

// On qemu, reads are buffered and we don't need to busywait for a signal to read next
