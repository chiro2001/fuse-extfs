extern crate rfs;
extern crate core;

pub mod driver;
pub mod utils;
mod bind;

use lazy_static::lazy_static;
use mut_static::MutStatic;
use std::sync::Mutex;
use rfs::disk_driver::DiskDriver;

use rfs::RFS;
use crate::driver::DDriver;

static mut FS: Option<Mutex<RFS>> = None;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        pub fn wrfs_init(file: &str);
    }
}

#[cfg(test)]
mod tests {
    use super::*;
}

pub fn wrfs_init(file: &str) {
    unsafe {
        let mut fs = RFS::new(Box::new(DDriver::new()));
        fs.rfs_init(file).unwrap();
        FS = Some(Mutex::new(fs));
    }
}