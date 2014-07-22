pub use self::os::{iovec};

use {int_t, ssize_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn readv(fd: int_t, iovec: &[iovec]) -> ssize_t {
    extern { fn readv(fd: int_t, iovec: *const iovec, count: int_t) -> ssize_t; }
    unsafe { readv(fd, iovec.as_ptr(), iovec.len() as int_t) }
}

pub fn writev(fd: int_t, iovec: &[iovec]) -> ssize_t {
    extern { fn writev(fd: int_t, iovec: *const iovec, count: int_t) -> ssize_t; }
    unsafe { writev(fd, iovec.as_ptr(), iovec.len() as int_t) }
}
