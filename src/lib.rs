#![crate_name = "posix"]
#![crate_type = "lib"]
#![allow(non_camel_case_types)]
#![allow(raw_pointer_derive)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

pub use os::arch::{char_t, schar_t, uchar_t, short_t, ushort_t, int_t, uint_t, long_t};
pub use os::arch::{ulong_t, longlong_t, ulonglong_t, float_t, double_t, size_t, ssize_t}; 

#[macro_use]
mod macros {
    macro_rules! new {
        ($name:ident) => {
            impl $name {
                pub fn new() -> $name {
                    unsafe { ::std::mem::zeroed() }
                }
            }
        }
    }
}

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
#[derive(Copy)]
pub enum void_t {
    __variant1,
    __variant2,
}

#[cfg(target_os = "linux")]
mod os {
    #[cfg(target_arch = "x86_64")]
    pub mod arch {
        pub type bool_t       = i8;
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
        pub type size_t       = u64;
        pub type ssize_t      = i64;
    }

    #[cfg(target_arch = "x86")]
    pub mod arch {
        pub type bool_t       = i8;
        pub type char_t       = i8;
        pub type schar_t      = i8;
        pub type uchar_t      = u8;
        pub type short_t      = i16;
        pub type ushort_t     = u16;
        pub type int_t        = i32;
        pub type uint_t       = u32;
        pub type long_t       = i32;
        pub type ulong_t      = u32;
        pub type longlong_t   = i64;
        pub type ulonglong_t  = u64;
        pub type float_t      = f32;
        pub type double_t     = f64;
        pub type size_t       = u32;
        pub type ssize_t      = i32;
    }
}

pub trait NTStr {
    fn as_ptr(&self) -> *const char_t;
}

pub struct NTStrBorrowed<'a> {
    data: &'a [u8],
}

pub struct NTStrOwned {
    vec: Vec<u8>,
}

impl<'a> NTStr for NTStrBorrowed<'a> {
    fn as_ptr(&self) -> *const char_t {
        self.data.as_ptr() as *const _
    }
}

impl NTStr for NTStrOwned {
    fn as_ptr(&self) -> *const char_t {
        self.vec.as_ptr() as *const _
    }
}

pub trait ToNTStr {
    fn to_nt_str(&self) -> NTStrOwned;
}

impl<'a> ToNTStr for &'a [u8] {
    fn to_nt_str(&self) -> NTStrOwned {
        let mut vec = self.to_vec();
        vec.push(0);
        NTStrOwned { vec: vec }
    }
}

impl ToNTStr for Path {
    fn to_nt_str(&self) -> NTStrOwned {
        use std::path::{BytesContainer};
        self.container_as_bytes().to_nt_str()
    }
}

impl<'a> ToNTStr for &'a str {
    fn to_nt_str(&self) -> NTStrOwned {
        self.as_bytes().to_nt_str()
    }
}

pub trait AsNTStr<'a> {
    fn as_nt_str(&'a self) -> NTStrBorrowed<'a>;
}

impl<'a> AsNTStr<'a> for &'a [u8] {
    fn as_nt_str(&'a self) -> NTStrBorrowed<'a> {
        NTStrBorrowed { data: *self }
    }
}

pub trait AsSlice: Sized {
    fn as_slice(&self) -> &[u8] {
        unsafe {
            ::std::mem::transmute(::std::raw::Slice {
                data: self as *const _ as *const u8,
                len: ::std::mem::size_of_val(self),
            })
        }
    }
}

pub trait AsMutSlice: Sized {
    fn as_mut_slice(&mut self) -> &mut [u8] {
        unsafe {
            ::std::mem::transmute(::std::raw::Slice {
                data: self as *mut _ as *const u8,
                len: ::std::mem::size_of_val(self),
            })
        }
    }
}
