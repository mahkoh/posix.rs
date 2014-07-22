use {size_t, char_t, ssize_t};
use locale::{locale_t};

extern {
    pub fn strfmon(s: *mut char_t, max: size_t, format: *const char_t, ...) -> ssize_t;
    pub fn strfmon_l(s: *mut char_t, max: size_t, loc: locale_t, format: *const char_t,
                     ...) -> ssize_t;
}
