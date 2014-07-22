pub use self::os::{iconv_t};

use {NTStr, char_t, int_t, size_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn iconv_open<T: NTStr, U: NTStr>(to: &T, from: &U) -> iconv_t {
    extern { fn iconv_open(to: *const char_t, from: *const char_t) -> iconv_t; }
    unsafe { iconv_open(to.as_ptr(), from.as_ptr()) }
}

pub fn iconv_close(cd: iconv_t) -> int_t {
    extern { fn iconv_close(cd: iconv_t) -> int_t; }
    unsafe { iconv_close(cd) }
}

extern "C" {
    pub fn iconv(cd: iconv_t, inbuf: *mut *mut char_t,
                 inbytesleft: *mut size_t, outbuf: *mut *mut char_t,
                 outbytesleft: *mut size_t) -> size_t;
}
