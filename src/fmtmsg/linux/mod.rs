pub use self::arch::{MM_HARD};
pub use self::arch::{MM_SOFT};
pub use self::arch::{MM_FIRM};
pub use self::arch::{MM_APPL};
pub use self::arch::{MM_UTIL};
pub use self::arch::{MM_OPSYS};
pub use self::arch::{MM_RECOVER};
pub use self::arch::{MM_NRECOV};
pub use self::arch::{MM_HALT};
pub use self::arch::{MM_ERROR};
pub use self::arch::{MM_WARNING};
pub use self::arch::{MM_INFO};
pub use self::arch::{MM_NOSEV};
pub use self::arch::{MM_PRINT};
pub use self::arch::{MM_CONSOLE};
pub use self::arch::{MM_OK};
pub use self::arch::{MM_NOTOK};
pub use self::arch::{MM_NOMSG};
pub use self::arch::{MM_NOCON};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
