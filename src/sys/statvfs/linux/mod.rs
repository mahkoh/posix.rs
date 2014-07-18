pub use self::arch::{statvfs};
pub use self::arch::{ST_RDONLY};
pub use self::arch::{ST_NOSUID};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
