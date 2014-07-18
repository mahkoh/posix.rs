pub use self::arch::{timeval};
pub use self::arch::{itimerval};
pub use self::arch::{ITIMER_REAL};
pub use self::arch::{ITIMER_VIRTUAL};
pub use self::arch::{ITIMER_PROF};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
