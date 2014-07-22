pub use self::os::{flock};
pub use self::os::{F_DUPFD};
pub use self::os::{F_DUPFD_CLOEXEC};
pub use self::os::{F_GETFD};
pub use self::os::{F_SETFD};
pub use self::os::{F_GETFL};
pub use self::os::{F_SETFL};
pub use self::os::{F_GETLK};
pub use self::os::{F_SETLK};
pub use self::os::{F_SETLKW};
pub use self::os::{F_GETOWN};
pub use self::os::{F_SETOWN};
pub use self::os::{FD_CLOEXEC};
pub use self::os::{F_RDLCK};
pub use self::os::{F_UNLCK};
pub use self::os::{F_WRLCK};
pub use self::os::{O_CLOEXEC};
pub use self::os::{O_CREAT};
pub use self::os::{O_DIRECTORY};
pub use self::os::{O_EXCL};
pub use self::os::{O_NOCTTY};
pub use self::os::{O_NOFOLLOW};
pub use self::os::{O_TRUNC};
pub use self::os::{O_APPEND};
pub use self::os::{O_DSYNC};
pub use self::os::{O_NONBLOCK};
pub use self::os::{O_RSYNC};
pub use self::os::{O_SYNC};
pub use self::os::{O_ACCMODE};
pub use self::os::{O_RDONLY};
pub use self::os::{O_RDWR};
pub use self::os::{O_WRONLY};
pub use self::os::{AT_FDCWD};
pub use self::os::{AT_EACCESS};
pub use self::os::{AT_SYMLINK_NOFOLLOW};
pub use self::os::{AT_SYMLINK_FOLLOW};
pub use self::os::{AT_REMOVEDIR};
pub use self::os::{POSIX_FADV_DONTNEED};
pub use self::os::{POSIX_FADV_NOREUSE};
pub use self::os::{POSIX_FADV_NORMAL};
pub use self::os::{POSIX_FADV_RANDOM};
pub use self::os::{POSIX_FADV_SEQUENTIAL};
pub use self::os::{POSIX_FADV_WILLNEED};

use {NTStr, int_t, char_t};
use sys::types::{mode_t, off_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn creat<T: NTStr>(file: &T, mode: mode_t) -> int_t {
    extern { fn creat(file: *const char_t, mode: mode_t) -> int_t; }
    unsafe { creat(file.as_ptr(), mode) }
}

pub fn posix_fadvise(fd: int_t, off: off_t, len: off_t, advise: int_t) -> int_t {
    extern {
        fn posix_fadvise(fd: int_t, off: off_t, len: off_t, advise: int_t) -> int_t;
    }
    unsafe { posix_fadvise(fd, off, len, advise) }
}

pub fn posix_fallocate(fd: int_t, off: off_t, len: off_t) -> int_t {
    extern { fn posix_fallocate(fd: int_t, off: off_t, len: off_t) -> int_t; }
    unsafe { posix_fallocate(fd, off, len) }
}

extern "C" {
    pub fn fcntl(fd: int_t, cmd: int_t, ...) -> int_t;
    pub fn open(file: *const char_t, oflag: int_t, ...) -> int_t;
    pub fn openat(fd: int_t, file: *const char_t, oflag: int_t, ...) -> int_t;
}
