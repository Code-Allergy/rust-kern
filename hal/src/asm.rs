#![allow(dead_code)]
use core::arch::asm;

/// # Get the current value of the DACR register
///
/// # Safety
/// This function uses raw assembly to read the DACR (Domain Access Control Register)
/// from the ARM system control coprocessor. Incorrect usage can lead to memory protection
/// violations or system instability. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. Any changes to memory access permissions are properly coordinated
/// 3. This operation is appropriate for the current system state
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Returns
/// The function returns the value of the DACR register.
///
/// # Assembly
/// ```asm
/// mrc p15, 0, {output}, c3, c0, 0
/// ```
/// This instruction reads the value of the DACR from the ARM coprocessor and stores it in the
/// `output` register.
#[inline(always)]
pub unsafe fn get_dacr() -> u32 {
    let dacr: u32;
    unsafe {
        asm!(
            "mrc p15, 0, {dacr}, c3, c0, 0",
            dacr = out(reg) dacr,
            options(nomem, nostack, preserves_flags)
        );
    }
    dacr
}

/// # Get the current value of the DACR register
///
/// # Safety
/// This function uses raw assembly to set the DACR (Domain Access Control Register)
/// in the ARM system control coprocessor. Incorrect usage can lead to memory protection
/// violations or system instability. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. Any changes to memory access permissions are properly coordinated
/// 3. This operation is appropriate for the current system state
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Parameters
/// * `dacr` - The value to write to the DACR register
///
/// # Assembly
/// mcr p15, 0, {input}, c3, c0, 0
#[inline(always)]
pub unsafe fn set_dacr(dacr: u32) {
    unsafe {
        asm!(
            "mcr p15, 0, {dacr}, c3, c0, 0",
            dacr = in(reg) dacr,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// # Execute a No Operation instruction that consumes a CPU cycle
///
/// # Safety
/// This function uses raw assembly to execute a NOP (No Operation) instruction.
/// The NOP instruction performs no operation, causing the processor to skip one cycle.
/// This is commonly used for:
///
/// 1. Creating small delays in timing-sensitive code
/// 2. Providing padding between instructions for alignment purposes
/// 3. Preventing instruction pipeline hazards in some processors
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Assembly
/// nop
#[inline(always)]
pub unsafe fn nop() {
    unsafe {
        asm!("nop", options(nomem, nostack, preserves_flags));
    }
}

/// # Safety
/// This function uses raw assembly to execute a WFI (Wait For Interrupt) instruction.
/// The WFI instruction suspends execution until one of the following events occurs:
///
/// 1. An IRQ interrupt
/// 2. An FIQ interrupt
/// 3. A debug event with debug enabled
///
/// This instruction is commonly used to:
/// - Reduce power consumption by putting the processor into a low-power state
/// - Implement idle loops in an energy-efficient manner
/// - Wait for external events in interrupt-driven systems
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Assembly
/// wfi
#[inline(always)]
pub unsafe fn wfi() {
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }
}

/// # Safety
/// This function uses raw assembly to execute a DSB (Data Synchronization Barrier) instruction.
/// The DSB instruction ensures that all explicit memory accesses occurring before the DSB
/// are completed before any instructions after the DSB are executed. This provides:
///
/// 1. Memory synchronization between different execution contexts
/// 2. Guaranteed visibility of data changes across the system
/// 3. A control point for memory access ordering
///
/// Common use cases include:
/// - Ensuring memory updates are visible to other processors
/// - Completing memory operations before entering a low-power state
/// - Synchronizing memory access with memory management operations
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Assembly
/// dsb
#[inline(always)]
pub unsafe fn dsb() {
    unsafe {
        asm!("dsb", options(nomem, nostack, preserves_flags));
    }
}

/// # Safety
/// This function uses raw assembly to execute an ISB (Instruction Synchronization Barrier) instruction.
/// The ISB instruction flushes the processor's pipeline and ensures that all instructions following
/// the ISB are fetched from cache or memory after the ISB has been completed. This provides:
///
/// 1. Context synchronization events required by some system operations
/// 2. Guaranteed that any changes to the instruction stream are visible to subsequent instructions
/// 3. A barrier that prevents speculative execution of subsequent instructions
///
/// Common use cases include:
/// - After modifying code or after self-modifying code
/// - Following changes to system control registers that affect instruction execution
/// - After context-changing operations like exception handling
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Assembly
/// isb
#[inline(always)]
pub unsafe fn isb() {
    unsafe {
        asm!("isb", options(nomem, nostack, preserves_flags));
    }
}
/// # Safety
/// This function uses raw assembly to read the TTBR0 (Translation Table Base Register 0)
/// from the ARM system control coprocessor. TTBR0 contains the base address of the first-level
/// translation table for memory management. Incorrect usage can lead to memory management errors.
/// The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The read operation is appropriate for the current system state
/// 3. Any subsequent use of the returned value is handled correctly
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Returns
/// The function returns the current value of the TTBR0 register.
///
/// # Assembly
/// mrc p15, 0, {output}, c2, c0, 0
#[inline(always)]
pub unsafe fn read_ttbr0() -> u32 {
    let ttbr0: u32;
    unsafe {
        asm!(
            "mrc p15, 0, {ttbr0}, c2, c0, 0",
            ttbr0 = out(reg) ttbr0,
            options(nomem, nostack, preserves_flags)
        );
    }
    ttbr0
}

/// # Safety
/// This function uses raw assembly to set the TTBR0 (Translation Table Base Register 0)
/// in the ARM system control coprocessor. TTBR0 contains the base address of the first-level
/// translation table for memory management. Incorrect usage can lead to memory management errors
/// or system instability. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The provided value is a valid translation table base address
/// 3. Any changes to the translation table are properly coordinated
/// 4. This operation is appropriate for the current system state
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Parameters
/// * `ttbr0` - The value to write to the TTBR0 register
///
/// # Assembly
/// mcr p15, 0, {input}, c2, c0, 0
#[inline(always)]
pub unsafe fn set_ttbr0(ttbr0: u32) {
    unsafe {
        asm!(
            "mcr p15, 0, {ttbr0}, c2, c0, 0",
            ttbr0 = in(reg) ttbr0,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// # Safety
/// This function uses raw assembly to read the TTBR1 (Translation Table Base Register 1)
/// from the ARM system control coprocessor. TTBR1 contains the base address of the first-level
/// translation table for memory management. Incorrect usage can lead to memory management errors.
/// The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The read operation is appropriate for the current system state
/// 3. Any subsequent use of the returned value is handled correctly
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Returns
/// The function returns the current value of the TTBR1 register.
///
/// # Assembly
/// mrc p15, 0, {output}, c2, c0, 1
#[inline(always)]
pub unsafe fn read_ttbr1() -> u32 {
    let ttbr1: u32;
    unsafe {
        asm!(
            "mrc p15, 0, {ttbr1}, c2, c0, 1",
            ttbr1 = out(reg) ttbr1,
            options(nomem, nostack, preserves_flags)
        );
    }
    ttbr1
}

/// # Safety
/// This function uses raw assembly to set the TTBR1 (Translation Table Base Register 1)
/// in the ARM system control coprocessor. TTBR1 contains the base address of the first-level
/// translation table for memory management. Incorrect usage can lead to memory management errors
/// or system instability. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The provided value is a valid translation table base address
/// 3. Any changes to the translation table are properly coordinated
/// 4. This operation is appropriate for the current system state
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Parameters
/// * `ttbr1` - The value to write to the TTBR1 register
///
/// # Assembly
/// mcr p15, 0, {input}, c2, c0, 1
#[inline(always)]
pub unsafe fn set_ttbr1(ttbr1: u32) {
    unsafe {
        asm!(
            "mcr p15, 0, {ttbr1}, c2, c0, 1",
            ttbr1 = in(reg) ttbr1,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// # Safety
/// This function uses raw assembly to flush the TLB (Translation Lookaside Buffer)
/// in the ARM system control coprocessor. Flushing the TLB invalidates all entries,
/// forcing the MMU to fetch new translations from the page tables. Incorrect usage can lead
/// to memory access errors. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. Flushing the TLB is appropriate for the current system state
/// 3. Any necessary synchronization is performed before accessing memory after the flush
/// 4. The TLB flush is only guaranteed to be finished after a [dsb] instruction
///
/// Common use cases include:
/// - After modifying page table entries
/// - During context switches between processes
/// - When changing memory protection attributes
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Assembly
/// mcr p15, 0, r0, c8, c7, 0
#[inline(always)]
pub unsafe fn flush_tlb() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c8, c7, 0",
            in(reg) 0,
            options(nomem, nostack)
        );
    }
}

/// # Safety
/// This function uses raw assembly to flush a specific TLB entry in the ARM system control coprocessor.
/// It invalidates the TLB entry that matches the specified Modified Virtual Address (MVA),
/// forcing the MMU to fetch a new translation from the page tables for that address.
/// Incorrect usage can lead to memory access errors. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The provided MVA corresponds to a valid virtual address
/// 3. Flushing the specific TLB entry is appropriate for the current system state
/// 4. Any necessary synchronization is performed before accessing the address after the flush
/// 5. The TLB flush is only guaranteed to be finished after a [dsb] instruction
///
/// Common use cases include:
/// - After modifying a specific page table entry
/// - When changing memory protection attributes for a specific page
/// - When unmapping a specific virtual memory address
///
/// The function internally uses inline assembly that does not access memory or stack.
///
/// # Parameters
/// * `mva` - The Modified Virtual Address for which the TLB entry should be invalidated
///
/// # Assembly
/// mcr p15, 0, {mva}, c8, c7, 1
#[inline(always)]
pub unsafe fn flush_tlb_entry(mva: u32) {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c8, c7, 1",
            in(reg) mva,
            options(nostack, nomem)
        );
    }
}

/// # Safety
/// This function uses raw assembly to flush TLB entries associated with a specific ASID
/// (Address Space Identifier) in the ARM system control coprocessor. This operation invalidates
/// all TLB entries that match the specified ASID, forcing the MMU to fetch new translations from
/// the page tables for that address space. Incorrect usage can lead to memory access errors.
/// The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The provided ASID is valid and currently in use by some address space
/// 3. Flushing the TLB entries is appropriate for the current system state
/// 4. Any necessary synchronization is performed before accessing memory after the flush
/// 5. The TLB flush is only guaranteed to be finished after a [dsb] instruction
///
/// Common use cases include:
/// - During context switches between processes with different ASIDs
/// - After modifying page table entries for a specific address space
/// - When changing memory protection attributes for a specific address space
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Parameters
/// * `asid` - The 8-bit ASID value (0-255) for which TLB entries should be invalidated
///
/// # Assembly
/// mcr p15, 0, {asid}, c8, c7, 2
#[inline(always)]
pub unsafe fn flush_tlb_asid(asid: u32) {
    assert!(asid < 256, "ASID must be 8 bits");
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c8, c7, 2",
            in(reg) asid,
            options(nomem, nostack)
        );
    }
}

/// # Safety
/// This function uses raw assembly to flush the instruction cache (I-Cache)
/// in the ARM system control coprocessor. Flushing the I-Cache invalidates all cached
/// instructions, forcing the processor to fetch them again from memory. Incorrect usage
/// can lead to execution errors. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. Flushing the I-Cache is appropriate for the current system state
/// 3. Any necessary synchronization is performed after the flush
/// 4. The I-Cache flush is only guaranteed to be finished after an [isb] instruction
///
/// Common use cases include:
/// - After loading new code into memory
/// - After self-modifying code has been written
/// - After updating instructions in memory through data access
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Assembly
/// mcr p15, 0, {0}, c7, c5, 0
#[inline(always)]
pub unsafe fn flush_i_cache() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c5, 0",
            in(reg) 0,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// # Safety
/// This function uses raw assembly to set specific control flags in the SCTLR (System Control Register)
/// of the ARM system control coprocessor. It reads the current value, sets the specified bit flag(s),
/// and writes the updated value back. Incorrect usage can lead to system instability or crashes.
/// The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The flag parameter contains valid bit positions for the SCTLR
/// 3. Setting the specified flags is appropriate for the current system state
/// 4. The operation won't conflict with other system requirements
///
/// Common use cases include:
/// - Enabling the MMU
/// - Enabling instruction or data caches
/// - Enabling branch prediction
/// - Changing alignment checking behavior
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Parameters
/// * `flag` - Bit flag(s) to set in the SCTLR
///
/// # Assembly
///
/// ```asm
/// mrc p15, 0, {control}, c1, c0, 0  // Read current SCTLR value
/// control |= flag
/// mcr p15, 0, {control}, c1, c0, 0  // Write updated SCTLR value
/// ```
#[inline(always)]
pub unsafe fn set_scltr_flag(flag: u32) {
    unsafe {
        let mut control: u32;
        asm!(
            "mrc p15, 0, {control}, c1, c0, 0",
            control = out(reg) control,
            options(nomem, nostack, preserves_flags)
        );
        control |= flag;
        asm!(
            "mcr p15, 0, {control}, c1, c0, 0",
            control = in(reg) control,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// # Safety
/// This function uses raw assembly to clear specific control flags in the SCTLR (System Control Register)
/// of the ARM system control coprocessor. It reads the current value, clears the specified bit flag(s),
/// and writes the updated value back. Incorrect usage can lead to system instability or crashes.
/// The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The flag parameter contains valid bit positions for the SCTLR
/// 3. Clearing the specified flags is appropriate for the current system state
/// 4. The operation won't conflict with other system requirements
///
/// Common use cases include:
/// - Disabling the MMU
/// - Disabling instruction or data caches
/// - Disabling branch prediction
/// - Changing alignment checking behavior
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Parameters
/// * `flag` - Bit flag(s) to clear in the SCTLR
///
/// # Assembly
///
/// ```asm
/// mrc p15, 0, {control}, c1, c0, 0  // Read current SCTLR value
/// mcr p15, 0, {control}, c1, c0, 0  // Write updated SCTLR value
/// ```
#[inline(always)]
pub unsafe fn clear_scltr_flag(flag: u32) {
    unsafe {
        let mut control: u32;
        asm!(
            "mrc p15, 0, {control}, c1, c0, 0",
            control = out(reg) control,
            options(nomem, nostack, preserves_flags)
        );
        control &= !flag;
        asm!(
            "mcr p15, 0, {control}, c1, c0, 0",
            control = in(reg) control,
            options(nomem, nostack, preserves_flags)
        );
    }
}

/// # Safety
/// This function enables the Memory Management Unit (MMU) by setting the M bit (bit 0)
/// in the SCTLR (System Control Register) of the ARM system control coprocessor.
/// Enabling the MMU activates virtual memory address translation. Incorrect usage can
/// lead to immediate system crashes or memory corruption. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. Valid translation tables are set up in TTBR0/TTBR1
/// 3. All necessary memory regions are properly mapped
/// 4. Any required TLB and cache maintenance operations are performed
/// 5. The system is prepared for the transition from physical to virtual addressing
///
/// After enabling the MMU, any memory accesses use virtual addresses that are
/// translated according to the configured page tables.
///
/// # Assembly
/// Sets bit 0 (M bit) in SCTLR
#[inline(always)]
pub unsafe fn mmu_enable() {
    unsafe {
        set_scltr_flag(0x1);
    }
}

/// # Safety
/// This function disables the Memory Management Unit (MMU) by clearing the M bit (bit 0)
/// in the SCTLR (System Control Register) of the ARM system control coprocessor.
/// Disabling the MMU deactivates virtual memory address translation. Incorrect usage can
/// lead to immediate system crashes or memory corruption. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The system is prepared for the transition from virtual to physical addressing
/// 3. Any code executing after this call is position-independent or properly relocated
/// 4. Any required cache maintenance operations are performed
///
/// After disabling the MMU, all memory accesses use physical addresses directly.
///
/// # Assembly
/// Clears bit 0 (M bit) in SCTLR
#[inline(always)]
pub unsafe fn mmu_disable() {
    unsafe {
        clear_scltr_flag(0x1);
    }
}

/// # Safety
/// This function enables the Data Cache by setting the C bit (bit 2) in the SCTLR
/// (System Control Register) of the ARM system control coprocessor. Enabling the
/// data cache allows the processor to store copies of data from memory in its cache
/// for faster access. Incorrect usage can lead to cache coherency issues. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. If the MMU is enabled, cache attributes in translation tables are properly set
/// 3. Proper cache maintenance operations are performed when necessary
/// 4. The system is prepared to handle cached data access
///
/// Common use cases include:
/// - Improving system performance by reducing memory access latency
/// - Part of the system initialization process
///
/// # Assembly
/// Sets bit 2 (C bit) in SCTLR
#[inline(always)]
pub unsafe fn d_cache_enable() {
    unsafe {
        set_scltr_flag(0x4);
    }
}

/// # Safety
/// This function disables the Data Cache by clearing the C bit (bit 2) in the SCTLR
/// (System Control Register) of the ARM system control coprocessor. Disabling the
/// data cache forces the processor to access data directly from memory. Incorrect usage
/// can lead to performance issues or data inconsistency. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. Any dirty cache lines are written back to memory (cache clean) before disabling
/// 3. The cache is invalidated if necessary
/// 4. The system can handle the performance impact of uncached data access
///
/// Common use cases include:
/// - System initialization or shutdown procedures
/// - When direct memory access is required without cache interference
/// - Before power-saving states
///
/// # Assembly
/// Clears bit 2 (C bit) in SCTLR
#[inline(always)]
pub unsafe fn d_cache_disable() {
    unsafe {
        clear_scltr_flag(0x4);
    }
}

/// # Safety
/// This function enables the Instruction Cache by setting the I bit (bit 12) in the SCTLR
/// (System Control Register) of the ARM system control coprocessor. Enabling the
/// instruction cache allows the processor to store copies of instructions from memory
/// in its cache for faster execution. Incorrect usage can lead to execution of stale
/// instructions. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. If the MMU is enabled, cache attributes in translation tables are properly set
/// 3. The instruction cache is invalidated before enabling if it contains stale data
/// 4. The system is prepared to handle cached instruction fetches
///
/// Common use cases include:
/// - Improving system performance by reducing instruction fetch latency
/// - Part of the system initialization process
///
/// # Assembly
/// Sets bit 12 (I bit) in SCTLR
#[inline(always)]
pub unsafe fn i_cache_enable() {
    unsafe {
        set_scltr_flag(0x1000);
    }
}

/// # Safety
/// This function disables the Instruction Cache by clearing the I bit (bit 12) in the SCTLR
/// (System Control Register) of the ARM system control coprocessor. Disabling the
/// instruction cache forces the processor to fetch instructions directly from memory.
/// Incorrect usage can lead to performance issues. The caller must ensure:
///
/// 1. The code runs in a privileged mode with access to CP15 registers
/// 2. The system can handle the performance impact of uncached instruction fetches
/// 3. Any required synchronization is performed
///
/// Common use cases include:
/// - System initialization or shutdown procedures
/// - Before updating code in memory
/// - Before entering certain debug states
/// - Before power-saving states
///
/// # Assembly
/// Clears bit 12 (I bit) in SCTLR
#[inline(always)]
pub unsafe fn i_cache_disable() {
    unsafe {
        clear_scltr_flag(0x1000);
    }
}

/// # Safety
/// This function uses raw assembly to execute an SVC (Supervisor Call) instruction.
/// The SVC instruction generates a supervisor call exception, which causes the processor
/// to enter supervisor mode and execute the corresponding exception handler. This is commonly
/// used to:
///
/// 1. Request privileged operations from an operating system or hypervisor
/// 2. Implement system calls in an OS environment
/// 3. Transition from user mode to a higher privilege level
/// 4. Request services from a trusted execution environment
///
/// The immediate value passed to SVC is typically used by the handler to determine
/// which service is being requested.
///
/// The function internally uses inline assembly that does not access memory or stack
/// and preserves processor flags.
///
/// # Parameters
/// * `num` - An 8-bit immediate value passed to the SVC instruction, identifying the
///   specific service or function being requested
///
/// # Assembly
///
/// ```asm
/// svc {immediate}
/// ```
#[inline(always)]
pub unsafe fn svc(num: u8) {
    unsafe {
        asm!(
            "svc {0}",
            in(reg) num,
            options(nomem, nostack, preserves_flags)
        );
    }
}
