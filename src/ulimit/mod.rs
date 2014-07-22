pub use self::os::{UL_GETFSIZE};
pub use self::os::{UL_SETFSIZE};

use {int_t, long_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

extern {
    pub fn ulimit(cmd: int_t, ...) -> long_t;
}
