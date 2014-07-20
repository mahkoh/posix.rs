#![crate_name = "posix"]
#![crate_type = "lib"]
#![allow(non_camel_case_types)]
#![allow(non_snake_case_functions)]

pub use os::arch::{char_t, schar_t, uchar_t, short_t, ushort_t, int_t, uint_t, long_t};
pub use os::arch::{ulong_t, longlong_t, ulonglong_t, float_t, double_t, size_t, ssize_t}; 

pub mod aio;
pub mod arpa;
pub mod cpio;
pub mod ctype;
pub mod dirent;
pub mod dlfcn;
pub mod errno;
pub mod fcntl;
pub mod fenv;
pub mod float;
pub mod fmtmsg;
pub mod fnmatch;
pub mod ftw;
pub mod glob;
pub mod grp;
pub mod iconv;
pub mod inttypes;
pub mod langinfo;
pub mod libgen;
pub mod limits;
pub mod locale;
pub mod monetary;
pub mod mqueue;
pub mod ndbm;
pub mod net;
pub mod netdb;
pub mod netinet;
pub mod nl_types;
pub mod poll;
pub mod pthread;
pub mod pwd;
pub mod regex;
pub mod sched;
pub mod search;
pub mod semaphore;
pub mod setjmp;
pub mod spawn;
pub mod stddef;
pub mod stdint;
pub mod stdio;
pub mod stdlib;
pub mod string;
pub mod strings;
pub mod stropts;
pub mod signal;
pub mod sys;
pub mod syslog;
pub mod tar;
pub mod termios;
pub mod time;
pub mod ulimit;
pub mod unistd;
pub mod utime;
pub mod utmpx;
pub mod wchar;
pub mod wctype;
pub mod wordexp;

#[repr(u8)]
pub enum void_t {
    __variant1,
    __variant2,
}

#[cfg(target_os = "linux")]
mod os {
    #[cfg(target_arch = "x86_64")]
    pub mod arch {
        pub type char_t       = i8;
        pub type schar_t      = i8;
        pub type uchar_t      = u8;
        pub type short_t      = i16;
        pub type ushort_t     = u16;
        pub type int_t        = i32;
        pub type uint_t       = u32;
        pub type long_t       = i64;
        pub type ulong_t      = u64;
        pub type longlong_t   = i64;
        pub type ulonglong_t  = u64;
        pub type float_t      = f32;
        pub type double_t     = f64;
        pub type longdouble_t = f64;
        pub type size_t       = u64;
        pub type ssize_t      = i64;
    }
}

pub trait NTStr {
    fn as_ptr(&self) -> *const char_t;
}

pub trait NTStrMut: NTStr {
    fn as_mut_ptr(&mut self) -> *mut char_t;
}

pub struct NTStrOwned {
    vec: Vec<u8>,
}

impl NTStr for NTStrOwned {
    fn as_ptr(&self) -> *const char_t {
        self.vec.as_ptr() as *const _
    }
}

impl NTStrMut for NTStrOwned {
    fn as_mut_ptr(&mut self) -> *mut char_t {
        self.vec.as_mut_ptr() as *mut _
    }
}
