extern crate rfs;
extern crate core;

pub mod driver;
pub mod utils;
mod bind;

use std::ptr::read;
use lazy_static::lazy_static;
use mut_static::MutStatic;
use std::sync::Mutex;
use rfs::disk_driver::DiskDriver;

use rfs::{RFS, RFSBase};
use rfs::desc::{EXT2_ROOT_INO, Ext2FileType, Ext2INode};
use crate::driver::DDriver;

lazy_static! {
    pub static ref BASE: MutStatic<RFSBase> = MutStatic::new();
    pub static ref DRIVER: MutStatic<DDriver> = MutStatic::new();
}

pub fn get_fs() -> RFS<DDriver> {
    RFS::from_base(BASE.read().unwrap().clone(), DRIVER.read().unwrap().clone())
}

pub fn save_fs(fs: RFS<DDriver>) {
    DRIVER.write().unwrap().set(fs.driver);
    BASE.write().unwrap().set(fs.into());
}

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        pub fn wrfs_init(file: &str);
        pub fn wrfs_destroy();
    }
}

pub fn wrfs_init(file: &str) {
    println!("wrfs_init({})", file);
    DRIVER.set(DDriver::new()).unwrap();
    BASE.set(RFSBase::default()).unwrap();
    let mut fs = get_fs();
    fs.rfs_init(file).unwrap();
    save_fs(fs);
}

pub fn wrfs_destroy() {
    let mut fs = get_fs();
    fs.rfs_destroy().unwrap();
    save_fs(fs);
}

pub fn wrfs_mkdir(path: &str, mode: usize) -> i32 {
    let mut fs = get_fs();
    let splits = path.split("/").collect::<Vec<&str>>()
        .into_iter().filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    let mut ino = EXT2_ROOT_INO;
    let mut name = splits.iter();
    let mut inode: Ext2INode;
    loop {
        let n = match name.next() {
            Some(n) => n,
            None => "",
        };
        if n.is_empty() { return 1; }
        match fs.rfs_lookup(ino, n) {
            Ok(r) => {
                ino = r.0;
                inode = r.1;
            }
            Err(_) => break
        };
    }
    match name.next() {
        Some(n) => fs.make_node(ino, n, mode, Ext2FileType::Directory).unwrap(),
        None => return 1,
    };
    save_fs(fs);
    0
}

#[cfg(test)]
mod tests {}