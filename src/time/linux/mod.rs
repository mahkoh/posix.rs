pub use self::arch::{timespec};
pub use self::arch::{tm};
pub use self::arch::{itimerspec};
pub use self::arch::{CLOCKS_PER_SEC};
pub use self::arch::{CLOCK_MONOTONIC};
pub use self::arch::{CLOCK_PROCESS_CPUTIME_ID};
pub use self::arch::{CLOCK_REALTIME};
pub use self::arch::{CLOCK_THREAD_CPUTIME_ID};
pub use self::arch::{TIMER_ABSTIME};
pub use self::arch::{getdate_err};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

