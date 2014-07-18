pub use self::os::{regex_t};
pub use self::os::{regoff_t};
pub use self::os::{regmatch_t};
pub use self::os::{REG_EXTENDED};
pub use self::os::{REG_ICASE};
pub use self::os::{REG_NOSUB};
pub use self::os::{REG_NEWLINE};
pub use self::os::{REG_NOTBOL};
pub use self::os::{REG_NOTEOL};
pub use self::os::{REG_NOMATCH};
pub use self::os::{REG_BADPAT};
pub use self::os::{REG_ECOLLATE};
pub use self::os::{REG_ECTYPE};
pub use self::os::{REG_EESCAPE};
pub use self::os::{REG_ESUBREG};
pub use self::os::{REG_EBRACK};
pub use self::os::{REG_EPAREN};
pub use self::os::{REG_EBRACE};
pub use self::os::{REG_BADBR};
pub use self::os::{REG_ERANGE};
pub use self::os::{REG_ESPACE};
pub use self::os::{REG_BADRPT};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn regcomp<T: ::NTStr>(preg: &mut regex_t, regex: &T, cflags: ::int_t) -> ::int_t {
    extern { fn regcomp(preg: *mut regex_t, pattern: *const ::char_t,
                        cflags: ::int_t) -> ::int_t; }
    unsafe { regcomp(preg as *mut _, regex.as_ptr(), cflags) }
}

pub fn regexec<T: ::NTStr>(preg: &regex_t, string: &T, pmatch: &mut [regmatch_t],
                           eflags: ::int_t) -> ::int_t {
    extern { fn regexec(preg: *const regex_t, string: *const ::char_t,
                        nmatch: ::size_t, pmatch: *mut regmatch_t,
                        eflags: ::int_t) -> ::int_t; }
    unsafe { regexec(preg as *const _, string.as_ptr(), pmatch.len() as ::size_t,
                     pmatch.as_mut_ptr(), eflags) }
}

pub fn regerror(errcode: ::int_t, preg: &regex_t, errbuf: &mut [u8]) -> ::size_t {
    extern { fn regerror(errcode: ::int_t, preg: *const regex_t,
                         errbuf: *mut ::char_t, errbuf_size: ::size_t) -> ::size_t; }
    unsafe { regerror(errcode, preg as *const _, errbuf.as_mut_ptr() as *mut _,
                      errbuf.len() as ::size_t) }
}

pub fn regfree(preg: &mut regex_t) {
    extern { fn regfree(preg: *mut regex_t); }
    unsafe { regfree(preg as *mut _) }
}
