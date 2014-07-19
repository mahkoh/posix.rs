pub use self::arch::{wint_t};
pub use self::arch::{wctype_t};
pub use self::arch::{mbstate_t};
pub use self::arch::{WEOF};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
