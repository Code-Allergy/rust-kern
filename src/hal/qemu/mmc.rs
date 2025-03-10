use crate::{
    hal::{
        asm,
        util::{reg32_read, reg32_read_masked, reg32_write, reg32_write_masked},
    },
    panic,
};

pub const MMC0_BASE: u32 = 0x01C0F000;

pub const MMC_GCTRL: u32 = 0x00; // Global Control
pub const MMC_CLKCR: u32 = 0x04; // Clock Control
pub const MMC_TIMEOUT: u32 = 0x08; // Timeout
pub const MMC_WIDTH: u32 = 0x0C; // Bus Width
pub const MMC_BLKSZ: u32 = 0x10; // Block Size
pub const MMC_BYTECNT: u32 = 0x14; // Byte Count
pub const MMC_CMD: u32 = 0x18; // Command
pub const MMC_ARG: u32 = 0x1C; // Argument
pub const MMC_RESP0: u32 = 0x20; // Response 0
pub const MMC_RESP1: u32 = 0x24; // Response 1
pub const MMC_RESP2: u32 = 0x28; // Response 2
pub const MMC_RESP3: u32 = 0x2C; // Response 3
pub const MMC_IMASK: u32 = 0x30; // Interrupt Mask
pub const MMC_MINT: u32 = 0x34; // Masked Interrupt Status
pub const MMC_RINT: u32 = 0x38; // Raw Interrupt Status
pub const MMC_STATUS: u32 = 0x3C; // Status
pub const MMC_FTRGLEVEL: u32 = 0x40; // FIFO Water Level
pub const MMC_FUNCSEL: u32 = 0x44; // Function Select
pub const MMC_CBCR: u32 = 0x48; // CIU Byte Count
pub const MMC_BBCR: u32 = 0x4C; // BIU Byte Count
pub const MMC_DBGC: u32 = 0x50; // Debug Enable
pub const MMC_DMAC: u32 = 0x5C; // DMA Control
pub const MMC_DLBA: u32 = 0x60; // Descriptor List Base Address
pub const MMC_IDST: u32 = 0x64; // Internal DMA Status
pub const MMC_IDIE: u32 = 0x68; // Internal DMA Interrupt Enable
pub const MMC_CHDA: u32 = 0x6C; // Current Host Descriptor Address
pub const MMC_CBDA: u32 = 0x70; // Current Buffer Descriptor Address
pub const MMC_FIFO: u32 = 0x200; // FIFO

// other
pub const SD_RISR_CMD_COMPLETE: u32 = 1 << 2;
pub const SD_RISR_NO_RESPONSE: u32 = 1 << 1;

pub const SD_GCTL_SOFT_RST: u32 = 1 << 0;

// Command flags (SD_CMDR)
pub const SD_CMDR_DATA: u32 = 1 << 5;
pub const SD_CMDR_WRITE: u32 = 0 << 4; // Write direction
pub const SD_CMDR_READ: u32 = 1 << 4; // Read direction
pub const SD_CMDR_AUTOSTOP: u32 = 1 << 12;
pub const SD_CMDR_LOAD: u32 = 1 << 31;

pub const SD_CMDR_NO_RESP: u32 = 0 << 6;
pub const SD_CMDR_SHORT_RESP: u32 = 1 << 6;
pub const SD_CMDR_LONG_RESP: u32 = 1 << 7;

pub const SD_STA_DATA_REQ: u32 = 0x8;
pub const SD_STA_FIFO_FULL: u32 = 0x10;
pub const SD_STA_FIFO_EMPTY: u32 = 0x20;

pub const SD_RISR_DATA_COMPLETE: u32 = 1 << 3;

// DMA Status flags (SD_IDST)
pub const SD_IDST_INT_SUMMARY: u32 = 1 << 8;
pub const SD_IDST_RECEIVE_IRQ: u32 = 1 << 1;

pub const SD_GCTL_DMA_ENB: u32 = 1 << 5;
pub const SD_GCTL_DMA_RST: u32 = 1 << 2;

pub const DESC_STATUS_HOLD: u32 = 1 << 31;
pub const DESC_STATUS_ERROR: u32 = 1 << 30;
pub const DESC_STATUS_LAST: u32 = 1 << 2;

