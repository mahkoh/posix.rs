pub use self::os::{wordexp_t};
pub use self::os::{WRDE_APPEND};
pub use self::os::{WRDE_DOOFFS};
pub use self::os::{WRDE_NOCMD};
pub use self::os::{WRDE_REUSE};
pub use self::os::{WRDE_SHOWERR};
pub use self::os::{WRDE_UNDEF};
pub use self::os::{WRDE_BADCHAR};
pub use self::os::{WRDE_BADVAL};
pub use self::os::{WRDE_CMDSUB};
pub use self::os::{WRDE_NOSPACE};
pub use self::os::{WRDE_SYNTAX};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn wordexp<T: ::NTStr>(words: &T, pwordexp: &mut wordexp_t,
                           flags: ::int_t) -> ::int_t {
    extern { fn wordexp(words: *const ::char_t, pwordexp: *mut wordexp_t,
                        flags: ::int_t) -> ::int_t; }
    unsafe { wordexp(words.as_ptr(), pwordexp as *mut _, flags) }
}

pub fn wordfree(wordexp: &mut wordexp_t) {
    extern { fn wordfree(wordexp: *mut wordexp_t); }
    unsafe { wordfree(wordexp as *mut _); }
}
