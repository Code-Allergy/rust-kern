use crate::println;

pub use platform::{DRAM_END, DRAM_START};

/// Initialize the DRAM, then run a quick test
pub fn init() {
    platform::init();
    simple_memtest();
}

fn simple_memtest() {
    #[cfg(feature = "bbb")]
    simple_memtest_from(DRAM_START, DRAM_END);

    // On qemu, the bootloader is loaded immediately
    // into dram, we do this so we dont overwrite it
    #[cfg(feature = "qemu")]
    simple_memtest_from(DRAM_START + 0x20000, DRAM_END);
}

fn simple_memtest_from(start: usize, end: usize) {
    let mut errors = 0;
    let size = end - start;
    unsafe {
        println!(
            "Testing DRAM from 0x{:x} to 0x{:x} ({} bytes)",
            start, end, size
        );

        // Test patterns
        let patterns = [
            0xAAAA_AAAA, // Alternating bits (10101010...)
            0x5555_5555, // Alternating bits (01010101...)
            0xFF00_FF00, // Alternating bytes
            0x00FF_00FF, // Alternating bytes
            0xFFFF_0000, // Alternating half-words
            0x0000_FFFF, // Alternating half-words
            0xDEAD_BEEF, // Random-like pattern
            0x1234_5678, // Incremental pattern
        ];

        // Number of words to test with each pattern
        // Testing every 4KB interval to cover the full range without taking too long
        const STRIDE: usize = 4096 / 4; // 1MB in 4-byte words
        let num_test_locations: usize = size / (STRIDE * 4);

        for &pattern in patterns.iter() {
            for i in 0..num_test_locations {
                let addr = start + (i * STRIDE * 4);
                let ptr = addr as *mut u32;

                // Write a pattern that is address-dependent to make detection of addressing errors easier
                let value = pattern ^ (addr as u32);
                ptr.write_volatile(value);
            }
            for i in 0..num_test_locations {
                let addr = start + (i * STRIDE * 4);
                let ptr = addr as *mut u32;

                let expected = pattern ^ (addr as u32);
                let actual = ptr.read_volatile();

                if actual != expected {
                    errors += 1;
                    if errors <= 10 {
                        // Limit error reporting to avoid flooding output
                        println!(
                            "ERROR at address 0x{:x}: Expected 0x{:x}, got 0x{:x}",
                            addr, expected, actual
                        );
                    }
                }
            }

            if errors > 0 {
                println!("Pattern 0x{:x} failed with {} errors", pattern, errors);
            }
        }
        let test_addr = start as *mut u32;

        for bit in 0..32 {
            let pattern = 1u32 << bit;
            test_addr.write_volatile(pattern);
            let read_value = test_addr.read_volatile();

            if read_value != pattern {
                errors += 1;
                println!(
                    "Walking ones: ERROR at bit {}: Expected 0x{:x}, got 0x{:x}",
                    bit, pattern, read_value
                );
            }
        }

        if errors > 0 {
            println!("Walking ones test failed with {} errors", errors);
        }
    }
    if errors == 0 {
        println!("DRAM test passed!");
    } else {
        panic!("DRAM test failed with {} errors", errors);
    }
}

// Platform-specific UART functions
#[cfg(feature = "qemu")]
mod platform {
    pub use crate::qemu::dram::{DRAM_END, DRAM_START};

    /// no init needed for qemu, already initialized in dram
    pub fn init() {}
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::bbb::dram::{DRAM_END, DRAM_START};
    use crate::bbb::dram::{init_ddr_final, init_ddr_phys, init_emif, init_vtp};

    pub fn init() {
        init_emif();
        init_vtp();
        init_ddr_phys();
        init_ddr_final();
    }
}
