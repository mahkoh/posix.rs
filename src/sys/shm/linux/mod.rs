pub use self::arch::{shmatt_t};
pub use self::arch::{shmid_ds};
pub use self::arch::{SHM_RDONLY};
pub use self::arch::{SHM_RND};
pub use self::arch::{SHMLBA};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

