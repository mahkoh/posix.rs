pub use self::os::{timeval};
pub use self::os::{itimerval};
pub use self::os::{ITIMER_REAL};
pub use self::os::{ITIMER_VIRTUAL};
pub use self::os::{ITIMER_PROF};

use {int_t, void_t, char_t, NTStr};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn getitimer(which: int_t, value: &mut itimerval) -> int_t {
    extern { fn getitimer(which: int_t, value: *mut itimerval) -> int_t; }
    unsafe { getitimer(which, value as *mut _) }
}

pub fn gettimeofday(tv: &mut timeval) -> int_t {
    extern { fn gettimeofday(tv: *mut timeval, tz: *mut void_t) -> int_t; }
    unsafe { gettimeofday(tv as *mut _, 0 as *mut _) }
}

pub fn setitimer(which: int_t, new: &itimerval,
                 old: Option<&mut itimerval>) -> int_t {
    extern { fn setitimer(which: int_t, new: *const itimerval,
                          old: *mut itimerval) -> int_t; }
    let old_ptr = old.map(|v| v as *mut _).unwrap_or(0 as *mut _);
    unsafe { setitimer(which, new as *const _, old_ptr) }
}

pub fn utimes<T: NTStr>(file: &T, tvp: &[timeval]) -> int_t {
    extern { fn utimes(file: *const char_t, tvp: *const timeval) -> int_t; }
    if tvp.len() < 2 {
        return -1;
    }
    unsafe { utimes(file.as_ptr(), tvp.as_ptr()) }
}
