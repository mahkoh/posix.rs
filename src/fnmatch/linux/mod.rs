pub use self::arch::{FNM_NOMATCH};
pub use self::arch::{FNM_PATHNAME};
pub use self::arch::{FNM_PERIOD};
pub use self::arch::{FNM_NOESCAPE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

