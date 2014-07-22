pub use self::os::{stat};
pub use self::os::{S_IFMT};
pub use self::os::{S_IFBLK};
pub use self::os::{S_IFCHR};
pub use self::os::{S_IFIFO};
pub use self::os::{S_IFREG};
pub use self::os::{S_IFDIR};
pub use self::os::{S_IFLNK};
pub use self::os::{S_IFSOCK};
pub use self::os::{S_ISBLK};
pub use self::os::{S_ISCHR};
pub use self::os::{S_ISDIR};
pub use self::os::{S_ISFIFO};
pub use self::os::{S_ISREG};
pub use self::os::{S_ISLNK};
pub use self::os::{S_ISSOCK};
pub use self::os::{S_TYPEISMQ};
pub use self::os::{S_TYPEISSEM};
pub use self::os::{S_TYPEISSHM};
pub use self::os::{UTIME_NOW};
pub use self::os::{UTIME_OMIT};

use {NTStr, int_t, char_t};
use sys::types::{mode_t, dev_t};
use time::{timespec};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub static S_IRWXU: mode_t = 0o700;
pub static S_IRUSR: mode_t = 0o400;
pub static S_IWUSR: mode_t = 0o200;
pub static S_IXUSR: mode_t = 0o100;
pub static S_IRWXG: mode_t = 0o70;
pub static S_IRGRP: mode_t = 0o40;
pub static S_IWGRP: mode_t = 0o20;
pub static S_IXGRP: mode_t = 0o10;
pub static S_IRWXO: mode_t = 0o7;
pub static S_IROTH: mode_t = 0o4;
pub static S_IWOTH: mode_t = 0o2;
pub static S_IXOTH: mode_t = 0o1;
pub static S_ISUID: mode_t = 0o4000;
pub static S_ISGID: mode_t = 0o2000;
pub static S_ISVTX: mode_t = 0o1000;

pub fn chmod<T: NTStr>(file: &T, mode: mode_t) -> int_t {
    extern { fn chmod(file: *const char_t, mode: mode_t) -> int_t; }
    unsafe { chmod(file.as_ptr(), mode) }
}

pub fn fchmod(fd: int_t, mode: mode_t) -> int_t {
    extern { fn fchmod(fd: int_t, mode: mode_t) -> int_t; }
    unsafe { fchmod(fd, mode) }
}

pub fn fchmodat<T: NTStr>(fd: int_t, file: &T, mode: mode_t, flag: int_t) -> int_t {
    extern { fn fchmodat(fd: int_t, file: *const char_t, mode: mode_t, flag: int_t) -> int_t; }
    unsafe { fchmodat(fd, file.as_ptr(), mode, flag) }
}

pub fn fstat(fd: int_t, buf: &mut stat) -> int_t {
    extern { fn fstat(fd: int_t, buf: *mut stat) -> int_t; }
    unsafe { fstat(fd, buf as *mut _) }
}

pub fn fstatat<T: NTStr>(fd: int_t, file: &T, buf: &mut stat, flag: int_t) -> int_t {
    extern { fn fstatat(fd: int_t, file: *const char_t, buf: *mut stat, flag: int_t) -> int_t; }
    unsafe { fstatat(fd, file.as_ptr(), buf as *mut _, flag) }
}

pub fn futimens(fd: int_t, times: &mut timespec) -> int_t {
    extern { fn futimens(fd: int_t, times: *mut timespec) -> int_t; }
    unsafe { futimens(fd, times as *mut _) }
}

pub fn lstat<T: NTStr>(file: &T, buf: &mut stat) -> int_t {
    extern { fn lstat(file: *const char_t, buf: *mut stat) -> int_t; }
    unsafe { lstat(file.as_ptr(), buf as *mut _) }
}

pub fn mkdir<T: NTStr>(path: &T, mode: mode_t) -> int_t {
    extern { fn mkdir(path: *const char_t, mode: mode_t) -> int_t; }
    unsafe { mkdir(path.as_ptr(), mode) }
}

pub fn mkdirat<T: NTStr>(fd: int_t, path: &T, mode: mode_t) -> int_t {
    extern { fn mkdirat(fd: int_t, path: *const char_t, mode: mode_t) -> int_t; }
    unsafe { mkdirat(fd, path.as_ptr(), mode) }
}

pub fn mkfifo<T: NTStr>(path: &T, mode: mode_t) -> int_t {
    extern { fn mkfifo(path: *const char_t, mode: mode_t) -> int_t; }
    unsafe { mkfifo(path.as_ptr(), mode) }
}

pub fn mkfifoat<T: NTStr>(fd: int_t, path: &T, mode: mode_t) -> int_t {
    extern { fn mkfifoat(fd: int_t, path: *const char_t, mode: mode_t) -> int_t; }
    unsafe { mkfifoat(fd, path.as_ptr(), mode) }
}

pub fn mknod<T: NTStr>(path: &T, mode: mode_t, dev: dev_t) -> int_t {
    extern { fn mknod(path: *const char_t, mode: mode_t, dev: dev_t) -> int_t; }
    unsafe { mknod(path.as_ptr(), mode, dev) }
}

pub fn mknodat<T: NTStr>(fd: int_t, path: &T, mode: mode_t, dev: dev_t) -> int_t {
    extern { fn mknodat(fd: int_t, path: *const char_t, mode: mode_t, dev: dev_t) -> int_t; }
    unsafe { mknodat(fd, path.as_ptr(), mode, dev) }
}

pub fn stat<T: NTStr>(file: &T, buf: &mut stat) -> int_t {
    extern { fn stat(file: *const char_t, buf: *mut stat) -> int_t; }
    unsafe { stat(file.as_ptr(), buf as *mut _) }
}

pub fn umask(mask: mode_t) -> mode_t {
    extern { fn umask(mask: mode_t) -> mode_t; }
    unsafe { umask(mask) }
}

pub fn utimensat<T: NTStr>(fd: int_t, path: &T, times: &mut timespec, flags: int_t) -> int_t {
    extern { fn utimensat(fd: int_t, path: *const char_t, times: *mut timespec, flags: int_t) -> int_t; }
    unsafe { utimensat(fd, path.as_ptr(), times as *mut _, flags) }
}
