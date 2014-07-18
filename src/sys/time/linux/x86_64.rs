#[repr(C)]
pub struct timeval {
    pub tv_sec: ::sys::types::time_t,
    pub tv_usec: ::sys::types::suseconds_t,
}

#[repr(C)]
pub struct itimerval {
    pub it_interval: timeval,
    pub it_value: timeval,
}

pub static ITIMER_REAL:    ::int_t = 0;
pub static ITIMER_VIRTUAL: ::int_t = 1;
pub static ITIMER_PROF:    ::int_t = 2;
