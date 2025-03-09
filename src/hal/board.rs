/// Get info about the current running board

pub const EEPROM_BOARD_HEADER_LEN: u32 = 4;
pub const EEPROM_BOARD_NAME_LEN: u32 = 8;
pub const EEPROM_BOARD_VERSION_LEN: u32 = 4;
pub const EEPROM_BOARD_SERIAL_LEN: u32 = 12;

pub const EEPROM_BOARD_INFO_LEN: u32 = EEPROM_BOARD_HEADER_LEN
    + EEPROM_BOARD_NAME_LEN
    + EEPROM_BOARD_SERIAL_LEN
    + EEPROM_BOARD_VERSION_LEN;

#[derive(Debug)]
pub struct BoardInfo {
    header: [u8; EEPROM_BOARD_HEADER_LEN as usize],
    name: [u8; EEPROM_BOARD_NAME_LEN as usize],
    serial: [u8; EEPROM_BOARD_SERIAL_LEN as usize],
    version: [u8; EEPROM_BOARD_VERSION_LEN as usize],
}

pub fn get_board_info() -> BoardInfo {
    platform::get_board_info()
}

// Platform-specific UART functions
#[cfg(feature = "qemu")]
mod platform {
    pub fn get_board_info() {}
}

#[cfg(feature = "bbb")]
mod platform {
    use super::*;
    use crate::hal::bbb::eeprom::*;
    pub fn get_board_info() -> BoardInfo {
        let mut info = BoardInfo {
            header: [0; EEPROM_BOARD_HEADER_LEN as usize],
            name: [0; EEPROM_BOARD_NAME_LEN as usize],
            serial: [0; EEPROM_BOARD_SERIAL_LEN as usize],
            version: [0; EEPROM_BOARD_VERSION_LEN as usize],
        };

        init(0x50);
        read(&mut info.header, EEPROM_BOARD_HEADER_LEN, 0);
        read(
            &mut info.name,
            EEPROM_BOARD_NAME_LEN,
            EEPROM_BOARD_HEADER_LEN,
        );
        read(
            &mut info.version,
            EEPROM_BOARD_VERSION_LEN,
            EEPROM_BOARD_HEADER_LEN + EEPROM_BOARD_NAME_LEN,
        );
        read(
            &mut info.serial,
            EEPROM_BOARD_SERIAL_LEN,
            EEPROM_BOARD_HEADER_LEN + EEPROM_BOARD_NAME_LEN + EEPROM_BOARD_VERSION_LEN,
        );

        info
    }
}
