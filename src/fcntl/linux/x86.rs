#[repr(C)]
#[derive(Copy)]
pub struct flock {
    pub l_type: ::short_t,
    pub l_whence: ::short_t,
    pub l_start: ::sys::types::off_t,
    pub l_len: ::sys::types::off_t,
    pub l_pid: ::sys::types::pid_t,
}
new!(flock);
pub const F_DUPFD: ::int_t = 0;
pub const F_DUPFD_CLOEXEC: ::int_t = 1030;
pub const F_GETFD: ::int_t = 1;
pub const F_SETFD: ::int_t = 2;
pub const F_GETFL: ::int_t = 3;
pub const F_SETFL: ::int_t = 4;
pub const F_GETLK: ::int_t = 5;
pub const F_SETLK: ::int_t = 6;
pub const F_SETLKW: ::int_t = 7;
pub const F_GETOWN: ::int_t = 9;
pub const F_SETOWN: ::int_t = 8;
pub const FD_CLOEXEC: ::int_t = 1;
pub const F_RDLCK: ::int_t = 0;
pub const F_UNLCK: ::int_t = 2;
pub const F_WRLCK: ::int_t = 1;
pub const O_CLOEXEC: ::int_t = 0o2000000;
pub const O_CREAT: ::int_t = 0o100;
pub const O_DIRECTORY: ::int_t = 0o200000;
pub const O_EXCL: ::int_t = 0o200;
pub const O_NOCTTY: ::int_t = 0o400;
pub const O_NOFOLLOW: ::int_t = 0o400000;
pub const O_TRUNC: ::int_t = 0o1000;
pub const O_APPEND: ::int_t = 0o2000;
pub const O_DSYNC: ::int_t = 0o10000;
pub const O_NONBLOCK: ::int_t = 0o4000;
pub const O_RSYNC: ::int_t = 0o4010000;
pub const O_SYNC: ::int_t = 0o4010000;
pub const O_ACCMODE: ::int_t = 0o003;
pub const O_RDONLY: ::int_t = 0o0;
pub const O_RDWR: ::int_t = 0o2;
pub const O_WRONLY: ::int_t = 0o1;
pub const AT_FDCWD: ::int_t = 100;
pub const AT_EACCESS: ::int_t = 0x200;
pub const AT_SYMLINK_NOFOLLOW: ::int_t = 0x100;
pub const AT_SYMLINK_FOLLOW: ::int_t = 0x400;
pub const AT_REMOVEDIR: ::int_t = 0x200;
pub const POSIX_FADV_DONTNEED: ::int_t = 4;
pub const POSIX_FADV_NOREUSE: ::int_t = 5;
pub const POSIX_FADV_NORMAL: ::int_t = 0;
pub const POSIX_FADV_RANDOM: ::int_t = 1;
pub const POSIX_FADV_SEQUENTIAL: ::int_t = 2;
pub const POSIX_FADV_WILLNEED: ::int_t = 3;
