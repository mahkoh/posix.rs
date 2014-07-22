pub use self::os::{tms};

use sys::types::{clock_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn times(buffer: &mut tms) -> clock_t {
    extern { fn times(buffer: *mut tms) -> clock_t; }
    unsafe { times(buffer as *mut _) }
}
