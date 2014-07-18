pub use self::os::{sockaddr_un};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;
