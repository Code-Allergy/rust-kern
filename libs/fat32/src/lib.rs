#![no_std]
#![no_main]
#![cfg_attr(test, feature(custom_test_frameworks))]
#![cfg_attr(test, test_runner(crate::test::test_runner))]
#![cfg_attr(test, reexport_test_harness_main = "test_main")]

use core::mem::MaybeUninit;

mod test;
pub mod raw {
    #![allow(non_camel_case_types)]
    #![allow(non_snake_case)]
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

// use traits later down the line

// type Fat32DiskIOReadSector = extern "C" fn(u32, &mut [u8; 512]) -> i32;
type Fat32DiskIOReadSectorPtr = unsafe extern "C" fn(u32, *mut u8) -> i32;
type Fat32DiskIOWriteSector = fn(u32, &[u8; raw::FAT32_SECTOR_SIZE as usize]) -> i32;

pub type Fat32DiskIO = raw::fat32_diskio_t;
impl Fat32DiskIO {
    pub fn new(read: Fat32DiskIOReadSectorPtr, _write: Option<Fat32DiskIOWriteSector>) -> Self {
        Self {
            read_sector: Some(read),
        }
    }

    pub fn from_read_fn(read: Fat32DiskIOReadSectorPtr) -> Self {
        Self::new(read, None)
    }
}

#[derive(Debug)]
pub struct Fat32FileSystem {
    // Legacy C
    fs: raw::fat32_fs_t,
    // New Rust stuff
}

#[derive(Debug)]
pub struct Fat32File {
    file: raw::fat32_file_t,
}

impl Fat32File {
    pub fn open(fs: &mut Fat32FileSystem, path: &str) -> Result<Fat32File, Fat32Error> {
        fs.open_file(path)
    }

    pub fn read(&self, buffer: &mut [u8]) -> Result<usize, Fat32Error> {
        unsafe {
            let res = raw::fat32_read(
                &self.file as *const raw::fat32_file_t as *mut raw::fat32_file_t,
                buffer.as_mut_ptr() as *mut core::ffi::c_void,
                buffer.len() as i32,
            );

            if res < raw::FAT32_SUCCESS as i32 {
                return Err(Fat32Error::from(res));
            }

            Ok(res as usize)
        }
    }

    pub fn size(&self) -> u32 {
        self.file.file_size
    }

    pub fn seek(&mut self, offset: u32) -> Result<(), Fat32Error> {
        unsafe {
            let res = raw::fat32_seek(&mut self.file, offset as i32);

            if res != raw::FAT32_SUCCESS as i32 {
                return Err(Fat32Error::from(res));
            }

            Ok(())
        }
    }

    pub fn close(&mut self) -> Result<(), Fat32Error> {
        unsafe {
            let res = raw::fat32_close(&mut self.file);

            if res != raw::FAT32_SUCCESS as i32 {
                return Err(Fat32Error::from(res));
            }

            Ok(())
        }
    }
}

#[derive(Debug)]
pub enum Fat32Error {
    IOError = raw::FAT32_ERROR_IO as isize,
    InvalidBootSector = raw::FAT32_ERROR_INVALID_BOOT_SECTOR as isize,
    NoFile = raw::FAT32_ERROR_NO_FILE as isize,
    BadParam = raw::FAT32_ERROR_BAD_PARAMETER as isize,
    CorruptedFS = raw::FAT32_ERROR_CORRUPTED_FS as isize,
    NoSpace = raw::FAT32_ERROR_NO_SPACE as isize,
    NoDir = raw::FAT32_ERROR_NO_DIR as isize,
    NoPath = raw::FAT32_ERROR_NO_PATH as isize,
    IsDirNotFile = raw::FAT32_ERROR_IS_DIR as isize,
    NotDir = raw::FAT32_ERROR_NOT_DIR as isize,
    EndOfDir = raw::FAT32_ERROR_END_OF_DIR as isize,
    NoLabel = raw::FAT32_ERROR_NO_LABEL as isize,
    InvalidCluster = raw::FAT32_ERROR_INVALID_CLUSTER as isize,
}

// implement fat32 error from i32
impl From<i32> for Fat32Error {
    fn from(err: i32) -> Self {
        match err {
            raw::FAT32_ERROR_IO => Self::IOError,
            raw::FAT32_ERROR_INVALID_BOOT_SECTOR => Self::InvalidBootSector,
            raw::FAT32_ERROR_NO_FILE => Self::NoFile,
            raw::FAT32_ERROR_BAD_PARAMETER => Self::BadParam,
            raw::FAT32_ERROR_CORRUPTED_FS => Self::CorruptedFS,
            raw::FAT32_ERROR_NO_SPACE => Self::NoSpace,
            raw::FAT32_ERROR_NO_DIR => Self::NoDir,
            raw::FAT32_ERROR_NO_PATH => Self::NoPath,
            raw::FAT32_ERROR_IS_DIR => Self::IsDirNotFile,
            raw::FAT32_ERROR_NOT_DIR => Self::NotDir,
            raw::FAT32_ERROR_END_OF_DIR => Self::EndOfDir,
            raw::FAT32_ERROR_NO_LABEL => Self::NoLabel,
            raw::FAT32_ERROR_INVALID_CLUSTER => Self::InvalidCluster,
            _ => panic!("Unknown error code: {}", err),
        }
    }
}

impl Fat32FileSystem {
    pub fn mount(diskio: Fat32DiskIO) -> Result<Self, Fat32Error> {
        unsafe {
            let mut fs: raw::fat32_fs_t = core::mem::zeroed();
            let fs_ptr = &mut fs as *mut raw::fat32_fs_t;
            let diskio_cast = &diskio as *const Fat32DiskIO as *mut raw::fat32_diskio_t;

            let res = raw::fat32_mount(fs_ptr, diskio_cast);

            if res != raw::FAT32_SUCCESS as i32 {
                return Err(Fat32Error::from(res));
            }

            Ok(Self { fs })
        }
    }

    pub fn from_read_fn(read: Fat32DiskIOReadSectorPtr) -> Result<Self, Fat32Error> {
        let diskio = Fat32DiskIO::from_read_fn(read);
        Self::mount(diskio)
    }

    pub fn open_file(&mut self, filename: &str) -> Result<Fat32File, Fat32Error> {
        unsafe {
            // let mut buffer = [0u8; 256];
            // let bytes = filename.as_bytes();
            // buffer[..bytes.len()].copy_from_slice(bytes);
            // buffer[bytes.len()] = 0;

            let mut file: MaybeUninit<raw::fat32_file_t> = MaybeUninit::uninit();
            let filename_ptr = filename.as_ptr();
            let res = raw::fat32_open(&mut self.fs, filename_ptr, file.as_mut_ptr());

            if res != raw::FAT32_SUCCESS as i32 {
                return Err(Fat32Error::from(res));
            }

            Ok(Fat32File {
                file: file.assume_init(),
            })
        }
    }
}
