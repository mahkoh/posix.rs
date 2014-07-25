pub static FLT_RADIX:       ::int_t = 2;
pub static FLT_MANT_DIG:    ::int_t = 24;
pub static DBL_MANT_DIG:    ::int_t = 53;
pub static LDBL_MANT_DIG:   ::int_t = 64;
pub static FLT_DIG:         ::int_t = 6;
pub static DBL_DIG:         ::int_t = 15;
pub static LDBL_DIG:        ::int_t = 18;
pub static FLT_MIN_EXP:     ::int_t = -125;
pub static DBL_MIN_EXP:     ::int_t = -1021;
pub static LDBL_MIN_EXP:    ::int_t = -16381;
pub static FLT_MIN_10_EXP:  ::int_t = -37;
pub static DBL_MIN_10_EXP:  ::int_t = -307;
pub static LDBL_MIN_10_EXP: ::int_t = -4931;
pub static FLT_MAX_EXP:     ::int_t = 128;
pub static DBL_MAX_EXP:     ::int_t = 1024;
pub static LDBL_MAX_EXP:    ::int_t = 16384;
pub static FLT_MAX_10_EXP:  ::int_t = 38;
pub static DBL_MAX_10_EXP:  ::int_t = 308;
pub static LDBL_MAX_10_EXP: ::int_t = 4932;

pub fn FLT_MAX() -> &'static mut ::float_t {
    extern { static mut _FLT_MAX: ::float_t; }
    unsafe { &mut _FLT_MAX }
}

pub fn DBL_MAX() -> &'static mut ::double_t {
    extern { static mut _DBL_MAX: ::double_t; }
    unsafe { &mut _DBL_MAX }
}

pub fn LDBL_MAX() -> &'static mut ::double_t {
    extern { static mut _LDBL_MAX: ::double_t; }
    unsafe { &mut _LDBL_MAX }
}

pub fn FLT_EPSILON() -> &'static mut ::float_t {
    extern { static mut _FLT_EPSILON: ::float_t; }
    unsafe { &mut _FLT_EPSILON }
}

pub fn DBL_EPSILON() -> &'static mut ::double_t {
    extern { static mut _DBL_EPSILON: ::double_t; }
    unsafe { &mut _DBL_EPSILON }
}

pub fn LDBL_EPSILON() -> &'static mut ::double_t {
    extern { static mut _LDBL_EPSILON: ::double_t; }
    unsafe { &mut _LDBL_EPSILON }
}

pub fn FLT_MIN() -> &'static mut ::float_t {
    extern { static mut _FLT_MIN: ::float_t; }
    unsafe { &mut _FLT_MIN }
}

pub fn DBL_MIN() -> &'static mut ::double_t {
    extern { static mut _DBL_MIN: ::double_t; }
    unsafe { &mut _DBL_MIN }
}

pub fn LDBL_MIN() -> &'static mut ::double_t {
    extern { static mut _LDBL_MIN: ::double_t; }
    unsafe { &mut _LDBL_MIN }
}
