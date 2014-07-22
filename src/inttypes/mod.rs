pub use self::os::{imaxdiv_t};

use {char_t, int_t};
use stdint::{intmax_t, uintmax_t};
use stddef::{wchar_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn imaxabs(n: intmax_t) -> intmax_t {
    extern { fn imaxabs(n: intmax_t) -> intmax_t; }
    unsafe { imaxabs(n) }
}

pub fn imaxdiv(numer: intmax_t, denom: intmax_t) -> imaxdiv_t {
    extern { fn imaxdiv(numer: intmax_t, denom: intmax_t) -> imaxdiv_t; }
    unsafe { imaxdiv(numer, denom) }
}

extern "C" {
    pub fn strtoimax(nptr: *const char_t, endptr: *mut *mut char_t,
                     base: int_t) -> intmax_t;
    pub fn strtoumax(nptr: *const char_t, endptr: *mut *mut char_t,
                     base: int_t) -> uintmax_t;
    pub fn wcstoimax(nptr: *const wchar_t, endptr: *mut *mut wchar_t,
                     base: int_t) -> intmax_t;
    pub fn wcstoumax(nptr: *const wchar_t, endptr: *mut *mut wchar_t,
                     base: int_t) -> uintmax_t;
}

#[cfg(test)]
mod tests {
    use stdint::{intmax_t};
    use std::num::{Bounded};

    #[test]
    fn test_abs() {
        assert_eq!(super::imaxabs(-55), 55);
        assert_eq!(super::imaxabs(44), 44);
        let min: intmax_t = Bounded::min_value();
        assert_eq!(super::imaxabs(min + 55), (min + 55).abs());
    }

    #[test]
    fn test_div() {
        let div = super::imaxdiv(4, 2);
        assert_eq!(div.quot, 2);
        assert_eq!(div.rem, 0);
        let div = super::imaxdiv(5, 2);
        assert_eq!(div.quot, 2);
        assert_eq!(div.rem, 1);
    }
}
