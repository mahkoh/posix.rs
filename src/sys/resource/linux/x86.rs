pub type rlim_t = ::ulong_t;
#[repr(C)]
#[derive(Copy)]
pub struct rlimit {
    pub rlim_cur: rlim_t,
    pub rlim_max: rlim_t,
}
new!(rlimit);
#[repr(C)]
#[derive(Copy)]
pub struct rusage {
    pub ru_utime: ::sys::time::timeval,
    pub ru_stime: ::sys::time::timeval,
}
new!(rusage);
pub const PRIO_PROCESS: ::int_t = 0;
pub const PRIO_PGRP: ::int_t = 1;
pub const PRIO_USER: ::int_t = 2;
pub const RUSAGE_SELF: ::int_t = 0;
pub const RUSAGE_CHILDREN: ::int_t = -1;
pub const RLIMIT_CORE: ::int_t = 4;
pub const RLIMIT_CPU: ::int_t = 0;
pub const RLIMIT_DATA: ::int_t = 2;
pub const RLIMIT_FSIZE: ::int_t = 1;
pub const RLIMIT_NOFILE: ::int_t = 7;
pub const RLIMIT_STACK: ::int_t = 3;
pub const RLIMIT_AS: ::int_t = 9;
pub const RLIM_INFINITY:  rlim_t = -1;
pub const RLIM_SAVED_MAX: rlim_t = -1;
pub const RLIM_SAVED_CUR: rlim_t = -1;
