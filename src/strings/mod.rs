use {int_t, size_t, char_t};
use locale::{locale_t};

pub fn ffs(i: int_t) -> int_t {
    extern { fn ffs(i: int_t) -> int_t; }
    unsafe { ffs(i) }
}

extern {
    pub fn strcasecmp(s1: *const char_t, s2: *const char_t) -> int_t;
    pub fn strcasecmp_l(s1: *const char_t, s2: *const char_t,
                        loc: locale_t) -> int_t;
    pub fn strncasecmp(s1: *const char_t, s2: *const char_t, n: size_t) -> int_t;
    pub fn strncasecmp_l(s1: *const char_t, s2: *const char_t, n: size_t,
                         loc: locale_t) -> int_t;
}
