pub use self::os::{passwd};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn endpwent() {
    extern { fn endpwent(); }
    unsafe { endpwent() }
}

pub fn setpwent() {
    extern { fn setpwent(); }
    unsafe { setpwent() }
}

pub fn getpwnam_r<T: ::NTStr>(name: &T, pwd: &mut passwd, buf: &mut [u8],
                              res: &mut uint) -> ::int_t {
    extern { fn getpwnam_r(name: *const ::char_t,
                           resultbuf: *mut passwd,
                           buffer: *mut ::char_t, buflen: ::size_t,
                           result: *mut *mut passwd) -> ::int_t; }
    unsafe { getpwnam_r(name.as_ptr(), pwd as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as ::size_t, res as *mut _ as *mut _) }
}

pub fn getpwuid_r<T: ::NTStr>(uid: ::sys::types::uid_t, pwd: &mut passwd, buf: &mut [u8],
                              res: &mut uint) -> ::int_t {
    extern { fn getpwuid_r(uid: ::sys::types::uid_t,
                           resultbuf: *mut passwd,
                           buffer: *mut ::char_t, buflen: ::size_t,
                           result: *mut *mut passwd) -> ::int_t; }
    unsafe { getpwuid_r(uid, pwd as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as ::size_t, res as *mut _ as *mut _) }
}

extern "C" {
    pub fn getpwent() -> *mut passwd;
    pub fn getpwnam(name: *const ::char_t) -> *mut passwd;
    pub fn getpwuid(uid: ::sys::types::uid_t) -> *mut passwd;
}
