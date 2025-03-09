use core::panic::PanicInfo;

/// Panic handler (required for `no_std`)
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
