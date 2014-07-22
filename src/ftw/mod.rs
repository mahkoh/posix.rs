pub use self::os::{FTW};
pub use self::os::{FTW_F};
pub use self::os::{FTW_D};
pub use self::os::{FTW_DNR};
pub use self::os::{FTW_DP};
pub use self::os::{FTW_NS};
pub use self::os::{FTW_SL};
pub use self::os::{FTW_SLN};
pub use self::os::{FTW_PHYS};
pub use self::os::{FTW_MOUNT};
pub use self::os::{FTW_DEPTH};
pub use self::os::{FTW_CHDIR};

use {NTStr, int_t, char_t};
use sys::stat::{stat};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn ftw<T: NTStr>(path: &T,
                     fct: extern fn(*const char_t, *const stat, flag: int_t) -> int_t,
                     ndirs: int_t) -> int_t {
    extern {
        fn ftw(path: *const char_t,
               fct: extern fn(*const char_t, *const stat, flag: int_t) -> int_t,
               ndirs: int_t) -> int_t;
    }
    unsafe { ftw(path.as_ptr(), fct, ndirs) }
}

pub fn nftw<T: NTStr>(path: &T,
                      fct: extern fn(*const char_t, *const stat, int_t, *mut FTW),
                      fd_limit: int_t, flags: int_t) -> int_t {
    extern {
        fn nftw(path: *const char_t,
                fct: extern fn(*const char_t, *const stat, int_t, *mut FTW),
                fd_limit: int_t, flags: int_t) -> int_t;
    }
    unsafe { nftw(path.as_ptr(), fct, fd_limit, flags) }
}
