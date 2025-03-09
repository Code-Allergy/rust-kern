use core::ptr;

type Reg32 = u32;

pub unsafe fn reg32_write_masked(base: u32, offset: u32, mask: u32, value: u32) {
    unsafe {
        let addr = base + offset;
        let current_value = ptr::read_volatile(addr as *const u32);
        let new_value = (current_value & !mask) | (value & mask);
        ptr::write_volatile(addr as *mut u32, new_value);
    }
}

pub unsafe fn reg32_read_masked(base: u32, offset: u32, mask: u32) -> u32 {
    unsafe {
        let addr = base + offset;
        let current_value = ptr::read_volatile(addr as *const u32);
        current_value & mask
    }
}

// Unsafe function to write to a 32-bit register at a specific address
pub unsafe fn reg32_write(base: u32, offset: u32, value: u32) {
    unsafe {
        let addr = base + offset;
        ptr::write_volatile(addr as *mut u32, value);
    }
}

// Unsafe function to read from a 32-bit register at a specific address
pub unsafe fn reg32_read(base: u32, offset: u32) -> u32 {
    unsafe {
        let addr = base + offset;
        ptr::read_volatile(addr as *const u32)
    }
}

pub unsafe fn reg32_clear_bits(base: u32, offset: u32, bits: u32) {
    let addr = (base + offset) as *mut u32;
    ptr::write_volatile(addr, ptr::read_volatile(addr) & !bits);
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write; // Ensure the trait is in scope
            $crate::hal::Writer.write_fmt(format_args!($($arg)*)).unwrap();
        }
    };
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => {
        $crate::print!("{}\n", format_args!($($arg)*))
    };
}

#[macro_export]
macro_rules! dbg {
    // Handle empty case
    () => {
        $crate::println!("[{}:{}]", file!(), line!())
    };
    // Handle single expression case
    ($val:expr) => {
        match $val {
            tmp => {
                $crate::println!("[{}:{}] {} = {:#?}",
                    file!(), line!(), stringify!($val), &tmp);
                tmp
            }
        }
    };
    // Handle multiple expressions case
    ($val:expr, $($vals:expr),+ $(,)?) => {
        ($crate::dbg!($val), $($crate::dbg!($vals)),+)
    };
}
