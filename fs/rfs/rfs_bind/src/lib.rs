mod bind;

extern crate rfs;
extern crate core;

use std::default::Default;
use std::ffi::CString;
use std::os::raw::{c_char, c_ulong, c_void};
use rfs::disk_driver::*;
use anyhow::{anyhow, Result};
use bind::{ddriver_seek, ddriver_read, ddriver_write, ddriver_open, ddriver_close, ddriver_ioctl};
use rfs::utils::*;

#[derive(Default)]
pub struct DDriver {
    pub fd: i32,
    // pub a: Vec<>
}

macro_rules! ret {
    ($n:expr, $e:expr, $expected:expr) => {
        {
            let ret = $e;
            match ret {
                $expected => Ok(()),
                _ => Err(anyhow!("{} returns error! value = {}", $n, ret))
            }
        }
    };
    ($n:expr, $e:expr) => {
        ret!($n, $e, 0)
    }
}
macro_rules! ret_ne {
    ($n:expr, $e:expr) => {
        {
            let ret = $e;
            if ret != 0 { Ok(()) } else { Err(anyhow!("{} returns error! value = {}", $n, ret)) }
        }
    }
}

impl DiskDriver for DDriver {
    fn ddriver_open(self: &mut Self, path: &str) -> Result<()> {
        unsafe {
            self.fd = ddriver_open(CString::new(path).unwrap().into_raw());
            ret_ne!("ddriver_open", self.fd)
        }
    }

    fn ddriver_close(self: &mut Self) -> Result<()> {
        self.fd = 0;
        unsafe {
            ddriver_close(self.fd);
        };
        Ok(())
    }

    fn ddriver_seek(self: &mut Self, offset: i64, whence: SeekType) -> Result<u64> {
        unsafe {
            Ok(ddriver_seek(self.fd, offset, whence.to_int()) as u64)
        }
    }

    fn ddriver_write(self: &mut Self, buf: &[u8], size: usize) -> Result<usize> {
        unsafe {
            Ok((ddriver_write(self.fd, SliceExt::cast_mut_force(buf).as_mut_ptr(), size)) as usize)
        }
    }

    fn ddriver_read(self: &mut Self, buf: &mut [u8], size: usize) -> Result<usize> {
        unsafe {
            Ok((ddriver_read(self.fd, SliceExt::cast_mut(buf).as_mut_ptr(), size)) as usize)
        }
    }

    fn ddriver_ioctl(self: &mut Self, cmd: u32, arg: &mut [u8]) -> Result<()> {
        unsafe {
            ret_ne!("ddriver_ioctl", ddriver_ioctl(self.fd, cmd as c_ulong, SliceExt::cast_mut(arg).as_mut_ptr()))
        }
    }

    fn ddriver_reset(self: &mut Self) -> Result<()> {
        self.fd = 0;
        Ok(())
    }
}

impl DDriver {
    pub fn new() -> Self {
        Self::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn bind_test() -> Result<()> {
        const MEM_DISK_SIZE: usize = 4 * 0x400 * 0x400;
        const MEM_DISK_UNIT: usize = 512;

        let mut driver = DDriver::new();
        driver.ddriver_open("/home/chiro/ddriver")?;
        let write_data = [0x55 as u8; MEM_DISK_UNIT];
        driver.ddriver_write(&write_data, MEM_DISK_UNIT)?;
        driver.ddriver_seek(0, SeekType::Set)?;
        let mut read_data = [0 as u8; MEM_DISK_UNIT];
        driver.ddriver_read(&mut read_data, MEM_DISK_UNIT)?;
        assert_eq!(read_data, write_data);
        driver.ddriver_close()?;
        Ok(())
    }
}
