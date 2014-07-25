pub use self::arch::{rlim_t};
pub use self::arch::{rlimit};
pub use self::arch::{rusage};
pub use self::arch::{PRIO_PROCESS};
pub use self::arch::{PRIO_PGRP};
pub use self::arch::{PRIO_USER};
pub use self::arch::{RLIM_INFINITY};
pub use self::arch::{RLIM_SAVED_MAX};
pub use self::arch::{RLIM_SAVED_CUR};
pub use self::arch::{RUSAGE_SELF};
pub use self::arch::{RUSAGE_CHILDREN};
pub use self::arch::{RLIMIT_CORE};
pub use self::arch::{RLIMIT_CPU};
pub use self::arch::{RLIMIT_DATA};
pub use self::arch::{RLIMIT_FSIZE};
pub use self::arch::{RLIMIT_NOFILE};
pub use self::arch::{RLIMIT_STACK};
pub use self::arch::{RLIMIT_AS};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

