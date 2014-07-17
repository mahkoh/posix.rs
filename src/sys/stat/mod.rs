pub use self::os::{stat};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;
