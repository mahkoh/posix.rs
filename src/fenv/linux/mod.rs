pub use self::arch::{fexcept_t};
pub use self::arch::{fenv_t};
pub use self::arch::{FE_DIVBYZERO};
pub use self::arch::{FE_INEXACT};
pub use self::arch::{FE_INVALID};
pub use self::arch::{FE_OVERFLOW};
pub use self::arch::{FE_UNDERFLOW};
pub use self::arch::{FE_ALL_EXCEPT};
pub use self::arch::{FE_DOWNWARD};
pub use self::arch::{FE_TONEAREST};
pub use self::arch::{FE_TOWARDZERO};
pub use self::arch::{FE_UPWARD};
pub use self::arch::{FE_DFL_ENV};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

