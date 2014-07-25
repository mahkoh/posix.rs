pub use self::arch::{utmpx};
pub use self::arch::{EMPTY};
pub use self::arch::{BOOT_TIME};
pub use self::arch::{OLD_TIME};
pub use self::arch::{NEW_TIME};
pub use self::arch::{USER_PROCESS};
pub use self::arch::{INIT_PROCESS};
pub use self::arch::{LOGIN_PROCESS};
pub use self::arch::{DEAD_PROCESS};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

