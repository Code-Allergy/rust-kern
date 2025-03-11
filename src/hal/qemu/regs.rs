pub mod base {
    pub const MMC0_BASE: u32 = 0x01C0F000;
}

pub mod mmc {
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
}
