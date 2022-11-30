#ifndef _TYPES_H_
#define _TYPES_H_

// #define MAX_NAME_LEN    128
#include <ext2fs/ext2_fs.h>

struct custom_options {
  const char *device;
};

// struct rfs_super {
//   uint32_t magic;
//   int fd;
//   /* TODO: Define yourself */
// };

// struct rfs_inode {
//   uint32_t ino;
//   /* TODO: Define yourself */
// };

// struct rfs_dentry {
//   char name[MAX_NAME_LEN];
//   uint32_t ino;
//   /* TODO: Define yourself */
// };

using rfs_super = struct ext2_super_block;
using rfs_dentry = struct ext2_dir_entry;

#endif /* _TYPES_H_ */