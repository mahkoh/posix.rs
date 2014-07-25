pub use self::arch::{glob_t};
pub use self::arch::{GLOB_APPEND};
pub use self::arch::{GLOB_DOOFFS};
pub use self::arch::{GLOB_ERR};
pub use self::arch::{GLOB_MARK};
pub use self::arch::{GLOB_NOCHECK};
pub use self::arch::{GLOB_NOESCAPE};
pub use self::arch::{GLOB_NOSORT};
pub use self::arch::{GLOB_ABORTED};
pub use self::arch::{GLOB_NOMATCH};
pub use self::arch::{GLOB_NOSPACE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

