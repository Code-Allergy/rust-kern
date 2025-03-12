#![allow(dead_code)]
use super::asm;
use core::cell::UnsafeCell;
use core::fmt;
use core::mem::size_of;

use crate::{dram, mmu};

const VIRT_MEM_START: u32 = 0x8000_0000;
const VIRT_DRAM_START: u32 = 0x8000_0000;
const VIRT_DRAM_END: u32 = 0x9FFF_FFFF;

const SECTION_ADDR_MASK: u32 = 0xFFF0_0000;

const L1_SECTION_DESCRIPTOR: u32 = 0b10;
const L1_PAGE_DESCRIPTOR: u32 = 0b01;

// Raw permission bits, needs to be shifted into place
const RAW_AP_NO_NO: u32 = 0b00;
const RAW_AP_RW_NO: u32 = 0b01;
const RAW_AP_RW_RO: u32 = 0b10;
const RAW_AP_RW_RW: u32 = 0b11;

const RAW_AP2_0: u32 = 0;
const RAW_AP2_1: u32 = 1;

// Raw tex bits, needs to be shifted into place
const RAW_TEX_XN: u32 = 0b000;
const RAW_TEX_XR: u32 = 0b001;
const RAW_TEX_XRW: u32 = 0b010;
const RAW_TEX_XRWB: u32 = 0b011;

const L1_AP_SHIFT: u32 = 10;
const L1_AP2_SHIFT: u32 = 15;

const L2_AP_SHIFT: u32 = 4;
const L2_AP2_SHIFT: u32 = 9;

pub const L1_SHAREABLE: u32 = 1 << 16;
pub const L1_CACHEABLE: u32 = 1 << 3;
pub const L1_NOT_GLOBAL: u32 = 1 << 17;
pub const L1_GLOBAL: u32 = 0 << 17;
pub const L1_NON_SECURE: u32 = 1 << 19;

pub const L1_ACCESS_NX: u32 = 1 << 4;
pub const L1_ACCESS_X: u32 = 0 << 4;

/// L1 AP bits for read/write access for KERN_USR
pub const L1_ACCESS_NO_NO: u32 = (RAW_AP_NO_NO << L1_AP_SHIFT) | (RAW_AP2_0 << L1_AP2_SHIFT);
pub const L1_ACCESS_RW_NO: u32 = (RAW_AP_RW_NO << L1_AP_SHIFT) | (RAW_AP2_0 << L1_AP2_SHIFT);
pub const L1_ACCESS_RW_RO: u32 = (RAW_AP_RW_RO << L1_AP_SHIFT) | (RAW_AP2_0 << L1_AP2_SHIFT);
pub const L1_ACCESS_RW_RW: u32 = (RAW_AP_RW_RW << L1_AP_SHIFT) | (RAW_AP2_0 << L1_AP2_SHIFT);
pub const L1_ACCESS_RO_NO: u32 = (RAW_AP_RW_NO << L1_AP_SHIFT) | (RAW_AP2_1 << L1_AP2_SHIFT);
pub const L1_ACCESS_RO_RO: u32 = (RAW_AP_RW_RW << L1_AP_SHIFT) | (RAW_AP2_1 << L1_AP2_SHIFT);

pub const L1_KERNEL_CODE_FLAGS: u32 =
    L1_ACCESS_RO_NO | L1_ACCESS_X | L1_SHAREABLE | L1_CACHEABLE | L1_GLOBAL;

pub const L1_KERNEL_DATA_FLAGS: u32 =
    L1_ACCESS_RW_NO | L1_ACCESS_NX | L1_SHAREABLE | L1_CACHEABLE | L1_GLOBAL;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct L1PageTableEntry(u32);

impl fmt::Debug for L1PageTableEntry {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let value = self.0;

        let section_type = value & 0b11; // Bits 1:0: Should be `0b10` for a section
        let b = (value >> 2) & 1; // Bit 2: Bufferable
        let c = (value >> 3) & 1; // Bit 3: Cacheable
        let xn = (value >> 4) & 1; // Bit 4: eXecute Never (XN)
        let domain = (value >> 5) & 0xF; // Bits 8:5: Domain
        let _imp = (value >> 9) & 1; // Bit 9: Implementation Defined
        let ap = (value >> 10) & 0b11; // Bits 11:10: AP
        let tex = (value >> 12) & 0b111; // Bits 14:12: TEX
        let ap2 = (value >> 15) & 0b1; // Bits 15: AP2
        let s = (value >> 16) & 1; // Bit 16: Shareable
        let n_g = (value >> 17) & 1; // Bit 17: Not Global
        let supersection = (value >> 18) & 1; // Bit 18: Supersection flag
        let ns = (value >> 19) & 1; // Bit 19: Non-Secure
        let section_base = value & 0xFFF00000; // Bits 31:20

