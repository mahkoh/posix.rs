pub use self::arch::{ipc_perm};
pub use self::arch::{IPC_CREAT};
pub use self::arch::{IPC_EXCL};
pub use self::arch::{IPC_NOWAIT};
pub use self::arch::{IPC_PRIVATE};
pub use self::arch::{IPC_RMID};
pub use self::arch::{IPC_SET};
pub use self::arch::{IPC_STAT};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
