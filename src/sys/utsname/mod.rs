pub use self::os::{utsname};

use {int_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn uname(name: &mut utsname) -> int_t {
    extern { fn uname(name: *mut utsname) -> int_t; }
    unsafe { uname(name as *mut _) }
}
