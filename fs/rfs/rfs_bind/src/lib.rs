mod bind;

extern crate rfs;
extern crate core;

use std::ffi::CString;
use rfs::disk_driver::*;
use anyhow::{anyhow, Result};
use bind::{ddriver_seek, ddriver_read, ddriver_write, ddriver_open, ddriver_close, ddriver_ioctl};

pub struct DDriver;

macro_rules! generate_ret {
    ($n:expr, $e:expr) => {
        {
            let ret = $e;
            match ret {
                0 => Ok(()),
                _ => Err(anyhow!("{} $e returns error! value = {}", $n, ret))
            }
        }
    }
}

impl DiskDriver for DDriver {
    fn ddriver_open(self: &mut Self, path: &str) -> Result<()> {
        unsafe { generate_ret!("ddriver_open", ddriver_open(CString::new(path).unwrap().into_raw())) }
    }

    fn ddriver_close(self: &mut Self) -> Result<()> {
        todo!()
    }

    fn ddriver_seek(self: &mut Self, offset: i64, whence: SeekType) -> Result<u64> {
        todo!()
    }

    fn ddriver_write(self: &mut Self, buf: &[u8], size: usize) -> Result<usize> {
        todo!()
    }

    fn ddriver_read(self: &mut Self, buf: &mut [u8], size: usize) -> Result<usize> {
        todo!()
    }

    fn ddriver_ioctl(self: &mut Self, cmd: u32, arg: &mut [u8]) -> Result<()> {
        todo!()
    }

    fn ddriver_reset(self: &mut Self) -> Result<()> {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use anyhow::Result;

    #[test]
    fn bind_test() -> Result<()> {
        let mut driver = DDriver {};
        driver.ddriver_open("path")
    }
}
