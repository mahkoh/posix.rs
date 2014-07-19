pub use self::os::{imaxdiv_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn imaxabs(n: ::stdint::intmax_t) -> ::stdint::intmax_t {
    extern { fn imaxabs(n: ::stdint::intmax_t) -> ::stdint::intmax_t; }
    unsafe { imaxabs(n) }
}

pub fn imaxdiv(numer: ::stdint::intmax_t, denom: ::stdint::intmax_t) -> imaxdiv_t {
    extern { fn imaxdiv(numer: ::stdint::intmax_t,
                        denom: ::stdint::intmax_t) -> imaxdiv_t; }
    unsafe { imaxdiv(numer, denom) }
}

extern "C" {
    pub fn strtoimax(nptr: *const ::char_t, endptr: *mut *mut ::char_t,
                     base: ::int_t) -> ::stdint::intmax_t;
    pub fn strtoumax(nptr: *const ::char_t, endptr: *mut *mut ::char_t,
                     base: ::int_t) -> ::stdint::uintmax_t;
    pub fn wcstoimax(nptr: *const ::stddef::wchar_t, endptr: *mut *mut ::stddef::wchar_t,
                     base: ::int_t) -> ::stdint::intmax_t;
    pub fn wcstoumax(nptr: *const ::stddef::wchar_t, endptr: *mut *mut ::stddef::wchar_t,
                     base: ::int_t) -> ::stdint::uintmax_t;
}
