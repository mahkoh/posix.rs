pub use self::os::{passwd};

use {NTStr, int_t, char_t, size_t};
use sys::types::{uid_t};

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

pub fn getpwnam_r<T: NTStr>(name: &T, pwd: &mut passwd, buf: &mut [u8],
                            res: &mut uint) -> int_t {
    extern { fn getpwnam_r(name: *const char_t, resultbuf: *mut passwd,
                           buffer: *mut char_t, buflen: size_t,
                           result: *mut *mut passwd) -> int_t; }
    unsafe { getpwnam_r(name.as_ptr(), pwd as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as size_t, res as *mut _ as *mut _) }
}

pub fn getpwuid_r(uid: uid_t, pwd: &mut passwd, buf: &mut [u8], res: &mut uint) -> int_t {
    extern { fn getpwuid_r(uid: uid_t,
                           resultbuf: *mut passwd,
                           buffer: *mut char_t, buflen: size_t,
                           result: *mut *mut passwd) -> int_t; }
    unsafe { getpwuid_r(uid, pwd as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as size_t, res as *mut _ as *mut _) }
}

extern {
    pub fn getpwent() -> *mut passwd;
    pub fn getpwnam(name: *const char_t) -> *mut passwd;
    pub fn getpwuid(uid: uid_t) -> *mut passwd;
}

#[cfg(test)]
mod tests {
    use std::c_str::{CString};
    use {ToNTStr};

    #[test]
    fn test_pwuid() {
        // Maybe this test assumes too much.
        let mut buf = [0, ..1024];
        let mut res = 0;
        let mut dst = super::passwd::new();
        assert_eq!(super::getpwuid_r(0, &mut dst, buf, &mut res), 0);
        assert!(res != 0);
        assert_eq!(dst.pw_uid, 0);
        unsafe {
            assert_eq!(CString::new(dst.pw_name as *const _, false).as_bytes(),
                       b"root\x00");
        }
    }

    #[test]
    fn test_pwnam() {
        // Maybe this test assumes too much.
        let mut buf = [0, ..1024];
        let mut res = 0;
        let mut dst = super::passwd::new();
        assert_eq!(super::getpwnam_r(&"root".to_nt_str(), &mut dst, buf, &mut res), 0);
        assert!(res != 0);
        assert_eq!(dst.pw_uid, 0);
        unsafe {
            assert_eq!(CString::new(dst.pw_name as *const _, false).as_bytes(),
                       b"root\x00");
        }
    }
}
