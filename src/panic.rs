use core::panic::PanicInfo;

use crate::println;

/// Panic handler (required for `no_std`)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    println!("OOPS! Panic!");
    loop {}
}
