pub use self::os::{wint_t};
pub use self::os::{mbstate_t};
pub use self::os::{WEOF};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;
/*
extern "C" {
    pub fn wcscpy(dest: *mut wchar_t, src: *const wchar_t) -> *mut wchar_t;
    pub fn wcsncpy(dest: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wcscat(dest: *mut wchar_t, src: *const wchar_t) -> *mut wchar_t;
    pub fn wcsncat(dest: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wcscmp(s1: *const wchar_t, s2: *const wchar_t) -> ::int_t;
    pub fn wcsncmp(s1: *const wchar_t, s2: *const wchar_t, n: size_t) -> ::int_t;
    pub fn wcscasecmp(s1: *const wchar_t, s2: *const wchar_t) -> ::int_t;
    pub fn wcsncasecmp(s1: *const wchar_t, s2: *const wchar_t, n: size_t) -> ::int_t;
    pub fn wcscasecmp_l(s1: *const wchar_t, s2: *const wchar_t, loc: locale_t) -> ::int_t;
    pub fn wcsncasecmp_l(s1: *const wchar_t, s2: *const wchar_t, n: size_t, loc: locale_t) -> ::int_t;
    pub fn wcscoll(s1: *const wchar_t, s2: *const wchar_t) -> ::int_t;
    pub fn wcsxfrm(s1: *mut wchar_t, s2: *const wchar_t, n: size_t) -> size_t;
    pub fn wcscoll_l(s1: *const wchar_t, s2: *const wchar_t, loc: locale_t) -> ::int_t;
    pub fn wcsxfrm_l(s1: *mut wchar_t, s2: *const wchar_t, n: size_t, loc: locale_t) -> size_t;
    pub fn wcsdup(s: *const wchar_t) -> *mut wchar_t;
    pub fn wcschr(wcs: *const wchar_t, wc: wchar_t) -> *mut wchar_t;
    pub fn wcsrchr(wcs: *const wchar_t, wc: wchar_t) -> *mut wchar_t;
    pub fn wcscspn(wcs: *const wchar_t, reject: *const wchar_t) -> size_t;
    pub fn wcsspn(wcs: *const wchar_t, accept: *const wchar_t) -> size_t;
    pub fn wcspbrk(wcs: *const wchar_t, accept: *const wchar_t) -> *mut wchar_t;
    pub fn wcsstr(haystack: *const wchar_t, needle: *const wchar_t) -> *mut wchar_t;
    pub fn wcstok(s: *mut wchar_t, delim: *const wchar_t, ptr: *mut *mut wchar_t) -> *mut wchar_t;
    pub fn wcslen(s: *const wchar_t) -> size_t;
    pub fn wcsnlen(s: *const wchar_t, maxlen: size_t) -> size_t;
    pub fn wmemchr(s: *const wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wmemcmp(s1: *const wchar_t, s2: *const wchar_t, n: size_t) -> ::int_t;
    pub fn wmemcpy(s1: *mut wchar_t, s2: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wmemmove(s1: *mut wchar_t, s2: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn wmemset(s: *mut wchar_t, c: wchar_t, n: size_t) -> *mut wchar_t;
    pub fn btowc(c: ::int_t) -> wint_t;
    pub fn wctob(c: wint_t) -> ::int_t;
    pub fn mbsinit(ps: *const mbstate_t) -> ::int_t;
    pub fn mbrtowc(pwc: *mut wchar_t, s: *const ::char_t, n: size_t, p: *mut mbstate_t) -> size_t;
    pub fn wcrtomb(s: *mut ::char_t, wc: wchar_t, ps: *mut mbstate_t) -> size_t;
    pub fn mbrlen(s: *const ::char_t, n: size_t, ps: *mut mbstate_t) -> size_t;
    pub fn mbsrtowcs(dst: *mut wchar_t, src: *mut *const ::char_t, len: size_t, ps: *mut mbstate_t) -> size_t;
    pub fn wcsrtombs(dst: *mut ::char_t, src: *mut *const wchar_t, len: size_t, ps: *mut mbstate_t) -> size_t;
    pub fn mbsnrtowcs(dst: *mut wchar_t, src: *mut *const ::char_t, nmc: size_t, len: size_t, ps: *mut mbstate_t) -> size_t;
    pub fn wcsnrtombs(dst: *mut ::char_t, src: *mut *const wchar_t, nwc: size_t, len: size_t, ps: *mut mbstate_t) -> size_t;
    pub fn wcstod(nptr: *const wchar_t, endptr: *mut *mut wchar_t) -> ::libc::double_t;
    pub fn wcstof(nptr: *const wchar_t, endptr: *mut *mut wchar_t) -> ::libc::float_t;
    pub fn wcstold(nptr: *const wchar_t, endptr: *mut *mut wchar_t) -> ::libc::double_t;
    pub fn wcstol(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: ::int_t) -> ::long_t;
    pub fn wcstoul(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: ::int_t) -> ::ulong_t;
    pub fn wcstoll(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: ::int_t) -> ::longlong_t;
    pub fn wcstoull(nptr: *const wchar_t, endptr: *mut *mut wchar_t, base: ::int_t) -> ::ulonglong_t;
    pub fn wcpcpy(dest: *mut wchar_t, src: *const wchar_t) -> *mut wchar_t;
    pub fn wcpncpy(dest: *mut wchar_t, src: *const wchar_t, n: size_t) -> *mut wchar_t;
    pub fn open_wmemstream(bufloc: *mut *mut wchar_t, sizeloc: *mut size_t) -> *mut FILE;
    pub fn fwide(fp: *mut FILE, mode: ::int_t) -> ::int_t;
    pub fn fwprintf(stream: *mut FILE, format: *const wchar_t, ...) -> ::int_t;
    pub fn wprintf(format: *const wchar_t, ...) -> ::int_t;
    pub fn swprintf(s: *mut wchar_t, n: size_t, format: *const wchar_t, ...) -> ::int_t;
    pub fn fwscanf(stream: *mut FILE, format: *const wchar_t, ...) -> ::int_t;
    pub fn wscanf(format: *const wchar_t, ...) -> ::int_t;
    pub fn swscanf(s: *const wchar_t, format: *const wchar_t, ...) -> ::int_t;
    pub fn fgetwc(stream: *mut FILE) -> wint_t;
    pub fn getwc(stream: *mut FILE) -> wint_t;
    pub fn getwchar() -> wint_t;
    pub fn fputwc(wc: wchar_t, stream: *mut FILE) -> wint_t;
    pub fn putwc(wc: wchar_t, stream: *mut FILE) -> wint_t;
    pub fn putwchar(wc: wchar_t) -> wint_t;
    pub fn fgetws(ws: *mut wchar_t, n: ::int_t, stream: *mut ::stdio::FILE) -> *mut wchar_t;
    pub fn fputws(ws: *const wchar_t, stream: *mut ::stdio::FILE) -> ::int_t;
    pub fn ungetwc(wc: wint_t, stream: *mut ::stdio::FILE) -> wint_t;
    pub fn wcsftime(s: *mut wchar_t, maxsize: ::size_t, format: *const wchar_t, tp: *const ::time::tm) -> ::size_t;
}
*/
