pub use self::arch::{sem_t};
pub use self::arch::{SEM_FAILED};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
