pub use self::arch::{RTLD_LAZY};
pub use self::arch::{RTLD_NOW};
pub use self::arch::{RTLD_GLOBAL};
pub use self::arch::{RTLD_LOCAL};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
