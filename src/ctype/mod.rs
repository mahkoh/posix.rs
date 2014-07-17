pub fn isalnum(arg1: ::int_t) -> ::int_t {
    extern { fn isalnum(arg1: ::int_t) -> ::int_t; }
    unsafe { isalnum(arg1) }
}

pub fn isalnum_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isalnum_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isalnum_l(arg1, arg2) }
}

pub fn isalpha(arg1: ::int_t) -> ::int_t {
    extern { fn isalpha(arg1: ::int_t) -> ::int_t; }
    unsafe { isalpha(arg1) }
}

pub fn isalpha_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isalpha_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isalpha_l(arg1, arg2) }
}

pub fn isascii(c: ::int_t) -> ::int_t {
    extern { fn isascii(c: ::int_t) -> ::int_t; }
    unsafe { isascii(c) }
}

pub fn isblank(c: ::int_t) -> ::int_t {
    extern { fn isblank(c: ::int_t) -> ::int_t; }
    unsafe { isblank(c) }
}

pub fn isblank_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isblank_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isblank_l(arg1, arg2) }
}

pub fn iscntrl(c: ::int_t) -> ::int_t {
    extern { fn iscntrl(c: ::int_t) -> ::int_t; }
    unsafe { iscntrl(c) }
}

pub fn iscntrl_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn iscntrl_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { iscntrl_l(arg1, arg2) }
}

pub fn isdigit(c: ::int_t) -> ::int_t {
    extern { fn isdigit(c: ::int_t) -> ::int_t; }
    unsafe { isdigit(c) }
}

pub fn isdigit_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isdigit_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isdigit_l(arg1, arg2) }
}

pub fn isgraph(c: ::int_t) -> ::int_t {
    extern { fn isgraph(c: ::int_t) -> ::int_t; }
    unsafe { isgraph(c) }
}

pub fn isgraph_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isgraph_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isgraph_l(arg1, arg2) }
}

pub fn islower(c: ::int_t) -> ::int_t {
    extern { fn islower(c: ::int_t) -> ::int_t; }
    unsafe { islower(c) }
}

pub fn islower_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn islower_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { islower_l(arg1, arg2) }
}

pub fn isprint(c: ::int_t) -> ::int_t {
    extern { fn isprint(c: ::int_t) -> ::int_t; }
    unsafe { isprint(c) }
}

pub fn isprint_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isprint_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isprint_l(arg1, arg2) }
}

pub fn ispunct(c: ::int_t) -> ::int_t {
    extern { fn ispunct(c: ::int_t) -> ::int_t; }
    unsafe { ispunct(c) }
}

pub fn ispunct_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn ispunct_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { ispunct_l(arg1, arg2) }
}

pub fn isspace(c: ::int_t) -> ::int_t {
    extern { fn isspace(c: ::int_t) -> ::int_t; }
    unsafe { isspace(c) }
}

pub fn isspace_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isspace_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isspace_l(arg1, arg2) }
}

pub fn isupper(c: ::int_t) -> ::int_t {
    extern { fn isupper(c: ::int_t) -> ::int_t; }
    unsafe { isupper(c) }
}

pub fn isupper_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isupper_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isupper_l(arg1, arg2) }
}

pub fn isxdigit(c: ::int_t) -> ::int_t {
    extern { fn isxdigit(c: ::int_t) -> ::int_t; }
    unsafe { isxdigit(c) }
}

pub fn isxdigit_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn isxdigit_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { isxdigit_l(arg1, arg2) }
}

pub fn toascii(c: ::int_t) -> ::int_t {
    extern { fn toascii(c: ::int_t) -> ::int_t; }
    unsafe { toascii(c) }
}

pub fn tolower(c: ::int_t) -> ::int_t {
    extern { fn tolower(c: ::int_t) -> ::int_t; }
    unsafe { tolower(c) }
}

pub fn tolower_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn tolower_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { tolower_l(arg1, arg2) }
}

pub fn toupper(c: ::int_t) -> ::int_t {
    extern { fn toupper(c: ::int_t) -> ::int_t; }
    unsafe { toupper(c) }
}

pub fn toupper_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t {
    extern { fn toupper_l(arg1: ::int_t, arg2: ::locale::locale_t) -> ::int_t; }
    unsafe { toupper_l(arg1, arg2) }
}
