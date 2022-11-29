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

static mut FS: Option<RFS> = None;

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

// pub fn get_fs() -> &mut RFS {
//     unsafe { FS.unwrap().get_mut().unwrap() }
// }

pub fn wrfs_init(file: &str) {
    println!("wrfs_init({})", file);
    unsafe {
        let mut fs = RFS::new(Box::new(DDriver::new()));
        fs.rfs_init(file).unwrap();
        // FS = Some(Mutex::new(fs));
        // FS = Mutex::new(fs);
        FS = Some(fs);
    }
    unsafe {
        // FS.unwrap().get_mut().unwrap().rfs_init("test").unwrap();
        // FS.unwrap().rfs_init("test").unwrap();
    }
}

