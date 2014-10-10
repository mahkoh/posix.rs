#[repr(C)]
pub struct div_t {
    pub quot: ::int_t,
    pub rem: ::int_t,
}

#[repr(C)]
pub struct ldiv_t {
    pub quot: ::long_t,
    pub rem: ::long_t,
}

#[repr(C)]
pub struct lldiv_t {
    pub quot: ::longlong_t,
    pub rem: ::longlong_t,
}

pub const EXIT_FAILURE: ::int_t = 1;
pub const EXIT_SUCCESS: ::int_t = 0;
pub const RAND_MAX:     ::int_t = 2147483647;

pub fn MB_CUR_MAX() -> ::size_t {
    extern { fn __ctype_get_mb_cur_max() -> ::size_t; }
    unsafe { __ctype_get_mb_cur_max() }
}
