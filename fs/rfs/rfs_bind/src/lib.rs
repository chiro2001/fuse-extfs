#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(deprecated)]

include!("../bindings.rs");

extern crate rfs;
extern crate core;

pub mod driver;
pub mod utils;

use std::mem::size_of;
use anyhow::{anyhow, Result};
use lazy_static::lazy_static;
use log::{debug, info, warn};
use mut_static::MutStatic;

use rfs::{DEVICE_FILE, FORCE_FORMAT, MKFS_FORMAT, MOUNT_POINT, RFS, RFSBase};
use rfs::desc::{EXT2_ROOT_INO, Ext2DirEntry, Ext2FileType, Ext2INode};
use rfs::desc::Ext2FileType::{Directory, RegularFile};
use rfs::utils::{deserialize_row, init_logs, serialize_row};
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
        pub fn wrfs_mkdir(path: &str, mode: usize) -> i32;
        pub fn wrfs_getattr(path: &str, rfs_stat: &mut [u8]) -> i32;
        pub fn wrfs_mknod(path: &str, mode: usize, dev: u32) -> i32;
        pub fn wrfs_readdir(path: &str, offset: i64, buf: &mut [u8]) -> i32;
    }
}

pub fn wrfs_init(file: &str) {
    println!("wrfs_init({})", file);
    init_logs();
    info!("log initiation done.");
    DRIVER.set(DDriver::new()).unwrap();
    BASE.set(RFSBase::default()).unwrap();
    // set static mutable values
    DEVICE_FILE.set(file.to_string().clone()).unwrap();
    MOUNT_POINT.set(format!("{}/ddriver", std::env::var("HOME").unwrap().to_string())).unwrap();
    FORCE_FORMAT.set(false).unwrap();
    MKFS_FORMAT.set(false).unwrap();
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
    info!("wrfs_mkdir(path={}, mode={:o}(0x{:x}))", path, mode, mode);
    wrfs_make_node(path, mode, Directory)
}

pub fn wrfs_mknod(path: &str, mode: usize, _dev: u32) -> i32 {
    info!("wrfs_mknod(path={}, mode={:o}(0x{:x}), _dev={})", path, mode, mode, _dev);
    wrfs_make_node(path, mode, RegularFile)
}

fn wrfs_make_node(path: &str, mode: usize, node_type: Ext2FileType) -> i32 {
    let mut fs = get_fs();
    let splits = path.split("/").collect::<Vec<&str>>()
        .into_iter().filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    let mut ino = EXT2_ROOT_INO;
    let mut name = splits.iter();
    // let mut inode: Ext2INode;
    let mut basename = "unknown";
    loop {
        let n = name.next();
        debug!("name.next = {:?}", n);
        let n = match n {
            Some(n) => n,
            None => "",
        };
        if n.is_empty() {
            if ino == EXT2_ROOT_INO {
                save_fs(fs);
                return -2;
            } else {
                break;
            }
        }
        basename = n.clone();
        match fs.rfs_lookup(ino, n) {
            Ok(r) => {
                ino = r.0;
                // inode = r.1;
            }
            Err(_) => break
        };
    }
    // let r = match name.next() {
    //     Some(n) => {
    //         fs.make_node(ino, n, mode, node_type).unwrap();
    //         0
    //     }
    //     None => -2
    // };
    let r = match fs.make_node(ino, basename, mode, node_type) {
        Ok(_) => 0,
        Err(_) => -2,
    };
    save_fs(fs);
    r
}

pub fn wrfs_getattr(path: &str, rfs_stat: &mut [u8]) -> i32 {
    let mut stat_struct: stat = unsafe { deserialize_row(rfs_stat) };
    let r = wrfs_getattr_inner(path, &mut stat_struct);
    rfs_stat.copy_from_slice(unsafe { serialize_row(&stat_struct) });
    r
}

