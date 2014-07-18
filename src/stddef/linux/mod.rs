pub use self::arch::{ptrdiff_t};
pub use self::arch::{wchar_t};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
