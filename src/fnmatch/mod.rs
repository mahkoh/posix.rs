pub use self::os::{FNM_NOMATCH};
pub use self::os::{FNM_PATHNAME};
pub use self::os::{FNM_PERIOD};
pub use self::os::{FNM_NOESCAPE};

use {NTStr, int_t, char_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn fnmatch<T: NTStr, U: NTStr>(pattern: &T, string: &U, flags: int_t) -> int_t {
    extern {
        fn fnmatch(pattern: *const char_t, name: *const char_t, flags: int_t) -> int_t;
    }
    unsafe { fnmatch(pattern.as_ptr(), string.as_ptr(), flags) }
}

#[cfg(test)]
mod tests {
    use {ToNTStr};

    #[test]
    fn test() {
        let pat = "abc*123".to_nt_str();
        let stn = "abcTE/ST123".to_nt_str();
        let pat2 = "*123".to_nt_str();
        let stn2 = ".123".to_nt_str();
        assert_eq!(super::fnmatch(&pat,  &stn,  0),                   0);
        assert_eq!(super::fnmatch(&pat,  &stn,  super::FNM_PATHNAME), super::FNM_NOMATCH);
        assert_eq!(super::fnmatch(&pat2, &stn2, super::FNM_PATHNAME), 0);
        assert_eq!(super::fnmatch(&pat,  &stn,  super::FNM_PERIOD),   0);
        assert_eq!(super::fnmatch(&pat2, &stn2, super::FNM_PERIOD),   super::FNM_NOMATCH);
    }
}
