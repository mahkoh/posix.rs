pub use self::arch::{posix_spawnattr_t};
pub use self::arch::{posix_spawn_file_actions_t};
pub use self::arch::{POSIX_SPAWN_RESETIDS};
pub use self::arch::{POSIX_SPAWN_SETPGROUP};
pub use self::arch::{POSIX_SPAWN_SETSCHEDPARAM};
pub use self::arch::{POSIX_SPAWN_SETSCHEDULER};
pub use self::arch::{POSIX_SPAWN_SETSIGDEF};
pub use self::arch::{POSIX_SPAWN_SETSIGMASK};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

