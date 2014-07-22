pub use self::os::{utimbuf};

use {NTStr, char_t, int_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn utime<T: NTStr>(file: &T, file_times: &utimbuf) -> int_t {
    extern { fn utime(file: *const char_t, file_times: *const utimbuf) -> int_t; }
    unsafe { utime(file.as_ptr(), file_times as *const _) }
}
