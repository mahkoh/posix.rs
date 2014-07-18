pub use self::os::{UL_GETFSIZE};
pub use self::os::{UL_SETFSIZE};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

extern "C" {
    pub fn ulimit(cmd: ::int_t, ...) -> ::long_t;
}
