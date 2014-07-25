pub use self::arch::{datum};
pub use self::arch::{DBM};
pub use self::arch::{DBM_INSERT};
pub use self::arch::{DBM_REPLACE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

