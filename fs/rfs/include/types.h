#ifndef _TYPES_H_
#define _TYPES_H_

#include <ext2fs/ext2_fs.h>

struct custom_options {
  const char *device;
};

using rfs_super = struct ext2_super_block;
using rfs_dentry = struct ext2_dir_entry;

#endif /* _TYPES_H_ */