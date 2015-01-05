#[repr(C)]
#[derive(Copy)]
pub struct timeval {
    pub tv_sec: ::sys::types::time_t,
    pub tv_usec: ::sys::types::suseconds_t,
}
new!(timeval);
#[repr(C)]
#[derive(Copy)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}
new!(itimerval);
pub const ITIMER_REAL: ::int_t = 0;
pub const ITIMER_VIRTUAL: ::int_t = 1;
pub const ITIMER_PROF: ::int_t = 2;
