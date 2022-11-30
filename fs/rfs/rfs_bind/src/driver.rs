use std::default::Default;
use std::ffi::CString;
use std::os::raw::c_ulong;
use rfs::disk_driver::*;
use anyhow::{anyhow, Result};
use crate::{ddriver_seek, ddriver_read, ddriver_write, ddriver_open, ddriver_close, ddriver_ioctl};
use rfs::utils::*;
use crate::ret_ne;

#[derive(Default, Clone, Copy)]
pub struct DDriver {
    pub fd: i32,
}

impl DDriver {
    pub fn set(&mut self, d: Self) {
        self.fd = d.fd;
    }
}

impl DiskDriver for DDriver {
    fn ddriver_open(self: &mut Self, path: &str) -> Result<()> {
        self.fd = unsafe { ddriver_open(CString::new(path).unwrap().into_raw()) };
        ret_ne!("ddriver_open", self.fd)
    }

    fn ddriver_close(self: &mut Self) -> Result<()> {
        if self.fd != 0 { unsafe { ddriver_close(self.fd) }; }
        self.fd = 0;
        Ok(())
    }

    fn ddriver_seek(self: &mut Self, offset: i64, whence: SeekType) -> Result<u64> {
        Ok(unsafe { ddriver_seek(self.fd, offset, whence.to_int()) as u64 })
    }

    fn ddriver_write(self: &mut Self, buf: &[u8], size: usize) -> Result<usize> {
        Ok(unsafe { (ddriver_write(self.fd, SliceExt::cast_mut_force(buf).as_mut_ptr(), size)) as usize })
    }

    fn ddriver_read(self: &mut Self, buf: &mut [u8], size: usize) -> Result<usize> {
        Ok(unsafe { (ddriver_read(self.fd, SliceExt::cast_mut(buf).as_mut_ptr(), size)) as usize })
    }

    fn ddriver_ioctl(self: &mut Self, cmd: u32, arg: &mut [u8]) -> Result<()> {
        ret_ne!("ddriver_ioctl", unsafe { ddriver_ioctl(self.fd, cmd as c_ulong, SliceExt::cast_mut(arg).as_mut_ptr()) })
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
