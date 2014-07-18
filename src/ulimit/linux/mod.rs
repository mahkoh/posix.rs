pub use self::arch::{UL_GETFSIZE};
pub use self::arch::{UL_SETFSIZE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
