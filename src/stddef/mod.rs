pub use self::os::{ptrdiff_t};
pub use self::os::{wchar_t};
pub use size_t;

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;
