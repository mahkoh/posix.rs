pub use self::arch::{stat};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
