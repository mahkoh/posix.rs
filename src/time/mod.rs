pub use self::os::{timespec};
pub use self::os::{tm};
pub use self::os::{itimerspec};
pub use self::os::{CLOCKS_PER_SEC};
pub use self::os::{CLOCK_MONOTONIC};
pub use self::os::{CLOCK_PROCESS_CPUTIME_ID};
pub use self::os::{CLOCK_REALTIME};
pub use self::os::{CLOCK_THREAD_CPUTIME_ID};
pub use self::os::{TIMER_ABSTIME};
pub use self::os::{getdate_err};

use {int_t, char_t, long_t, NTStr, size_t, double_t};
use sys::types::{pid_t, clock_t, clockid_t, time_t, timer_t};
use locale::{locale_t};
use signal::{sigevent};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn clock() -> clock_t {
    extern { fn clock() -> clock_t; }
    unsafe { clock() }
}

pub fn clock_getcpuclockid(pid: pid_t,
                           clock_id: &mut clockid_t) -> int_t {
    extern {
        fn clock_getcpuclockid(pid: pid_t,
                               clock_id: *mut clockid_t) -> int_t;
    }
    unsafe { clock_getcpuclockid(pid, clock_id as *mut _) }
}

pub fn clock_getres(id: clockid_t, res: &mut timespec) -> int_t {
    extern {
        fn clock_getres(id: clockid_t, res: *mut timespec) -> int_t;
    }
    unsafe { clock_getres(id, res as *mut _) }
}

pub fn clock_gettime(id: clockid_t, res: &mut timespec) -> int_t {
    extern {
        fn clock_gettime(id: clockid_t, res: *mut timespec) -> int_t;
    }
    unsafe { clock_gettime(id, res as *mut _) }
}

pub fn clock_nanosleep(id: clockid_t, flags: int_t, req: &timespec,
                       remain: Option<&mut timespec>) -> int_t {
    extern {
        fn clock_nanosleep(clock_id: clockid_t, flags: int_t,
                           req: *const timespec, rem: *mut timespec) -> int_t;
    }
    match remain {
        Some(p) => unsafe { clock_nanosleep(id, flags, req as *const _, p as *mut _) },
        _ => unsafe { clock_nanosleep(id, flags, req as *const _, 0 as *mut _) },
    }
}

pub fn clock_settime(id: clockid_t, res: &timespec) -> int_t {
    extern {
        fn clock_settime(id: clockid_t, res: *const timespec) -> int_t;
    }
    unsafe { clock_settime(id, res as *const _) }
}

pub fn difftime(time1: time_t, time0: time_t) -> f64 {
    extern {
        fn difftime(time1: time_t,
                    time0: time_t) -> double_t;
    }
    unsafe { difftime(time1, time0) }
}

pub fn gmtime_r(timep: &time_t, res: &mut tm) {
    extern {
        fn gmtime_r(timer: *const time_t, tp: *mut tm) -> *mut tm;
    }
    unsafe { gmtime_r(timep as *const _, res as *mut _); }
}

pub fn localtime_r(timep: &time_t, res: &mut tm) {
    extern {
        fn localtime_r(timer: *const time_t, tp: *mut tm) -> *mut tm;
    }
    unsafe { localtime_r(timep as *const _, res as *mut _); }
}

pub fn mktime(tm: &mut tm) -> time_t {
    extern {
        fn mktime(tp: *mut tm) -> time_t;
    }
    unsafe { mktime(tm as *mut _) }
}

pub fn nanosleep(req: &timespec, remain: Option<&mut timespec>) -> int_t {
    extern {
        fn nanosleep(req: *const timespec, rem: *mut timespec) -> int_t;
    }
    match remain {
        Some(p) => unsafe { nanosleep(req as *const _, p as *mut _) },
        _ => unsafe { nanosleep(req as *const _, 0 as *mut _) },
    }
}

pub fn strftime<T: NTStr>(dst: &mut [u8], fmt: &T, tm: &tm) -> size_t {
    extern {
        fn strftime(s: *mut char_t, max: size_t, format: *const char_t,
                    tp: *const tm) -> size_t;
    }
    unsafe { strftime(dst.as_mut_ptr() as *mut _, dst.len() as size_t, fmt.as_ptr(),
                      tm as *const _) }
}

pub fn strftime_l<T: NTStr>(dst: &mut [u8], fmt: &T, tm: &tm,
                            locale: locale_t) -> size_t {
    extern {
        fn strftime_l(s: *mut char_t, max: size_t, format: *const char_t,
                    tp: *const tm, locale: locale_t) -> size_t;
    }
    unsafe { strftime_l(dst.as_mut_ptr() as *mut _, dst.len() as size_t, fmt.as_ptr(),
                      tm as *const _, locale) }
}

pub fn strptime<T: NTStr, U: NTStr>(src: &T, fmt: &T, dst: &mut tm) -> usize {
    extern {
        fn strptime(src: *const char_t, fmt: *const char_t,
                    dst: *mut tm) -> *mut char_t;
    }
    let r = unsafe { strptime(src.as_ptr(), fmt.as_ptr(), dst as *mut _) };
    r as usize - src.as_ptr() as usize
}

pub fn time() -> time_t {
    extern { fn time(timer: *mut time_t) -> time_t; }
    unsafe { time(0 as *mut _) }
}

pub fn timer_create(id: clockid_t, evp: &mut sigevent,
                    timerid: &mut timer_t) -> int_t {
    extern { 
        fn timer_create(clock_id: clockid_t, evp: *mut sigevent,
                        timerid: *mut timer_t) -> int_t;
    }
    unsafe { timer_create(id, evp as *mut _, timerid as *mut _) }
}

pub fn timer_delete(timerid: timer_t) -> int_t {
    extern { fn timer_delete(timerid: timer_t) -> int_t; }
    unsafe { timer_delete(timerid) }
}

pub fn timer_getoverrun(timerid: timer_t) -> int_t {
    extern { fn timer_getoverrun(timerid: timer_t) -> int_t; }
    unsafe { timer_getoverrun(timerid) }
}

pub fn timer_gettime(timerid: timer_t, tm: &mut itimerspec) -> int_t {
    extern {
        fn timer_gettime(timerid: timer_t,
                         value: *mut itimerspec) -> int_t;
    }
    unsafe { timer_gettime(timerid, tm as *mut _) }
}

pub fn timre_settime(id: timer_t, flags: int_t, v: &itimerspec,
                     o: Option<&mut itimerspec>) -> int_t {
    extern {
        fn timer_settime(timerid: timer_t, flags: int_t,
                         value: *const itimerspec, ovalue: *mut itimerspec) -> int_t;
    }
    match o {
        Some(p) => unsafe { timer_settime(id, flags, v as *const _, p as *mut _) },
        _ => unsafe { timer_settime(id, flags, v as *const _, 0 as *mut _) },
    }
}

pub fn tzset() {
    extern { fn tzset(); }
    unsafe{ tzset() }
}

extern "C" {
    pub static mut tzname: [*mut char_t; 2usize];
    pub static mut daylight: int_t;
    pub static mut timezone: long_t;

    pub fn asctime(tp: *const tm) -> *mut char_t;
    pub fn asctime_r(tp: *const tm, buf: *mut char_t) -> *mut char_t;
    pub fn ctime(timer: *const time_t) -> *mut char_t;
    pub fn ctime_r(timer: *const time_t, buf: *mut char_t) -> *mut char_t;
    pub fn getdate(string: *const char_t) -> *mut tm;
    pub fn gmtime(timer: *const time_t) -> *mut tm;
    pub fn localtime(timer: *const time_t) -> *mut tm;
}
