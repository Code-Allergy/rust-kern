use core::arch::asm;

pub unsafe fn get_dacr() -> u32 {
    let dacr: u32;
    asm!(
        "mrc p15, 0, {dacr}, c3, c0, 0",
        dacr = out(reg) dacr,
        options(nomem, nostack, preserves_flags)
    );
    dacr
}

pub unsafe fn set_dacr(dacr: u32) {
    unsafe {
        asm!(
            "mcr p15, 0, {dacr}, c3, c0, 0",
            dacr = in(reg) dacr,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub unsafe fn wfi() {
    unsafe {
        asm!("wfi", options(nomem, nostack, preserves_flags));
    }
}

pub unsafe fn dsb() {
    unsafe {
        asm!("dsb", options(nomem, nostack, preserves_flags));
    }
}

pub unsafe fn isb() {
    unsafe {
        asm!("isb", options(nomem, nostack, preserves_flags));
    }
}

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

pub unsafe fn set_ttbr0(ttbr0: u32) {
    unsafe {
        asm!(
            "mcr p15, 0, {ttbr0}, c2, c0, 0",
            ttbr0 = in(reg) ttbr0,
            options(nomem, nostack, preserves_flags)
        );
    }
}

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

pub unsafe fn set_ttbr1(ttbr1: u32) {
    unsafe {
        asm!(
            "mcr p15, 0, {ttbr1}, c2, c0, 1",
            ttbr1 = in(reg) ttbr1,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub unsafe fn flush_tlb() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c8, c7, 0",
            in(reg) 0,
            options(nomem, nostack, preserves_flags)
        );
    }
}

pub unsafe fn flush_i_cache() {
    unsafe {
        asm!(
            "mcr p15, 0, {0}, c7, c5, 0",
            in(reg) 0,
            options(nomem, nostack, preserves_flags)
        );
    }
}

unsafe fn set_scltr_flag(flag: u32) {
    let mut control: u32;
    asm!(
        "mrc p15, 0, {control}, c1, c0, 0",
        control = out(reg) control,
        options(nomem, nostack, preserves_flags)
    );
    control |= 0x1;
    asm!(
        "mcr p15, 0, {control}, c1, c0, 0",
        control = in(reg) control,
        options(nomem, nostack, preserves_flags)
    );
}

unsafe fn clear_scltr_flag(flag: u32) {
    let mut control: u32;
    asm!(
        "mrc p15, 0, {control}, c1, c0, 0",
        control = out(reg) control,
        options(nomem, nostack, preserves_flags)
    );
    control &= !0x1;
    asm!(
        "mcr p15, 0, {control}, c1, c0, 0",
        control = in(reg) control,
        options(nomem, nostack, preserves_flags)
    );
}

pub unsafe fn mmu_enable() {
    unsafe {
        set_scltr_flag(0x1);
    }
}

pub unsafe fn mmu_disable() {
    unsafe {
        clear_scltr_flag(0x1);
    }
}

pub unsafe fn d_cache_enable() {
    unsafe {
        set_scltr_flag(0x4);
    }
}

pub unsafe fn d_cache_disable() {
    unsafe {
        clear_scltr_flag(0x4);
    }
}

pub unsafe fn i_cache_enable() {
    unsafe {
        set_scltr_flag(0x1000);
    }
}

pub unsafe fn i_cache_disable() {
    unsafe {
        clear_scltr_flag(0x1000);
    }
}
