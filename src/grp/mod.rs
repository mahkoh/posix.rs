pub use self::os::{group};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn getgrgid_r<'a>(gid: ::sys::types::gid_t, dst: &'a mut group, buf: &'a mut [u8],
                      res: &mut uint) -> ::int_t {
    extern {
        fn getgrgid_r(gid: ::sys::types::gid_t, resultbuf: *mut group,
                      buffer: *mut ::char_t, buflen: ::size_t,
                      result: *mut *mut group) -> ::int_t;
    }
    unsafe { getgrgid_r(gid, dst as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as ::size_t, res as *mut _ as *mut _) }
}

pub fn getgrnam_r<'a, T: ::NTStr>(name: &T, dst: &'a mut group, buf: &'a mut [u8],
                                  res: &mut uint) -> ::int_t {
    extern {
        fn getgrnam_r(name: *const ::char_t, resultbuf: *mut group,
                      buffer: *mut ::char_t, buflen: ::size_t,
                      result: *mut *mut group) -> ::int_t;
    }
    unsafe { getgrnam_r(name.as_ptr(), dst as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as ::size_t, res as *mut _ as *mut _) }
}

extern "C" {
    pub fn getgrent() -> *mut group;
    pub fn getgrgid(gid: ::sys::types::gid_t) -> *mut group;
    pub fn getgrnam(name: *const ::char_t) -> *mut group;
    pub fn setgrent();
}
