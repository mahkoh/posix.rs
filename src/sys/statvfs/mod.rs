pub use self::os::{statvfs};
pub use self::os::{ST_RDONLY};
pub use self::os::{ST_NOSUID};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn statvfs<T: ::NTStr>(file: &T, buf: &mut statvfs) -> ::int_t {
    extern { fn statvfs(file: *const ::char_t, buf: *mut statvfs) -> ::int_t; }
    unsafe { statvfs(file.as_ptr(), buf as *mut _) }
}

pub fn fstatvfs(fildes: ::int_t, buf: &mut statvfs) -> ::int_t {
    extern { fn fstatvfs(fildes: ::int_t, buf: *mut statvfs) -> ::int_t; }
    unsafe { fstatvfs(fildes, buf as *mut _) }
}
