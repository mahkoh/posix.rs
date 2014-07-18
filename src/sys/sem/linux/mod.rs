pub use self::arch::{semid_ds};
pub use self::arch::{sembuf};
pub use self::arch::{SEM_UNDO};
pub use self::arch::{GETNCNT};
pub use self::arch::{GETPID};
pub use self::arch::{GETVAL};
pub use self::arch::{GETALL};
pub use self::arch::{GETZCNT};
pub use self::arch::{SETVAL};
pub use self::arch::{SETALL};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