        let type_str = match section_type {
            0b00 => "Invalid",
            0b01 => "Page",
            0b10 => "Section",
            _ => "Reserved",
        };

        if supersection != 0 {
            panic!("Supersection not supported, bit should not be set");
        }

        write!(
            f,
            "L1PageTableEntry {{ base: {:#010X}, B: {}, C: {}, AP: {:01b} {:02b}, TEX: {:03b}, \
             Domain: {}, nG: {}, S: {}, XN: {}, NS: {}, type: {} }}",
            section_base, b, c, ap2, ap, tex, domain, n_g, s, xn, ns, type_str
        )
    }
}

impl L1PageTableEntry {
    pub fn map_section(&mut self, phys_addr: u32, flags: u32) {
        let entry = (phys_addr & SECTION_ADDR_MASK) | L1_SECTION_DESCRIPTOR | flags;
        self.0 = entry;
    }

    // fn map_page(&mut self, phys_addr: u32, flags: u32) {
    //     let entry = phys_addr | flags;
    //     self.0 = entry;
    // }
}

pub fn get_boot_tables() -> &'static mut [L1PageTableEntry; 4096] {
    unsafe extern "C" {
        static _boot_tables_start: UnsafeCell<u8>;
        static _boot_tables_end: UnsafeCell<u8>;
    }
    // Calculate addresses safely
    let start_ptr = unsafe { _boot_tables_start.get() };
    let end_ptr = unsafe { _boot_tables_end.get() };

    // Convert to numerical addresses
    let start_addr = start_ptr as usize;
    let end_addr = end_ptr as usize;

    // Validate size (4096 entries * 4 bytes each = 16384 bytes)
    const EXPECTED_SIZE: usize = 4096 * size_of::<L1PageTableEntry>();
    assert_eq!(
        end_addr - start_addr,
        EXPECTED_SIZE,
        "Memory region size mismatch for page tables"
    );

    // Validate alignment (critical for MMU)
    assert_eq!(start_addr % 4096, 0, "Page tables misaligned");

    // Safety: We've verified the memory region is exclusive and properly sized
    unsafe { &mut *(start_addr as *mut [L1PageTableEntry; 4096]) }
}

pub fn get_boot_entry_at_virt(virt_addr: u32) -> &'static mut L1PageTableEntry {
    let tables = get_boot_tables();
    let index = (virt_addr >> 20) as usize;
    &mut tables[index]
}

pub fn clear_boot_tables() {
    let tables = get_boot_tables();
    for entry in tables.iter_mut() {
        *entry = L1PageTableEntry(0);
    }
}

// for now, just map everything and 1MB of kernel space
pub fn init(kernel_base: u32) {
    clear_boot_tables();
    set_domains();

    // for now, map everything, no caching
    let tables = get_boot_tables();
    for (i, entry) in tables.iter_mut().enumerate() {
        entry.map_section(i as u32 * 0x100000, L1_ACCESS_RW_RW);
    }

    // for now, map the first 1MB of kernel space to dram
    let first_page = mmu::get_boot_entry_at_virt(kernel_base);
    first_page.map_section(dram::DRAM_START as u32, mmu::L1_KERNEL_CODE_FLAGS);
}

pub fn enable() {
    unsafe {
        let page_tables = get_boot_tables();
        asm::set_ttbr0(page_tables as *const _ as u32);
        asm::flush_tlb();
        asm::flush_i_cache();
        asm::d_cache_enable();
        asm::i_cache_enable();
        asm::mmu_enable();
    }
}

pub fn disable() {
    unsafe {
        asm::mmu_disable();
        asm::d_cache_disable();
        asm::i_cache_disable();
    }
}

/// for now, just enables domain 0 to client
pub fn set_domains() {
    unsafe {
        asm::set_dacr(0x1);
    }
}
