extern "C" {
    pub fn strfmon(s: *mut ::char_t, max: ::size_t, format: *const ::char_t,
                   ...) -> ::ssize_t;
    pub fn strfmon_l(s: *mut ::char_t, max: ::size_t, loc: ::locale::locale_t,
                     format: *const ::char_t, ...) -> ::ssize_t;
}
