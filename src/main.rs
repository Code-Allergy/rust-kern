#![no_std]
#![no_main]
#![feature(asm)]

use core::fmt::{self, Write};

mod hal;
mod panic;
use crate::hal::uart;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        $crate::hal::Writer.write_fmt(format_args!($($arg)*)).unwrap()
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[unsafe(no_mangle)]
pub extern "C" fn rust_main() -> ! {
    unsafe {
        uart::init();
        println!("Hello, world!");
        println!("Answer of life: {}", 42);
    } // unsafe
    loop {}
}
