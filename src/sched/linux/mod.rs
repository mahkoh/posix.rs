pub use self::arch::{sched_param};
pub use self::arch::{SCHED_FIFO};
pub use self::arch::{SCHED_RR};
pub use self::arch::{SCHED_OTHER};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
