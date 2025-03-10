use core::panic::PanicInfo;

use crate::hal::asm;
use crate::println;

/// Panic handler (required for `no_std`)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {
        unsafe {
            asm::wfi();
        }
    }
}
