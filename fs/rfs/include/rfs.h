#ifndef _RFS_H_
#define _RFS_H_

#define FUSE_USE_VERSION 26

#include "stdio.h"
#include "stdlib.h"
#include <unistd.h>
#include "fcntl.h"
#include "string.h"
#include "fuse.h"
#include <stddef.h>
#include "ddriver.h"
#include "errno.h"
#include "types.h"

#define RFS_MAGIC                  /* TODO: Define by yourself */
#define RFS_DEFAULT_PERM    0777   /* 全权限打开 */

/******************************************************************************
* SECTION: rfs.c
*******************************************************************************/
void *rfs_init(struct fuse_conn_info *);

void rfs_destroy(void *);

int rfs_mkdir(const char *, mode_t);

int rfs_getattr(const char *, struct stat *);

int rfs_readdir(const char *, void *, fuse_fill_dir_t, off_t,
                struct fuse_file_info *);

int rfs_mknod(const char *, mode_t, dev_t);

int rfs_write(const char *, const char *, size_t, off_t,
              struct fuse_file_info *);

int rfs_read(const char *, char *, size_t, off_t,
             struct fuse_file_info *);

int rfs_access(const char *, int);

int rfs_unlink(const char *);

int rfs_rmdir(const char *);

int rfs_rename(const char *, const char *);

int rfs_utimens(const char *, const struct timespec tv[2]);

int rfs_truncate(const char *, off_t);

int rfs_open(const char *, struct fuse_file_info *);

int rfs_opendir(const char *, struct fuse_file_info *);

#endif  /* _rfs_H_ */