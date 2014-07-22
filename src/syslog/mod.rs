pub use self::os::{LOG_PID};
pub use self::os::{LOG_CONS};
pub use self::os::{LOG_NDELAY};
pub use self::os::{LOG_ODELAY};
pub use self::os::{LOG_NOWAIT};
pub use self::os::{LOG_KERN};
pub use self::os::{LOG_USER};
pub use self::os::{LOG_MAIL};
pub use self::os::{LOG_NEWS};
pub use self::os::{LOG_UUCP};
pub use self::os::{LOG_DAEMON};
pub use self::os::{LOG_AUTH};
pub use self::os::{LOG_CRON};
pub use self::os::{LOG_LPR};
pub use self::os::{LOG_LOCAL0};
pub use self::os::{LOG_LOCAL1};
pub use self::os::{LOG_LOCAL2};
pub use self::os::{LOG_LOCAL3};
pub use self::os::{LOG_LOCAL4};
pub use self::os::{LOG_LOCAL5};
pub use self::os::{LOG_LOCAL6};
pub use self::os::{LOG_LOCAL7};
pub use self::os::{LOG_EMERG};
pub use self::os::{LOG_ALERT};
pub use self::os::{LOG_CRIT};
pub use self::os::{LOG_ERR};
pub use self::os::{LOG_WARNING};
pub use self::os::{LOG_NOTICE};
pub use self::os::{LOG_INFO};
pub use self::os::{LOG_DEBUG};
pub use self::os::{LOG_MASK};

use {NTStr, char_t, int_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn closelog() {
    extern { fn closelog(); }
    unsafe { closelog(); }
}

pub fn openlog<T: NTStr>(ident: &T, option: int_t, facility: int_t) {
    extern { fn openlog(ident: *const char_t, option: int_t, facility: int_t); }
    unsafe { openlog(ident.as_ptr(), option, facility) }
}

pub fn setlogmask(mask: int_t) -> int_t {
    extern { fn setlogmask(mask: int_t) -> int_t; }
    unsafe { setlogmask(mask) }
}

extern {
    pub fn syslog(pri: int_t, fmt: *const char_t, ...);
}
