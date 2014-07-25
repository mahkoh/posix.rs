pub use self::arch::{PROT_EXEC};
pub use self::arch::{PROT_NONE};
pub use self::arch::{PROT_READ};
pub use self::arch::{PROT_WRITE};
pub use self::arch::{MAP_FIXED};
pub use self::arch::{MAP_PRIVATE};
pub use self::arch::{MAP_SHARED};
pub use self::arch::{MS_ASYNC};
pub use self::arch::{MS_INVALIDATE};
pub use self::arch::{MS_SYNC};
pub use self::arch::{MCL_CURRENT};
pub use self::arch::{MCL_FUTURE};
pub use self::arch::{MAP_FAILED};
pub use self::arch::{POSIX_MADV_DONTNEED};
pub use self::arch::{POSIX_MADV_NORMAL};
pub use self::arch::{POSIX_MADV_RANDOM};
pub use self::arch::{POSIX_MADV_SEQUENTIAL};
pub use self::arch::{POSIX_MADV_WILLNEED};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

