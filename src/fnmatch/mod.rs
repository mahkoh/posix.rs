pub use self::os::{FNM_NOMATCH};
pub use self::os::{FNM_PATHNAME};
pub use self::os::{FNM_PERIOD};
pub use self::os::{FNM_NOESCAPE};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn fnmatch<T: ::NTStr, U: ::NTStr>(pattern: &T, string: &U,
                                       flags: ::int_t) -> ::int_t {
    extern {
        fn fnmatch(pattern: *const ::char_t, name: *const ::char_t,
                   flags: ::int_t) -> ::int_t;
    }
    unsafe { fnmatch(pattern.as_ptr(), string.as_ptr(), flags) }
}
