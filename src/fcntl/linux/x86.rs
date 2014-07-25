#[repr(C)]
pub struct flock {
    pub l_type: ::short_t,
    pub l_whence: ::short_t,
    pub l_start: ::sys::types::off_t,
    pub l_len: ::sys::types::off_t,
    pub l_pid: ::sys::types::pid_t,
}
new!(flock)
pub static F_DUPFD: ::int_t = 0;
pub static F_DUPFD_CLOEXEC: ::int_t = 1030;
pub static F_GETFD: ::int_t = 1;
pub static F_SETFD: ::int_t = 2;
pub static F_GETFL: ::int_t = 3;
pub static F_SETFL: ::int_t = 4;
pub static F_GETLK: ::int_t = 5;
pub static F_SETLK: ::int_t = 6;
pub static F_SETLKW: ::int_t = 7;
pub static F_GETOWN: ::int_t = 9;
pub static F_SETOWN: ::int_t = 8;
pub static FD_CLOEXEC: ::int_t = 1;
pub static F_RDLCK: ::int_t = 0;
pub static F_UNLCK: ::int_t = 2;
pub static F_WRLCK: ::int_t = 1;
pub static O_CLOEXEC: ::int_t = 0o2000000;
pub static O_CREAT: ::int_t = 0o100;
pub static O_DIRECTORY: ::int_t = 0o200000;
pub static O_EXCL: ::int_t = 0o200;
pub static O_NOCTTY: ::int_t = 0o400;
pub static O_NOFOLLOW: ::int_t = 0o400000;
pub static O_TRUNC: ::int_t = 0o1000;
pub static O_APPEND: ::int_t = 0o2000;
pub static O_DSYNC: ::int_t = 0o10000;
pub static O_NONBLOCK: ::int_t = 0o4000;
pub static O_RSYNC: ::int_t = 0o4010000;
pub static O_SYNC: ::int_t = 0o4010000;
pub static O_ACCMODE: ::int_t = 0o003;
pub static O_RDONLY: ::int_t = 0o0;
pub static O_RDWR: ::int_t = 0o2;
pub static O_WRONLY: ::int_t = 0o1;
pub static AT_FDCWD: ::int_t = 100;
pub static AT_EACCESS: ::int_t = 0x200;
pub static AT_SYMLINK_NOFOLLOW: ::int_t = 0x100;
pub static AT_SYMLINK_FOLLOW: ::int_t = 0x400;
pub static AT_REMOVEDIR: ::int_t = 0x200;
pub static POSIX_FADV_DONTNEED: ::int_t = 4;
pub static POSIX_FADV_NOREUSE: ::int_t = 5;
pub static POSIX_FADV_NORMAL: ::int_t = 0;
pub static POSIX_FADV_RANDOM: ::int_t = 1;
pub static POSIX_FADV_SEQUENTIAL: ::int_t = 2;
pub static POSIX_FADV_WILLNEED: ::int_t = 3;
