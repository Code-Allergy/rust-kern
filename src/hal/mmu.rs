use crate::panic;

use super::asm;
use core::cell::UnsafeCell;
use core::fmt;
use core::mem::size_of;

const VIRT_MEM_START: u32 = 0x8000_0000;
const VIRT_DRAM_START: u32 = 0x8000_0000;
const VIRT_DRAM_END: u32 = 0x9FFF_FFFF;

const SECTION_ADDR_MASK: u32 = 0xFFF0_0000;

const L1_SECTION_DESCRIPTOR: u32 = 0b10;

// Raw permission bits, needs to be shifted into place
const RAW_AP_RW_RW: u32 = 0b11;
const RAW_AP2_RW_RW: u32 = 0;

const L1_AP_SHIFT: u32 = 10;
const L1_AP2_SHIFT: u32 = 15;

const L2_AP_SHIFT: u32 = 4;
const L2_AP2_SHIFT: u32 = 9;

/// L1 AP bits for read/write access for KERN/USR
pub const L1_ACCESS_RW_RW: u32 = (RAW_AP_RW_RW << L1_AP_SHIFT) | (RAW_AP2_RW_RW << L1_AP2_SHIFT);

// #define MMU_SECTION_DESCRIPTOR (2 << 0)  // Section descriptor (b10)
// #define MMU_PAGE_DESCRIPTOR   (1 << 0)   // Page descriptor (b01)
// #define MMU_INVALID          (0 << 0)    // Invalid descriptor (b00)

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
        let imp = (value >> 9) & 1; // Bit 9: Implementation Defined
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

        if (supersection != 0) {
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
    let mut tables = get_boot_tables();
    let index = (virt_addr >> 20) as usize;
    &mut tables[index]
}

pub fn clear_boot_tables() {
    let tables = get_boot_tables();
    for entry in tables.iter_mut() {
        *entry = L1PageTableEntry(0);
    }
}

pub fn init() {
    clear_boot_tables();
    // Map the first 1MB of DRAM to the first 1MB of virtual memory
    // let tables = get_boot_tables();
    set_domains();
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

pub fn set_domains() {
    // for now, just enable domain 0 to client
    asm::set_dacr(0x1);
}
