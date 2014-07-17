pub use self::os::{TCP_NODELAY};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;
