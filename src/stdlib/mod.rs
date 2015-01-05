pub use self::os::{div_t};
pub use self::os::{ldiv_t};
pub use self::os::{lldiv_t};
pub use self::os::{EXIT_FAILURE};
pub use self::os::{EXIT_SUCCESS};
pub use self::os::{RAND_MAX};
pub use self::os::{MB_CUR_MAX};

use {int_t, longlong_t, char_t, void_t, size_t, uint_t, double_t, ushort_t, ulonglong_t,
     float_t, long_t, ulong_t};
use stddef::{wchar_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub type compar_fn_t = extern fn(*const void_t, *const void_t) -> int_t;

extern {
    pub fn _Exit(status: int_t) -> !;
    pub fn a64l(s: *const char_t) -> long_t;
    pub fn abort();
    pub fn abs(x: int_t) -> int_t;
    pub fn atexit(func: extern fn()) -> int_t;
    pub fn atof(nptr: *const char_t) -> double_t;
    pub fn atoi(nptr: *const char_t) -> int_t;
    pub fn atoll(nptr: *const char_t) -> longlong_t;
    pub fn atol(nptr: *const char_t) -> long_t;
    pub fn bsearch(key: *const void_t, base: *const void_t, nmemb: size_t, size: size_t, compar: compar_fn_t) -> *mut void_t;
    pub fn calloc(nmemb: size_t, size: size_t) -> *mut void_t;
    pub fn div(numer: int_t, denom: int_t) -> div_t;
    pub fn drand48() -> double_t;
    pub fn erand48(xsubi: [ushort_t; 3u]) -> double_t;
    pub fn exit(status: int_t);
    pub fn free(ptr: *mut void_t);
    pub fn getenv(name: *const char_t) -> *mut char_t;
    pub fn getsubopt(optionp: *mut *mut char_t, tokens: *const *mut char_t, valuep: *mut *mut char_t) -> int_t;
    pub fn initstate(seed: uint_t, statebuf: *mut char_t, statelen: size_t) -> *mut char_t;
    pub fn jrand48(xsubi: [ushort_t; 3u]) -> long_t;
    pub fn l64a(n: long_t) -> *mut char_t;
    pub fn labs(x: long_t) -> long_t;
    pub fn lcong48(param: [ushort_t; 7u]);
    pub fn ldiv(numer: long_t, denom: long_t) -> ldiv_t;
    pub fn llabs(x: longlong_t) -> longlong_t;
    pub fn lldiv(numer: longlong_t, denom: longlong_t) -> lldiv_t;
    pub fn lrand48() -> long_t;
    pub fn malloc(size: size_t) -> *mut void_t;
    pub fn mblen(s: *const char_t, n: size_t) -> int_t;
    pub fn mbstowcs(pwcs: *mut wchar_t, s: *const char_t, n: size_t) -> size_t;
    pub fn mbtowc(pwc: *mut wchar_t, s: *const char_t, n: size_t) -> int_t;
    pub fn mkdtemp(template: *mut char_t) -> *mut char_t;
    pub fn mkstemp(template: *mut char_t) -> int_t;
    pub fn mktemp(template: *mut char_t) -> *mut char_t;
    pub fn mrand48() -> long_t;
    pub fn nrand48(xsubi: [ushort_t; 3u]) -> long_t;
    pub fn posix_memalign(memptr: *mut *mut void_t, alignment: size_t, size: size_t) -> int_t;
    pub fn putenv(string: *mut char_t) -> int_t;
    pub fn qsort(base: *mut void_t, nmemb: size_t, size: size_t, compar: compar_fn_t);
    pub fn rand_r(seed: *mut uint_t) -> int_t;
    pub fn rand() -> int_t;
    pub fn random() -> long_t;
    pub fn realloc(ptr: *mut void_t, size: size_t) -> *mut void_t;
    pub fn realpath(name: *const char_t, resolved: *mut char_t) -> *mut char_t;
    pub fn seed48(seed16v: [ushort_t; 3u]) -> *mut ushort_t;
    pub fn setenv(name: *const char_t, value: *const char_t, replace: int_t) -> int_t;
    pub fn setstate(statebuf: *mut char_t) -> *mut char_t;
    pub fn srand(seed: uint_t);
    pub fn srand48(seedval: long_t);
    pub fn srandom(seed: uint_t);
    pub fn strtod(nptr: *const char_t, endptr: *mut *mut char_t) -> double_t;
    pub fn strtof(nptr: *const char_t, endptr: *mut *mut char_t) -> float_t;
    pub fn strtol(nptr: *const char_t, endptr: *mut *mut char_t, base: int_t) -> long_t;
    pub fn strtold(nptr: *const char_t, endptr: *mut *mut char_t) -> double_t;
    pub fn strtoll(nptr: *const char_t, endptr: *mut *mut char_t, base: int_t) -> longlong_t;
    pub fn strtoul(nptr: *const char_t, endptr: *mut *mut char_t, base: int_t) -> ulong_t;
    pub fn strtoull(nptr: *const char_t, endptr: *mut *mut char_t, base: int_t) -> ulonglong_t;
    pub fn system(command: *const char_t) -> int_t;
    pub fn unsetenv(name: *const char_t) -> int_t;
    pub fn wcstombs(s: *mut char_t, pwcs: *const wchar_t, n: size_t) -> size_t;
    pub fn wctomb(s: *mut char_t, wchar: wchar_t) -> int_t;
}
