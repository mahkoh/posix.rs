#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut ::char_t,
    pub thousands_sep: *mut ::char_t,
    pub grouping: *mut ::char_t,
    pub int_curr_symbol: *mut ::char_t,
    pub currency_symbol: *mut ::char_t,
    pub mon_decimal_point: *mut ::char_t,
    pub mon_thousands_sep: *mut ::char_t,
    pub mon_grouping: *mut ::char_t,
    pub positive_sign: *mut ::char_t,
    pub negative_sign: *mut ::char_t,
    pub int_frac_digits: ::char_t,
    pub frac_digits: ::char_t,
    pub p_cs_precedes: ::char_t,
    pub p_sep_by_space: ::char_t,
    pub n_cs_precedes: ::char_t,
    pub n_sep_by_space: ::char_t,
    pub p_sign_posn: ::char_t,
    pub n_sign_posn: ::char_t,
    pub int_p_cs_precedes: ::char_t,
    pub int_p_sep_by_space: ::char_t,
    pub int_n_cs_precedes: ::char_t,
    pub int_n_sep_by_space: ::char_t,
    pub int_p_sign_posn: ::char_t,
    pub int_n_sign_posn: ::char_t,
}

new!(lconv)

pub type locale_t = *mut ::void_t;

pub static LC_ALL:           ::int_t = 6;
pub static LC_COLLATE:       ::int_t = 3;
pub static LC_CTYPE:         ::int_t = 0;
pub static LC_MESSAGES:      ::int_t = 5;
pub static LC_MONETARY:      ::int_t = 4;
pub static LC_NUMERIC:       ::int_t = 1;
pub static LC_TIME:          ::int_t = 2;
pub static LC_COLLATE_MASK:  ::int_t = 8;
pub static LC_CTYPE_MASK:    ::int_t = 1;
pub static LC_MESSAGES_MASK: ::int_t = 32;
pub static LC_MONETARY_MASK: ::int_t = 16;
pub static LC_NUMERIC_MASK:  ::int_t = 2;
pub static LC_TIME_MASK:     ::int_t = 4;
pub static LC_ALL_MASK:      ::int_t = 8127;

pub static LC_GLOBAL_LOCALE: locale_t = -1i64 as locale_t;
