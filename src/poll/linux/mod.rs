pub use self::arch::{nfds_t};
pub use self::arch::{pollfd};
pub use self::arch::{POLLIN};
pub use self::arch::{POLLRDNORM};
pub use self::arch::{POLLRDBAND};
pub use self::arch::{POLLPRI};
pub use self::arch::{POLLOUT};
pub use self::arch::{POLLWRNORM};
pub use self::arch::{POLLWRBAND};
pub use self::arch::{POLLERR};
pub use self::arch::{POLLHUP};
pub use self::arch::{POLLNVAL};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

