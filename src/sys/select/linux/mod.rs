pub use self::arch::{timeval};
pub use self::arch::{fd_set};
pub use self::arch::{FD_SETSIZE};
pub use self::arch::{FD_CLR};
pub use self::arch::{FD_ISSET};
pub use self::arch::{FD_SET};
pub use self::arch::{FD_ZERO};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

