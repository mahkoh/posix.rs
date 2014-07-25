pub use self::arch::{FTW};
pub use self::arch::{FTW_F};
pub use self::arch::{FTW_D};
pub use self::arch::{FTW_DNR};
pub use self::arch::{FTW_DP};
pub use self::arch::{FTW_NS};
pub use self::arch::{FTW_SL};
pub use self::arch::{FTW_SLN};
pub use self::arch::{FTW_PHYS};
pub use self::arch::{FTW_MOUNT};
pub use self::arch::{FTW_DEPTH};
pub use self::arch::{FTW_CHDIR};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

