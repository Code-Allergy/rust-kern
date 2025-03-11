#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => {
        {
            use core::fmt::Write; // Ensure the trait is in scope
            $crate::Writer.write_fmt(format_args!($($arg)*)).unwrap();
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
