pub use self::os::{nl_catd};
pub use self::os::{nl_item};
pub use self::os::{NL_SETD};
pub use self::os::{NL_CAT_LOCALE};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn catopen<T: ::NTStr>(name: &T, flag: ::int_t) -> nl_catd {
    extern { fn catopen(name: *const ::char_t, flag: ::int_t) -> nl_catd; }
    unsafe { catopen(name.as_ptr(), flag) }
}

pub fn catclose(catalog: nl_catd) -> ::int_t {
    extern { fn catclose(catalog: nl_catd) -> ::int_t; }
    unsafe { catclose(catalog) }
}

extern "C" {
    pub fn catgets(catalog: nl_catd, set: ::int_t, number: ::int_t,
                   string: *const ::char_t) -> *mut ::char_t;
}
