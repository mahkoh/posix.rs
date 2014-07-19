pub use self::arch::{wordexp_t};
pub use self::arch::{WRDE_APPEND};
pub use self::arch::{WRDE_DOOFFS};
pub use self::arch::{WRDE_NOCMD};
pub use self::arch::{WRDE_REUSE};
pub use self::arch::{WRDE_SHOWERR};
pub use self::arch::{WRDE_UNDEF};
pub use self::arch::{WRDE_BADCHAR};
pub use self::arch::{WRDE_BADVAL};
pub use self::arch::{WRDE_CMDSUB};
pub use self::arch::{WRDE_NOSPACE};
pub use self::arch::{WRDE_SYNTAX};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
