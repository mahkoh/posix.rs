#[repr(C)]
pub struct regex_t {
    __buffer: *mut ::uchar_t,
    __allocated: ::ulong_t,
    __used: ::ulong_t,
    __syntax: ::ulong_t,
    __fastmap: *mut ::char_t,
    __translate: *mut ::uchar_t,
    pub re_nsub: ::size_t,
    __can_be_null: ::uint_t,
    __regs_allocated: ::uint_t,
    __fastmap_accurate: ::uint_t,
    __no_sub: ::uint_t,
    __not_bol: ::uint_t,
    __not_eol: ::uint_t,
    __newline_anchor: ::uint_t,
}

new!(regex_t)

pub type regoff_t = ::int_t;

#[repr(C)]
pub struct regmatch_t {
    pub rm_so: regoff_t,
    pub rm_eo: regoff_t,
}

new!(regmatch_t)

pub static REG_EXTENDED: ::int_t = 1;
pub static REG_ICASE:    ::int_t = 2;
pub static REG_NOSUB:    ::int_t = 8;
pub static REG_NEWLINE:  ::int_t = 4;
pub static REG_NOTBOL:   ::int_t = 1;
pub static REG_NOTEOL:   ::int_t = 2;
pub static REG_NOMATCH:  ::int_t = 1;
pub static REG_BADPAT:   ::int_t = 2;
pub static REG_ECOLLATE: ::int_t = 3;
pub static REG_ECTYPE:   ::int_t = 4;
pub static REG_EESCAPE:  ::int_t = 5;
pub static REG_ESUBREG:  ::int_t = 6;
pub static REG_EBRACK:   ::int_t = 7;
pub static REG_EPAREN:   ::int_t = 8;
pub static REG_EBRACE:   ::int_t = 9;
pub static REG_BADBR:    ::int_t = 10;
pub static REG_ERANGE:   ::int_t = 11;
pub static REG_ESPACE:   ::int_t = 12;
pub static REG_BADRPT:   ::int_t = 13;
