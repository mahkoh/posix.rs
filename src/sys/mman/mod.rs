pub use self::os::{PROT_EXEC};
pub use self::os::{PROT_NONE};
pub use self::os::{PROT_READ};
pub use self::os::{PROT_WRITE};
pub use self::os::{MAP_FIXED};
pub use self::os::{MAP_PRIVATE};
pub use self::os::{MAP_SHARED};
pub use self::os::{MS_ASYNC};
pub use self::os::{MS_INVALIDATE};
pub use self::os::{MS_SYNC};
pub use self::os::{MCL_CURRENT};
pub use self::os::{MCL_FUTURE};
pub use self::os::{MAP_FAILED};
pub use self::os::{POSIX_MADV_DONTNEED};
pub use self::os::{POSIX_MADV_NORMAL};
pub use self::os::{POSIX_MADV_RANDOM};
pub use self::os::{POSIX_MADV_SEQUENTIAL};
pub use self::os::{POSIX_MADV_WILLNEED};

use {void_t, size_t, int_t, NTStr, char_t};
use sys::types::{off_t, mode_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn mlock(addr: *const void_t, len: size_t) -> int_t {
    extern { fn mlock(addr: *const void_t, len: size_t) -> int_t; }
    unsafe { mlock(addr, len) }
}

pub fn mlockall(flags: int_t) -> int_t {
    extern { fn mlockall(flags: int_t) -> int_t; }
    unsafe { mlockall(flags) }
}

pub fn mmap(addr: *mut void_t, len: size_t, prot: int_t,
            flags: int_t, fd: int_t, offset: off_t) -> *mut void_t {
    extern { fn mmap(addr: *mut void_t, len: size_t, prot: int_t, flags: int_t,
                     fd: int_t, offset: off_t) -> *mut void_t; }
    unsafe { mmap(addr, len, prot, flags, fd, offset) }
}

pub fn mprotect(addr: *mut void_t, len: size_t, prot: int_t) -> int_t {
    extern { fn mprotect(addr: *mut void_t, len: size_t, prot: int_t) -> int_t; }
    unsafe { mprotect(addr, len, prot) }
}

pub fn msync(addr: *mut void_t, len: size_t, flags: int_t) -> int_t {
    extern { fn msync(addr: *mut void_t, len: size_t, flags: int_t) -> int_t; }
    unsafe { msync(addr, len, flags) }
}

pub fn munlock(addr: *const void_t, len: size_t) -> int_t {
    extern { fn munlock(addr: *const void_t, len: size_t) -> int_t; }
    unsafe { munlock(addr, len) }
}

pub fn munlockall() -> int_t {
    extern { fn munlockall() -> int_t; }
    unsafe { munlockall() }
}

pub fn munmap(addr: *mut void_t, len: size_t) -> int_t {
    extern { fn munmap(addr: *mut void_t, len: size_t) -> int_t; }
    unsafe { munmap(addr, len) }
}

pub fn posix_madvise(addr: *mut void_t, len: size_t, advice: int_t) -> int_t {
    extern { fn posix_madvise(addr: *mut void_t, len: size_t,
                              advice: int_t) -> int_t; }
    unsafe { posix_madvise(addr, len, advice) }
}

pub fn shm_open<T: NTStr>(name: &T, oflag: int_t,
                            mode: mode_t) -> int_t {
    extern { fn shm_open(name: *const char_t, oflag: int_t,
                         mode: mode_t) -> int_t; }
    unsafe { shm_open(name.as_ptr(), oflag, mode) }
}

pub fn shm_unlink<T: NTStr>(name: &T) -> int_t {
    extern { fn shm_unlink(name: *const char_t) -> int_t; }
    unsafe { shm_unlink(name.as_ptr()) }
}
