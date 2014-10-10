#[repr(C)]
pub struct lconv {
    pub decimal_point: *mut ::schar_t,
    pub thousands_sep: *mut ::schar_t,
    pub grouping: *mut ::schar_t,
    pub int_curr_symbol: *mut ::schar_t,
    pub currency_symbol: *mut ::schar_t,
    pub mon_decimal_point: *mut ::schar_t,
    pub mon_thousands_sep: *mut ::schar_t,
    pub mon_grouping: *mut ::schar_t,
    pub positive_sign: *mut ::schar_t,
    pub negative_sign: *mut ::schar_t,
    pub int_frac_digits: ::schar_t,
    pub frac_digits: ::schar_t,
    pub p_cs_precedes: ::schar_t,
    pub p_sep_by_space: ::schar_t,
    pub n_cs_precedes: ::schar_t,
    pub n_sep_by_space: ::schar_t,
    pub p_sign_posn: ::schar_t,
    pub n_sign_posn: ::schar_t,
    pub int_p_cs_precedes: ::schar_t,
    pub int_p_sep_by_space: ::schar_t,
    pub int_n_cs_precedes: ::schar_t,
    pub int_n_sep_by_space: ::schar_t,
    pub int_p_sign_posn: ::schar_t,
    pub int_n_sign_posn: ::schar_t,
}
new!(lconv)
pub type locale_t =  *mut ::void_t;
pub const LC_ALL: ::int_t = 6;
pub const LC_COLLATE: ::int_t = 3;
pub const LC_CTYPE: ::int_t = 0;
pub const LC_MESSAGES: ::int_t = 5;
pub const LC_MONETARY: ::int_t = 4;
pub const LC_NUMERIC: ::int_t = 1;
pub const LC_TIME: ::int_t = 2;
pub const LC_COLLATE_MASK: ::int_t = (1 << 3);
pub const LC_CTYPE_MASK: ::int_t = (1 << 0);
pub const LC_MESSAGES_MASK: ::int_t = (1 << 5);
pub const LC_MONETARY_MASK: ::int_t = (1 << 4);
pub const LC_NUMERIC_MASK: ::int_t = (1 << 1);
pub const LC_TIME_MASK: ::int_t = (1 << 2);
pub const LC_ALL_MASK: ::int_t = ((1 << 0) | (1 << 1) | (1 << 2) | (1 << 3) | (1 << 4) | (1 << 5) | (1 << 7) | (1 << 8) | (1 << 9) | (1 << 10) | (1 << 11) | (1 << 12) );
pub const LC_GLOBAL_LOCALE: locale_t = -1 as locale_t;