pub fn read_sector(sector: u32, buffer: &mut [u8; 512]) -> Result<(), ()> {
    mmc_send_cmd(17, sector * 512).expect("Failed to send read command");

    // Read 4 bytes at a time
    let buffer_ptr = buffer.as_mut_ptr() as *mut u32;
    unsafe {
        for i in 0..128 {
            *buffer_ptr.add(i) = reg32_read(MMC0_BASE, MMC_FIFO);
        }
    }

    Ok(())
}

pub fn mmc_send_cmd(cmd: u32, arg: u32) -> Result<(), ()> {
    let mut cmd_flags = match cmd {
        0 => SD_CMDR_NO_RESP,
        2 => SD_CMDR_LONG_RESP,
        3 => SD_CMDR_SHORT_RESP,
        7 => SD_CMDR_SHORT_RESP,
        8 => SD_CMDR_SHORT_RESP,
        9 => SD_CMDR_LONG_RESP,
        12 => SD_CMDR_SHORT_RESP,
        13 => SD_CMDR_SHORT_RESP,
        16 => SD_CMDR_SHORT_RESP,
        17 => SD_CMDR_SHORT_RESP | SD_CMDR_READ,
        24 => SD_CMDR_SHORT_RESP | SD_CMDR_WRITE,
        25 => SD_CMDR_SHORT_RESP,
        41 => SD_CMDR_SHORT_RESP,
        55 => SD_CMDR_SHORT_RESP,
        _ => {
            if cmd > 55 {
                panic!("Invalid command number: {}", cmd);
            }
            println!("Unknown command: {}", cmd); // WARN
            SD_CMDR_SHORT_RESP
        }
    };

    unsafe {
        reg32_write(MMC0_BASE, MMC_ARG, arg);
        reg32_write(MMC0_BASE, MMC_CMD, cmd & 0x3F | cmd_flags | SD_CMDR_LOAD);

        while reg32_read_masked(
            MMC0_BASE,
            MMC_RINT,
            SD_RISR_CMD_COMPLETE | SD_RISR_NO_RESPONSE,
        ) == 0
        {}

        if (reg32_read(MMC0_BASE, MMC_RINT) & SD_RISR_NO_RESPONSE) != 0 {
            Err(())
        } else {
            Ok(())
        }
    }
}

pub fn init() {
    unsafe {
        reg32_write_masked(MMC0_BASE, MMC_GCTRL, SD_GCTL_SOFT_RST, SD_GCTL_SOFT_RST);
        while reg32_read_masked(MMC0_BASE, MMC_GCTRL, SD_GCTL_SOFT_RST) == 1 {}

        reg32_write_masked(MMC0_BASE, MMC_GCTRL, 1 << 1, 1 << 1);
        reg32_write(MMC0_BASE, MMC_CLKCR, (59 << 0) | (1 << 16)); // 24MHz/(59+1) = 400kHz

        mmc_send_cmd(0, 0).expect("Failed to send initialization CMD0");
        match mmc_send_cmd(8, 0x1AA) {
            Ok(()) => {
                let resp = reg32_read(MMC0_BASE, MMC_RESP0);
                if (resp & 0xFF) != 0xAA || (resp >> 8) != 0x1 {
                    panic!("Invalid CMD8 response");
                }
            }
            Err(_) => panic!("CMD8 failed - card not SDv2 (or not present)"),
        }

        // wait until ready
        loop {
            mmc_send_cmd(55, 0).expect("CMD55 failed");
            mmc_send_cmd(41, 0x40FF8000).expect("ACMD41 failed");
            let resp = reg32_read(MMC0_BASE, MMC_RESP0);
            if (resp & (1 << 31)) != 0 {
                break;
            }
        }

        mmc_send_cmd(2, 0);
        mmc_send_cmd(3, 0);
        let rca = (reg32_read(MMC0_BASE, MMC_RESP0) >> 16) & 0xFFFF;
        mmc_send_cmd(7, rca << 16);

        reg32_write(MMC0_BASE, MMC_CLKCR, (0 << 0) | (1 << 16));
        reg32_write(MMC0_BASE, MMC_IDIE, (1 << 4) | (1 << 3));
        reg32_write(MMC0_BASE, MMC_BLKSZ, 512);

        // trying to read first sector
        let mut buffer = [0u8; 512];
        read_sector(0, &mut buffer).expect("Failed to read sector 0");
        // check magic number
        //

        if buffer[510] != 0x55 || buffer[511] != 0xAA {
            panic!("Invalid MBR magic number");
        } else {
            println!("MBR magic number is valid, read off disk successful!");
        }
    }
}
