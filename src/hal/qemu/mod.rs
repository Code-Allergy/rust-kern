pub struct CubieboardUart {
    // QEMU-specific registers (e.g., memory-mapped I/O)
    base_address: usize,
}

impl CubieboardUart {
    pub fn new(base_address: usize) -> Self {
        CubieboardUart { base_address }
    }
}
// TODO
impl Uart for CubieboardUart {
    fn init(&mut self) {
        // Configure UART for QEMU (e.g., set baud rate, enable FIFO)
        unsafe {
            // Example: Write to MMIO registers
            core::ptr::write_volatile(self.base_address as *mut u32, 0x01);
        }
    }

    fn write_byte(&mut self, byte: u8) {
        unsafe {
            // Write to QEMU's UART output (e.g., ARM PL011)
            core::ptr::write_volatile(self.base_address as *mut u8, byte);
        }
    }

    fn read_byte(&mut self) -> Option<u8> {
        // Read from QEMU's UART input (if needed)
        None
    }
}
