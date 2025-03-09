const GPIO1_BASE: u32 = 0x4804c000;

pub struct BeagleBoneBlackUart {
    reg: *mut u8,
}

impl BeagleBoneBlackUart {
    pub fn new() -> Self {
        BeagleBoneBlackUart {
            reg: GPIO1_BASE as *mut u8,
        }
    }
}
