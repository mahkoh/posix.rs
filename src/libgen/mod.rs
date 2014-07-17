pub use self::os::{basename};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

extern {
    pub fn dirname(path: *mut ::char_t) -> *mut ::char_t;
}
