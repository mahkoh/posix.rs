pub use self::arch::{div_t};
pub use self::arch::{ldiv_t};
pub use self::arch::{lldiv_t};
pub use self::arch::{EXIT_FAILURE};
pub use self::arch::{EXIT_SUCCESS};
pub use self::arch::{RAND_MAX};
pub use self::arch::{MB_CUR_MAX};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

