pub use self::arch::{dirent};
pub use self::arch::{DIR};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
