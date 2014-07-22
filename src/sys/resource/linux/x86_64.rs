pub type rlim_t = ::ulong_t;

#[repr(C)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}

new!(rlimit)

#[repr(C)]
pub struct rusage {
    pub ru_utime: ::sys::time::timeval,
    pub ru_stime: ::sys::time::timeval,
}

new!(rusage)

pub static PRIO_PROCESS:    ::int_t = 0;
pub static PRIO_PGRP:       ::int_t = 1;
pub static PRIO_USER:       ::int_t = 2;
pub static RLIM_INFINITY:   rlim_t = -1;
pub static RLIM_SAVED_MAX:  rlim_t = -1;
pub static RLIM_SAVED_CUR:  rlim_t = -1;
pub static RUSAGE_SELF:     ::int_t = 0;
pub static RUSAGE_CHILDREN: ::int_t = -1;
pub static RLIMIT_CORE:     ::int_t = 4;
pub static RLIMIT_CPU:      ::int_t = 0;
pub static RLIMIT_DATA:     ::int_t = 2;
pub static RLIMIT_FSIZE:    ::int_t = 1;
pub static RLIMIT_NOFILE:   ::int_t = 7;
pub static RLIMIT_STACK:    ::int_t = 3;
pub static RLIMIT_AS:       ::int_t = 9;
