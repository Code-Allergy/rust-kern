#[cfg(not(test))]
mod panic_handler {
    use crate::println;
    use core::panic::PanicInfo;
    use hal::asm;

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
}
