/// Writes a masked value to a 32-bit register.
///
/// This function reads the current value at the specified address, applies the mask
/// to both the current value and the new value, then writes the combined result back.
///
/// # Arguments
/// * `base` - Base address of the register
/// * `offset` - Offset from the base address
/// * `mask` - Bit mask to apply
/// * `value` - Value to write (will be masked with `mask`)
///
/// # Safety
/// The caller must ensure that `base + offset` points to a valid memory address
/// that can be read from and written to as a 32-bit value. Improper use may cause
/// undefined behavior or hardware issues.
pub unsafe fn reg32_write_masked(base: u32, offset: u32, mask: u32, value: u32) {
    unsafe {
        let addr = base + offset;
        let current_value = core::ptr::read_volatile(addr as *const u32);
        let new_value = (current_value & !mask) | (value & mask);
        core::ptr::write_volatile(addr as *mut u32, new_value);
    }
}

/// Reads and masks a value from a 32-bit register.
///
/// This function reads the current value at the specified address and applies
/// the mask to the read value before returning it.
///
/// # Arguments
/// * `base` - Base address of the register
/// * `offset` - Offset from the base address
/// * `mask` - Bit mask to apply to the read value
///
/// # Returns
/// The value read from the register with the mask applied
///
/// # Safety
/// The caller must ensure that `base + offset` points to a valid memory address
/// that can be read from as a 32-bit value. Improper use may cause
/// undefined behavior or hardware issues.
pub unsafe fn reg32_read_masked(base: u32, offset: u32, mask: u32) -> u32 {
    unsafe {
        let addr = base + offset;
        let current_value = core::ptr::read_volatile(addr as *const u32);
        current_value & mask
    }
}

/// Writes a value to a 32-bit register.
///
/// # Arguments
/// * `base` - Base address of the register
/// * `offset` - Offset from the base address
/// * `value` - Value to write to the register
///
/// # Safety
/// The caller must ensure that `base + offset` points to a valid memory address
/// that can be written to as a 32-bit value. Improper use may cause
/// undefined behavior or hardware issues.
pub unsafe fn reg32_write(base: u32, offset: u32, value: u32) {
    unsafe {
        let addr = base + offset;
        core::ptr::write_volatile(addr as *mut u32, value);
    }
}

/// Reads a value from a 32-bit register at a specific address.
///
/// # Arguments
/// * `base` - Base address of the register
/// * `offset` - Offset from the base address
///
/// # Returns
/// The value read from the register
///
/// # Safety
/// The caller must ensure that `base + offset` points to a valid memory address
/// that can be read from as a 32-bit value. Improper use may cause
/// undefined behavior or hardware issues.
pub unsafe fn reg32_read(base: u32, offset: u32) -> u32 {
    unsafe {
        let addr = base + offset;
        core::ptr::read_volatile(addr as *const u32)
    }
}

/// Clears specific bits in a 32-bit register.
///
/// # Arguments
/// * `base` - Base address of the register
/// * `offset` - Offset from the base address
/// * `bits` - Bits to clear (1s in this value will be cleared in the register)
///
/// # Safety
/// The caller must ensure that `base + offset` points to a valid memory address
/// that can be read from and written to as a 32-bit value. Improper use may cause
/// undefined behavior or hardware issues.
pub unsafe fn reg32_clear_bits(base: u32, offset: u32, bits: u32) {
    unsafe {
        let addr = (base + offset) as *mut u32;
        core::ptr::write_volatile(addr, core::ptr::read_volatile(addr) & !bits);
    }
}
