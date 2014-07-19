pub use self::os::{wctrans_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn iswalnum_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswalnum_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswalnum_l(wc, locale) }
}

pub fn iswalnum(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswalnum(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswalnum(wc) }
}

pub fn iswalpha_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswalpha_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswalpha_l(wc, locale) }
}

pub fn iswalpha(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswalpha(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswalpha(wc) }
}

pub fn iswblank_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswblank_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswblank_l(wc, locale) }
}

pub fn iswblank(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswblank(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswblank(wc) }
}

pub fn iswcntrl_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswcntrl_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswcntrl_l(wc, locale) }
}

pub fn iswcntrl(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswcntrl(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswcntrl(wc) }
}

pub fn iswctype_l(wc: ::wchar::wint_t, desc: ::wchar::wctype_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswctype_l(wc: ::wchar::wint_t, desc: ::wchar::wctype_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswctype_l(wc, desc, locale) }
}

pub fn iswctype(wc: ::wchar::wint_t, desc: ::wchar::wctype_t) -> ::int_t {
    extern { fn iswctype(wc: ::wchar::wint_t, desc: ::wchar::wctype_t) -> ::int_t; }
    unsafe { iswctype(wc, desc) }
}

pub fn iswdigit_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswdigit_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswdigit_l(wc, locale) }
}

pub fn iswdigit(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswdigit(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswdigit(wc) }
}

pub fn iswgraph_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswgraph_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswgraph_l(wc, locale) }
}

pub fn iswgraph(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswgraph(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswgraph(wc) }
}

pub fn iswlower_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswlower_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswlower_l(wc, locale) }
}

pub fn iswlower(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswlower(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswlower(wc) }
}

pub fn iswprint_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswprint_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswprint_l(wc, locale) }
}

pub fn iswprint(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswprint(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswprint(wc) }
}

pub fn iswpunct_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswpunct_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswpunct_l(wc, locale) }
}

pub fn iswpunct(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswpunct(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswpunct(wc) }
}

pub fn iswspace_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswspace_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswspace_l(wc, locale) }
}

pub fn iswspace(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswspace(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswspace(wc) }
}

pub fn iswupper_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswupper_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswupper_l(wc, locale) }
}

pub fn iswupper(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswupper(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswupper(wc) }
}

pub fn iswxdigit_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t {
    extern { fn iswxdigit_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::int_t; }
    unsafe { iswxdigit_l(wc, locale) }
}

pub fn iswxdigit(wc: ::wchar::wint_t) -> ::int_t {
    extern { fn iswxdigit(wc: ::wchar::wint_t) -> ::int_t; }
    unsafe { iswxdigit(wc) }
}

pub fn towctrans_l(wc: ::wchar::wint_t, desc: wctrans_t, locale: ::locale::locale_t) -> ::wchar::wint_t {
    extern { fn towctrans_l(wc: ::wchar::wint_t, desc: wctrans_t, locale: ::locale::locale_t) -> ::wchar::wint_t; }
    unsafe { towctrans_l(wc, desc, locale) }
}

pub fn towctrans(wc: ::wchar::wint_t, desc: wctrans_t) -> ::wchar::wint_t {
    extern { fn towctrans(wc: ::wchar::wint_t, desc: wctrans_t) -> ::wchar::wint_t; }
    unsafe { towctrans(wc, desc) }
}

pub fn towlower_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::wchar::wint_t {
    extern { fn towlower_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::wchar::wint_t; }
    unsafe { towlower_l(wc, locale) }
}

pub fn towlower(wc: ::wchar::wint_t) -> ::wchar::wint_t {
    extern { fn towlower(wc: ::wchar::wint_t) -> ::wchar::wint_t; }
    unsafe { towlower(wc) }
}

pub fn towupper_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::wchar::wint_t {
    extern { fn towupper_l(wc: ::wchar::wint_t, locale: ::locale::locale_t) -> ::wchar::wint_t; }
    unsafe { towupper_l(wc, locale) }
}

pub fn towupper(wc: ::wchar::wint_t) -> ::wchar::wint_t {
    extern { fn towupper(wc: ::wchar::wint_t) -> ::wchar::wint_t; }
    unsafe { towupper(wc) }
}

extern {
    pub fn wctrans_l(property: *const ::char_t, locale: ::locale::locale_t) -> wctrans_t;
    pub fn wctrans(property: *const ::char_t) -> wctrans_t;
    pub fn wctype_l(property: *const ::char_t, locale: ::locale::locale_t) -> ::wchar::wctype_t;
    pub fn wctype(property: *const ::char_t) -> ::wchar::wctype_t;
}
