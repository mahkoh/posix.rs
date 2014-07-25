pub use self::arch::{msgqnum_t};
pub use self::arch::{msglen_t};
pub use self::arch::{msqid_ds};
pub use self::arch::{msginfo};
pub use self::arch::{MSG_NOERROR};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

