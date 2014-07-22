pub use self::os::{lconv};
pub use self::os::{locale_t};
pub use self::os::{LC_ALL};
pub use self::os::{LC_COLLATE};
pub use self::os::{LC_CTYPE};
pub use self::os::{LC_MESSAGES};
pub use self::os::{LC_MONETARY};
pub use self::os::{LC_NUMERIC};
pub use self::os::{LC_TIME};
pub use self::os::{LC_COLLATE_MASK};
pub use self::os::{LC_CTYPE_MASK};
pub use self::os::{LC_MESSAGES_MASK};
pub use self::os::{LC_MONETARY_MASK};
pub use self::os::{LC_NUMERIC_MASK};
pub use self::os::{LC_TIME_MASK};
pub use self::os::{LC_ALL_MASK};
pub use self::os::{LC_GLOBAL_LOCALE};

use {NTStr, int_t, char_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn duplocale(l: locale_t) -> locale_t {
    extern { fn duplocale(dataset: locale_t) -> locale_t; }
    unsafe { duplocale(l) }
}

pub fn freelocale(l: locale_t) -> locale_t {
    extern { fn freelocale(dataset: locale_t) -> locale_t; }
    unsafe { freelocale(l) }
}

pub fn newlocale<T: NTStr>(mask: int_t, l: &T, base: Option<locale_t>) -> locale_t {
    extern { fn newlocale(mask: int_t, locale: *const char_t,
                          base: locale_t) -> locale_t; }
    match base {
        Some(p) => unsafe { newlocale(mask, l.as_ptr(), p) },
        None => unsafe { newlocale(mask, l.as_ptr(), 0 as locale_t) },
    }
}

pub fn uselocale(l: locale_t) -> locale_t {
    extern { fn uselocale(dataset: locale_t) -> locale_t; }
    unsafe { uselocale(l) }
}

extern "C" {
    pub fn localeconv() -> *mut lconv;
    pub fn setlocale(category: int_t, locale: *const char_t) -> *mut char_t;
}
