# user-land-filesystem
The repository is mainly for course project, aiming at file system teaching process.

## 说明

在本次实验中结合 Rust 与 C++ 实现了一个暂时不完全的 EXT2 文件系统。

文件系统主要逻辑在 [chiro2001/rfs](https://github.com/chiro2001/rfs)，本 repo 仅是 rfs 的 c++ 调用移植。

### Roadmap

- [x] basic
  - [x] init
  - [x] destory
  - [x] lookup
  - [x] getattr
  - [ ] setattr -- `rfs.cpp` 中未完成
  - [x] mknod
  - [x] mkdir
  - [x] readdir
  - [ ] read -- 读写大于第二层遍历的文件(~16MiB)暂时未测试
  - [ ] write
- [ ] extra
  - [x] copy -- 通过添加 [abi-7-28](https://github.com/chiro2001/fuse-rs/commit/01c90c20d17a75d5ea092f29e5b33ac326736866) 完成
  - [ ] cache
  - [ ] journal

### 运行说明

首先将 `ddriver` 编译为静态库 `libddriver.a`，然后 Rust 部分的 `rfs_bind` 和 `libddriver.a` 一同编译，生成 `librfs_bind.a` 和 `lib_rfs_bind_lib.a`，最后将 `rfs.cpp` 与所有的静态库链接起来得到 `rfs_drv` 可执行文件。

有两种运行方式：

1. 直接运行 `rfs`，使用 Rust 内的 `FileDiskDriver`

   ```shell
   git clone https://github.com/chiro2001/rfs
   # 或者进入项目子目录
   # cd fs/rfs/rfs
   cd rfs
   cargo run -- -help
   ```

   ```shell
   $ cargo run -- --help
      Compiling rfs v0.1.0 (/home/chiro/os/fuse-ext2/fs/rfs/rfs)
       Finished dev [unoptimized + debuginfo] target(s) in 2.26s
        Running `target/debug/rfs --help`
   Usage: rfs [OPTIONS] [mountpoint]
   
   Arguments:
     [mountpoint]  Optional mountpoint to mount on [default: tests/mnt]
   
   Options:
     -f, --front          Keep daemon running in front
         --format         Format disk
         --mkfs           Use mkfs.ext2 to format disk
     -r, --read_only      Mount as read only filesystem
     -v, --verbose        Print more debug information, or set `RUST_LOG=debug`
     -d, --device <FILE>  Device path (filesystem storage file) [default: ddriver]
     -h, --help           Print help information
     -V, --version        Print version information
   $ 
   ```

   1. `-f`：阻塞运行，不另外 fork 新进程
   2. `--format`：强制创建新的文件系统
   3. `--mkfs`：创建文件系统的时候使用系统的 `mkfs.ext2` 而不是程序内的

   当存在环境变量为 `RUST_LOG=debug` 时输出更多调试信息。

   ```shell
   $ cargo run -- --mkfs -d disk ~/mnt   
       Finished dev [unoptimized + debuginfo] target(s) in 0.05s
        Running `target/debug/rfs --mkfs -d disk /home/chiro/mnt`
   [2022-11-30T12:28:55Z INFO  rfs] Device: disk
   [2022-11-30T12:28:55Z INFO  rfs] Daemon running at pid: 81716
   [2022-11-30T12:28:55Z INFO  rfs] [try 1/3] Mount to /home/chiro/mnt
   [2022-11-30T12:28:55Z INFO  fuse::session] Mounting /home/chiro/mnt
   [2022-11-30T12:28:55Z INFO  disk_driver::file] FileDrv open: disk                                                           
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] disk layout size: 4194304
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] disk unit size: 512
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] Disk disk has 8192 IO blocks.
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] disk info: DiskInfo { stats: DiskStats { write_cnt: 0, read_cnt: 0, seek_cnt: 0 }, consts: DiskConst { read_lat: 2, write_lat: 1, seek_lat: 4, track_num: 0, major_num: 100, layout_size: 4194304, iounit_size: 512 } }
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] super block size 2 disk block (1024 bytes)
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] FileSystem found!
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] fs stats: EXT2 1024 inodes, 1 KiB per block, free inodes 1013, free blocks 3950
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] fs layout:
   | BSIZE = 1024 B |
   | Boot(1) | Super(1) | GroupDesc(1) | DATA Map(1) | Inode Map(1) | Inode Table(128) | DATA(*) |
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] For inode bitmap, see @ 1000
   [2022-11-30T12:28:55Z INFO  rfs::rfs_lib] For  data bitmap, see @ c00
   $ echo a>~/mnt/aaaa
   $ fusermount -u ~/mnt
   [2022-11-30T12:29:11Z INFO  fuse::session] Unmounted /home/chiro/mnt
   [2022-11-30T12:29:11Z INFO  rfs] All Done.                                                                                  
   $ 
   ```

2. 运行 `rfs_drv`，使用 C++ 端的 `DDriver`

   ```shell
   cmake -B build -S ,
   cmake --build build
   ```

   ```shell
   $ ./build/rfs_drv --help
   usage: ./build/rfs_drv mountpoint [options]
   
   general options:
       -o opt,[opt...]        mount options
       -h   --help            print help
       -V   --version         print version
   
   FUSE options:
       -d   -o debug          enable debug output (implies -f)
       -f                     foreground operation
       -s                     disable multi-threaded operation
   
       -o allow_other         allow access to other users
       -o allow_root          allow access to root
       -o auto_unmount        auto unmount on process termination
       -o nonempty            allow mounts over non-empty file/dir
       -o default_permissions enable permission checking by kernel
       -o fsname=NAME         set filesystem name
       -o subtype=NAME        set filesystem type
       -o large_read          issue large read requests (2.4 only)
       -o max_read=N          set maximum size of read requests
   
       -o hard_remove         immediate removal (don't hide files)
       -o use_ino             let filesystem set inode numbers
       -o readdir_ino         try to fill in d_ino in readdir
       -o direct_io           use direct I/O
       -o kernel_cache        cache files in kernel
       -o [no]auto_cache      enable caching based on modification times (off)
       -o umask=M             set file permissions (octal)
       -o uid=N               set file owner
       -o gid=N               set file group
       -o entry_timeout=T     cache timeout for names (1.0s)
       -o negative_timeout=T  cache timeout for deleted names (0.0s)
       -o attr_timeout=T      cache timeout for attributes (1.0s)
       -o ac_attr_timeout=T   auto cache timeout for attributes (attr_timeout)
       -o noforget            never forget cached inodes
       -o remember=T          remember cached inodes for T seconds (0s)
       -o nopath              don't supply path if not necessary
       -o intr                allow requests to be interrupted
       -o intr_signal=NUM     signal to send on interrupt (10)
       -o modules=M1[:M2...]  names of modules to push onto filesystem stack
   
       -o max_write=N         set maximum size of write requests
       -o max_readahead=N     set maximum readahead
       -o max_background=N    set number of maximum background requests
       -o congestion_threshold=N  set kernel's congestion threshold
       -o async_read          perform reads asynchronously (default)
       -o sync_read           perform reads synchronously
       -o atomic_o_trunc      enable atomic open+truncate support
       -o big_writes          enable larger than 4kB writes
       -o no_remote_lock      disable remote file locking
       -o no_remote_flock     disable remote file locking (BSD)
       -o no_remote_posix_lock disable remove file locking (POSIX)
       -o [no_]splice_write   use splice to write to the fuse device
       -o [no_]splice_move    move data while splicing to the fuse device
       -o [no_]splice_read    use splice to read from the fuse device
   
   Module options:
   
   [iconv]
       -o from_code=CHARSET   original encoding of file names (default: UTF-8)
       -o to_code=CHARSET      new encoding of the file names (default: UTF-8)
   
   [subdir]
       -o subdir=DIR           prepend this directory to all paths (mandatory)
       -o [no]rellinks         transform absolute symlinks to relative
   ```



经过格式化后的磁盘文件可以被 [fuse-ext2](https://github.com/alperakcan/fuse-ext2) 识别并挂载。

```shell
$ file disk
disk: Linux rev 0.0 ext2 filesystem data, UUID=c30eff85-3ac3-4290-9c25-1a166e101635
$ fuse-ext2 disk ~/mnt -o rw+
$ ls ~/mnt -lahi
总计 18K
       1 drwxr-xr-x  3 root  root  1.0K 11月29日 15:40 .
12058626 drwx------ 96 chiro chiro 4.0K 11月30日 20:36 ..
       3 drwxr-xr-x  0 root  root  1.0K 11月30日 20:25 a
       4 -rw-r--r--  0 root  root     2 11月30日 20:29 aaaa
       2 drwx------  2 root  root   12K 11月29日 15:40 lost+found
$ fusermount -u ~/mnt
$ cargo run -- -d disk ~/mnt 
    Finished dev [unoptimized + debuginfo] target(s) in 0.05s
     Running `target/debug/rfs -d disk /home/chiro/mnt`
[2022-11-30T12:37:35Z INFO  rfs] Device: disk
[2022-11-30T12:37:35Z INFO  rfs] Daemon running at pid: 86525
[2022-11-30T12:37:35Z INFO  rfs] [try 1/3] Mount to /home/chiro/mnt
[2022-11-30T12:37:35Z INFO  fuse::session] Mounting /home/chiro/mnt
[2022-11-30T12:37:35Z INFO  disk_driver::file] FileDrv open: disk                                                           
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] disk layout size: 4194304
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] disk unit size: 512
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] Disk disk has 8192 IO blocks.
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] disk info: DiskInfo { stats: DiskStats { write_cnt: 0, read_cnt: 0, seek_cnt: 0 }, consts: DiskConst { read_lat: 2, write_lat: 1, seek_lat: 4, track_num: 0, major_num: 100, layout_size: 4194304, iounit_size: 512 } }
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] super block size 2 disk block (1024 bytes)
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] FileSystem found!
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] fs stats: EXT2 1024 inodes, 1 KiB per block, free inodes 1013, free blocks 3950
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] fs layout:
| BSIZE = 1024 B |
| Boot(1) | Super(1) | GroupDesc(1) | DATA Map(1) | Inode Map(1) | Inode Table(128) | DATA(*) |
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] For inode bitmap, see @ 1000
[2022-11-30T12:37:35Z INFO  rfs::rfs_lib] For  data bitmap, see @ c00
$ ls ~/mnt -lahi
总计 18K
       1 drwxr-xr-x  3 root  root  1.0K 11月29日 15:40 .
12058626 drwx------ 96 chiro chiro 4.0K 11月30日 20:37 ..
      97 drwxr-xr-x  0 root  root  1.0K 11月30日 20:25 a
     100 -rw-r--r--  0 root  root     2 11月30日 20:29 aaaa
      11 drwx------  2 root  root   12K 11月29日 15:40 lost+found
$ mount
// ...
rfs on /home/chiro/mnt type fuse (rw,nosuid,nodev,relatime,user_id=1000,group_id=1000)
$ fusermount -u ~/mnt
[2022-11-30T12:38:57Z INFO  fuse::session] Unmounted /home/chiro/mnt
[2022-11-30T12:38:57Z INFO  rfs] All Done.    
$ 
```

