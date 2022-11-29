extern crate rfs;
extern crate core;

pub mod driver;
pub mod utils;
mod bind;

use lazy_static::lazy_static;
use mut_static::MutStatic;
use std::sync::Mutex;
use rfs::disk_driver::DiskDriver;

use rfs::{RFS, RFSBase};
use crate::driver::DDriver;

// static mut FS: Option<RFS> = None;
// static mut BASE: Option<RFSBase> = RFSBase::default();
// static mut DRIVER: Option<DDriver> = DDriver::default();
lazy_static! {
    // Store static mount point argument for signal call use
    pub static ref BASE: MutStatic<RFSBase> = MutStatic::new();
    pub static ref DRIVER: MutStatic<DDriver> = MutStatic::new();
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        pub fn wrfs_init(file: &str);
    }
}

// pub fn get_fs() -> &mut RFS {
//     unsafe { FS.unwrap().get_mut().unwrap() }
// }

pub fn get_fs() -> RFS<DDriver> {
    RFS::from_base(BASE.read().unwrap().clone(), DRIVER.read().unwrap().clone())
}

pub fn save_fs(fs: RFS<DDriver>) {
    DRIVER.write().unwrap().set(fs.driver);
    BASE.write().unwrap().set(fs.into());
}

pub fn wrfs_init(file: &str) {
    println!("wrfs_init({})", file);
    DRIVER.set(DDriver::new()).unwrap();
    BASE.set(RFSBase::default()).unwrap();
    let mut fs = get_fs();
    fs.rfs_init(file).unwrap();
    save_fs(fs);
}

#[cfg(test)]
mod tests {
}