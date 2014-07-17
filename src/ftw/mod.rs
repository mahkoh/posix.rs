extern crate libc;

pub use self::arch::{FTW, FTW_F, FTW_D, FTW_DNR, FTW_DP, FTW_NS, FTW_SL, FTW_SLN};
pub use self::arch::{FTW_PHYS, FTW_MOUNT, FTW_DEPTH, FTW_CHDIR};

use sys::stat::{stat};

#[cfg(target_os = "linux")]
#[cfg(target_os = "android")]
#[path = "linux.rs"]
mod arch;

pub fn ftw<T: ::NTStr>(path: &T,
                       fct: extern fn(*const ::char_t, *const stat, flag: ::int_t) -> ::int_t,
                       ndirs: ::int_t) -> ::int_t {
    extern {
        fn ftw(path: *const ::char_t,
               fct: extern fn(*const ::char_t, *const stat, flag: ::int_t) -> ::int_t,
               ndirs: ::int_t) -> ::int_t;
    }
    unsafe { ftw(path.as_ptr(), fct, ndirs) }
}

pub fn nftw<T: ::NTStr>(path: &T,
                        fct: extern fn(*const ::char_t, *const stat, ::int_t, *mut FTW),
                        fd_limit: ::int_t, flags: ::int_t) -> ::int_t {
    extern {
        fn nftw(path: *const ::char_t,
                fct: extern fn(*const ::char_t, *const stat, ::int_t, *mut FTW),
                fd_limit: ::int_t, flags: ::int_t) -> ::int_t;
    }
    unsafe { nftw(path.as_ptr(), fct, fd_limit, flags) }
}
