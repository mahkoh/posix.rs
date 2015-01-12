pub use self::os::{group};

use {int_t, char_t, NTStr, size_t};
use sys::types::{gid_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn getgrgid_r<'a>(gid: gid_t, dst: &'a mut group, buf: &'a mut [u8],
                      res: &mut usize) -> int_t {
    extern {
        fn getgrgid_r(gid: gid_t, resultbuf: *mut group, buffer: *mut char_t,
                      buflen: size_t, result: *mut *mut group) -> int_t;
    }
    unsafe { getgrgid_r(gid, dst as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as size_t, res as *mut _ as *mut _) }
}

pub fn getgrnam_r<'a, T: NTStr>(name: &T, dst: &'a mut group, buf: &'a mut [u8],
                                  res: &mut usize) -> int_t {
    extern {
        fn getgrnam_r(name: *const char_t, resultbuf: *mut group, buffer: *mut char_t,
                      buflen: size_t, result: *mut *mut group) -> int_t;
    }
    unsafe { getgrnam_r(name.as_ptr(), dst as *mut _, buf.as_mut_ptr() as *mut _,
                        buf.len() as size_t, res as *mut _ as *mut _) }
}

extern {
    pub fn getgrent() -> *mut group;
    pub fn getgrgid(gid: gid_t) -> *mut group;
    pub fn getgrnam(name: *const char_t) -> *mut group;
    pub fn setgrent();
}

#[cfg(test)]
mod tests {
    use std::c_str::{CString};
    use {ToNTStr};

    #[test]
    fn test_grgid() {
        // Maybe this test assumes too much.
        let mut buf = [0; 1024];
        let mut res = 0;
        let mut dst = super::group::new();
        assert_eq!(super::getgrgid_r(0, &mut dst, &mut buf, &mut res), 0);
        assert!(res != 0);
        assert_eq!(dst.gr_gid, 0);
        unsafe {
            assert_eq!(CString::new(dst.gr_name as *const _, false).as_bytes(),
                       b"root\x00");
        }
    }

    #[test]
    fn test_grnam() {
        // Maybe this test assumes too much.
        let mut buf = [0; 1024];
        let mut res = 0;
        let mut dst = super::group::new();
        assert_eq!(super::getgrnam_r(&"root".to_nt_str(), &mut dst, &mut buf, &mut res), 0);
        assert!(res != 0);
        assert_eq!(dst.gr_gid, 0);
        unsafe {
            assert_eq!(CString::new(dst.gr_name as *const _, false).as_bytes(),
                       b"root\x00");
        }
    }
}
