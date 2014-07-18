pub use self::os::{tms};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn times(buffer: &mut tms) -> ::sys::types::clock_t {
    extern { fn times(buffer: *mut tms) -> ::sys::types::clock_t; }
    unsafe { times(buffer as *mut _) }
}
