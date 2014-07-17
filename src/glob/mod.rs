pub use self::os::{glob_t};
pub use self::os::{GLOB_APPEND};
pub use self::os::{GLOB_DOOFFS};
pub use self::os::{GLOB_ERR};
pub use self::os::{GLOB_MARK};
pub use self::os::{GLOB_NOCHECK};
pub use self::os::{GLOB_NOESCAPE};
pub use self::os::{GLOB_NOSORT};
pub use self::os::{GLOB_ABORTED};
pub use self::os::{GLOB_NOMATCH};
pub use self::os::{GLOB_NOSPACE};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn glob<T: ::NTStr>(pattern: &T, flags: ::int_t,
                        errfunc: Option<extern fn(arg1: *const ::char_t, arg2: ::int_t) -> ::int_t>,
                        pglob: &mut glob_t) -> ::int_t {
    extern {
        fn glob(pattern: *const ::char_t, flags: ::int_t,
                errfunc:
                    Option<extern fn(arg1: *const ::char_t, arg2: ::int_t) -> ::int_t>,
                pglob: *mut glob_t) -> ::int_t;
    }
    unsafe { glob(pattern.as_ptr(), flags, errfunc, pglob as *mut _) }
}

pub fn globfree(pglob: &mut glob_t) {
    extern { fn globfree(pglob: *mut glob_t); }
    unsafe { globfree(pglob as *mut _) }
}
