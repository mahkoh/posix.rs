pub use self::arch::{nl_catd};
pub use self::arch::{nl_item};
pub use self::arch::{NL_SETD};
pub use self::arch::{NL_CAT_LOCALE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