pub fn wrfs_parse_path(fs: &mut RFS<DDriver>, path: &str) -> Result<(usize, Ext2INode)> {
    warn!("wrfs_parse_path(path={})", path);
    let mut ino = EXT2_ROOT_INO;
    if path == "/" {
        return Ok((EXT2_ROOT_INO, fs.get_inode(EXT2_ROOT_INO)?));
    }
    let splits = path.split("/").collect::<Vec<&str>>()
        .into_iter().filter(|x| !x.is_empty()).collect::<Vec<&str>>();
    let mut name_index = 0;
    let mut inode: Ext2INode = Default::default();
    let mut may_be_root = true;
    debug!("splits = {:?}", splits);
    loop {
        if name_index == splits.len() {
            if !may_be_root {
                break;
            }
        } else {
            match fs.rfs_lookup(ino, splits[name_index]) {
                Ok(r) => {
                    ino = r.0;
                    inode = r.1;
                }
                Err(e) => return Err(e)
            };
        }
        may_be_root = false;
        name_index += 1;
    };
    // not ROOT
    if ino == EXT2_ROOT_INO {
        Err(anyhow!("no such file"))
    } else {
        warn!("wrfs_parse_path found: ino={}, inode={:?}", ino, inode);
        Ok((ino, inode))
    }
}

pub fn wrfs_getattr_inner(path: &str, rfs_stat: &mut stat) -> i32 {
    let mut fs = get_fs();
    debug!("wrfs_getattr_inner(path={})", path);
    let ret = wrfs_parse_path(&mut fs, path);
    let mut r = 0;
    match ret {
        Ok((ino, inode)) => {
            debug!("got attr [{}] {:?}", ino, inode);
            // return attr
            let attr = inode.to_attr(ino);
            // what's this? device number?
            rfs_stat.st_dev = 0;
            rfs_stat.st_ino = attr.ino;
            rfs_stat.st_nlink = attr.nlink as u64;
            rfs_stat.st_mode = inode.i_mode as u32;
            rfs_stat.st_uid = attr.uid as u32;
            rfs_stat.st_gid = attr.gid as u32;
            rfs_stat.st_rdev = attr.rdev as u64;
            rfs_stat.st_size = attr.size as i64;
            rfs_stat.st_blksize = fs.block_size() as i64;
            rfs_stat.st_blocks = attr.blocks as i64;
            rfs_stat.st_atim = timespec { tv_sec: inode.i_atime as __time_t, tv_nsec: 0 };
            rfs_stat.st_mtim = timespec { tv_sec: inode.i_mtime as __time_t, tv_nsec: 0 };
            rfs_stat.st_ctim = timespec { tv_sec: inode.i_ctime as __time_t, tv_nsec: 0 };
        }
        Err(_) => {
            warn!("wrfs_getattr_inner({}) not found!", path);
            r = -2;
        }
    }
    save_fs(fs);
    r
}

pub fn wrfs_readdir(path: &str, offset: i64, buf: &mut [u8]) -> i32 {
    let v = match wrfs_readdir_inner(path, offset) {
        Ok(v) => v.into_iter()
            .map(|x| unsafe { serialize_row(&x) }.to_vec())
            .collect(),
        _ => { vec![] }
    };
    let r = v.len();
    for (i, e) in v.into_iter().enumerate() {
        buf[(i * size_of::<Ext2DirEntry>())..((i + 1) * size_of::<Ext2DirEntry>())]
            .copy_from_slice(&e);
    }
    r as i32
}

pub fn wrfs_readdir_inner(path: &str, offset: i64) -> Result<Vec<Ext2DirEntry>> {
    let mut fs = get_fs();
    let r = match wrfs_parse_path(&mut fs, path) {
        Ok((ino, _inode)) => {
            fs.rfs_readdir(ino as u64, offset)
        }
        Err(e) => Err(e)
    };
    save_fs(fs);
    r
}

#[cfg(test)]
mod tests {}