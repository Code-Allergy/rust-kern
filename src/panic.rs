use core::panic::PanicInfo;

use crate::{dbg, println};

/// Panic handler (required for `no_std`)
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    // dbg!(info);
    loop {}
}
