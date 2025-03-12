use crate::qemu::regs::{base::MMC0_BASE, mmc::*};

use crate::mmc::MMCError;
use crate::util::{reg32_read, reg32_read_masked, reg32_write, reg32_write_masked};

pub fn read_sector(sector: u32, buffer: &mut [u8; 512]) -> Result<(), MMCError> {
    mmc_send_cmd(17, sector * 512)?;
    let buffer_ptr = buffer.as_mut_ptr() as *mut u32; // cast to u32 ptr to read 32 bits at a time
    unsafe {
        for i in 0..128 {
            *buffer_ptr.add(i) = reg32_read(MMC0_BASE, MMC_FIFO);
        }
    }

    Ok(())
}

pub fn mmc_send_cmd(cmd: u32, arg: u32) -> Result<(), MMCError> {
    let cmd_flags = match cmd {
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
            Err(MMCError::NoResponse)
        } else {
            Ok(())
        }
    }
}

pub fn init() -> Result<(), MMCError> {
    unsafe {
        reg32_write_masked(MMC0_BASE, MMC_GCTRL, SD_GCTL_SOFT_RST, SD_GCTL_SOFT_RST);
        while reg32_read_masked(MMC0_BASE, MMC_GCTRL, SD_GCTL_SOFT_RST) == 1 {}

        reg32_write(MMC0_BASE, MMC_CLKCR, 59 | (1 << 16)); // 24MHz/(59+1) = 400kHz

        mmc_send_cmd(0, 0)?;
        mmc_send_cmd(8, 0x1AA)?;
        let resp = reg32_read(MMC0_BASE, MMC_RESP0);
        if (resp & 0xFF) != 0xAA || (resp >> 8) != 0x1 {
            return Err(MMCError::BadCMD8Response);
        }

        loop {
            mmc_send_cmd(55, 0)?;
            mmc_send_cmd(41, 0x40FF8000)?;
            let resp = reg32_read(MMC0_BASE, MMC_RESP0);
            if (resp & (1 << 31)) != 0 {
                break;
            }
        }

        mmc_send_cmd(2, 0)?;
        mmc_send_cmd(3, 0)?;
        let rca = (reg32_read(MMC0_BASE, MMC_RESP0) >> 16) & 0xFFFF;
        mmc_send_cmd(7, rca << 16)?;

        reg32_write(MMC0_BASE, MMC_CLKCR, 1 << 16);
        reg32_write(MMC0_BASE, MMC_IDIE, (1 << 4) | (1 << 3));
        reg32_write(MMC0_BASE, MMC_BLKSZ, 512);
    }

    Ok(())
}
