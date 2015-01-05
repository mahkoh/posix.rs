#[repr(C)]
#[derive(Copy)]
pub struct timespec {
    pub tv_sec: ::sys::types::time_t,
    pub tv_nsec: ::long_t,
}
new!(timespec);
#[repr(C)]
#[derive(Copy)]
pub struct tm {
    pub tm_sec: ::int_t,
    pub tm_min: ::int_t,
    pub tm_hour: ::int_t,
    pub tm_mday: ::int_t,
    pub tm_mon: ::int_t,
    pub tm_year: ::int_t,
    pub tm_wday: ::int_t,
    pub tm_yday: ::int_t,
    pub tm_isdst: ::int_t,
    pub tm_gmtoff: ::long_t,
    pub tm_zone: *const ::schar_t,
}
new!(tm);
#[repr(C)]
#[derive(Copy)]
pub struct itimerspec {
    pub it_interval: timespec,
    pub it_value: timespec,
}
new!(itimerspec);
pub const TIMER_ABSTIME: ::uint_t = 1;
pub const CLOCKS_PER_SEC: ::sys::types::clock_t = 1000000;
pub const CLOCK_MONOTONIC: ::sys::types::clockid_t = 1;
pub const CLOCK_PROCESS_CPUTIME_ID: ::sys::types::clockid_t = 2;
pub const CLOCK_REALTIME: ::sys::types::clockid_t = 0;
pub const CLOCK_THREAD_CPUTIME_ID: ::sys::types::clockid_t = 3;

pub fn getdate_err() -> &'static mut ::int_t {
    extern { static mut getdate_err: ::int_t; }
    unsafe { &mut getdate_err }
}
