use crate::println;

pub use platform::{DRAM_END, DRAM_SIZE, DRAM_START};

/// Initialize the DRAM, then run a quick test
pub fn init() {
    platform::init(); // Platform-specific initialization
    simple_memtest();
}

fn simple_memtest() {
    use platform::{DRAM_END, DRAM_SIZE, DRAM_START};
    unsafe {
        println!(
            "Starting DRAM test from 0x{:x} to 0x{:x} ({} bytes)",
            DRAM_START, DRAM_END, DRAM_SIZE
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
        // Testing every 1MB interval to cover the full range without taking too long
        const STRIDE: usize = 1024 * 1024 / 4; // 1MB in 4-byte words
        const NUM_TEST_LOCATIONS: usize = DRAM_SIZE / (STRIDE * 4);

        for &pattern in patterns.iter() {
            println!("Testing with pattern: 0x{:x}", pattern);
            let mut errors = 0;

            // Write pattern
            println!("Writing pattern...");
            for i in 0..NUM_TEST_LOCATIONS {
                let addr = DRAM_START + (i * STRIDE * 4);
                let ptr = addr as *mut u32;

                // Write a pattern that is address-dependent to make detection of addressing errors easier
                let value = pattern ^ (addr as u32);
                ptr.write_volatile(value);
            }

            // Verify pattern
            println!("Reading back and verifying...");
            for i in 0..NUM_TEST_LOCATIONS {
                let addr = DRAM_START + (i * STRIDE * 4);
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

            if errors == 0 {
                println!("Pattern 0x{:x} verified successfully!", pattern);
            } else {
                println!("Pattern 0x{:x} failed with {} errors", pattern, errors);
            }
        }

        // Walking ones test - useful for detecting stuck bits
        println!("Running walking ones test...");
        let mut errors = 0;
        let test_addr = DRAM_START as *mut u32;

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

        if errors == 0 {
            println!("Walking ones test passed!");
        } else {
            println!("Walking ones test failed with {} errors", errors);
        }

        println!("DRAM test completed");
    }
}

// Platform-specific UART functions
#[cfg(feature = "qemu")]
mod platform {
    pub const DRAM_START: usize = 0x4001_0000; // start offset of the bootloader
    pub const DRAM_END: usize = 0x5FFF_FFFF;
    pub const DRAM_SIZE: usize = DRAM_END - DRAM_START + 1;

    pub fn init() {}
}

#[cfg(feature = "bbb")]
mod platform {
    pub use crate::hal::bbb::dram::{init_ddr_final, init_ddr_phys, init_emif, init_vtp};
    use crate::{
        dbg,
        hal::bbb::{
            cm::{
                get_device_version, init_core_pll, init_ddr_pll, init_interface_clk, init_mpu_pll,
                init_per_pll, init_plls, init_power_domain_transition,
            },
            tps::*,
        },
        println,
    };

    pub const DRAM_START: usize = 0x8000_0000;
    pub const DRAM_END: usize = 0x9FFF_FFFF;
    pub const DRAM_SIZE: usize = DRAM_END - DRAM_START + 1;

    pub fn init() {
        init_emif();
        init_vtp();
        init_ddr_phys();
        init_ddr_final();
    }
}
