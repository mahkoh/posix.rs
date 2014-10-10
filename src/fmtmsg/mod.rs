pub use self::os::{MM_HARD};
pub use self::os::{MM_SOFT};
pub use self::os::{MM_FIRM};
pub use self::os::{MM_APPL};
pub use self::os::{MM_UTIL};
pub use self::os::{MM_OPSYS};
pub use self::os::{MM_RECOVER};
pub use self::os::{MM_NRECOV};
pub use self::os::{MM_HALT};
pub use self::os::{MM_ERROR};
pub use self::os::{MM_WARNING};
pub use self::os::{MM_INFO};
pub use self::os::{MM_NOSEV};
pub use self::os::{MM_PRINT};
pub use self::os::{MM_CONSOLE};
pub use self::os::{MM_OK};
pub use self::os::{MM_NOTOK};
pub use self::os::{MM_NOMSG};
pub use self::os::{MM_NOCON};

use {NTStr, int_t, long_t, char_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub const MM_NULLSEV: int_t = 0;
pub const MM_NULLMC: long_t = 0;

pub fn fmtmsg<T: NTStr, U: NTStr, V: NTStr, W: NTStr>
             (class: long_t, label: Option<&T>, severity: int_t, text: Option<&U>,
              action: Option<&V>, tag: Option<&W>) -> int_t {
    extern {
        fn fmtmsg(classification: long_t, label: *const char_t,
                  severity: int_t, text: *const char_t,
                  action: *const char_t, tag: *const char_t) -> int_t;
    }
    let label  = label.map(|v|  v.as_ptr()).unwrap_or(0 as *const char_t);
    let text   = text.map(|v|   v.as_ptr()).unwrap_or(0 as *const char_t);
    let action = action.map(|v| v.as_ptr()).unwrap_or(0 as *const char_t);
    let tag    = tag.map(|v|    v.as_ptr()).unwrap_or(0 as *const char_t);
    unsafe { fmtmsg(class, label, severity, text, action, tag) }
}
