pub use self::arch::{PTHREAD_BARRIER_SERIAL_THREAD};
pub use self::arch::{PTHREAD_CANCEL_ASYNCHRONOUS};
pub use self::arch::{PTHREAD_CANCEL_ENABLE};
pub use self::arch::{PTHREAD_CANCEL_DEFERRED};
pub use self::arch::{PTHREAD_CANCEL_DISABLE};
pub use self::arch::{PTHREAD_CREATE_DETACHED};
pub use self::arch::{PTHREAD_CREATE_JOINABLE};
pub use self::arch::{PTHREAD_EXPLICIT_SCHED};
pub use self::arch::{PTHREAD_INHERIT_SCHED};
pub use self::arch::{PTHREAD_MUTEX_DEFAULT};
pub use self::arch::{PTHREAD_MUTEX_ERRORCHECK};
pub use self::arch::{PTHREAD_MUTEX_NORMAL};
pub use self::arch::{PTHREAD_MUTEX_RECURSIVE};
pub use self::arch::{PTHREAD_MUTEX_ROBUST};
pub use self::arch::{PTHREAD_MUTEX_STALLED};
pub use self::arch::{PTHREAD_ONCE_INIT};
pub use self::arch::{PTHREAD_PRIO_INHERIT};
pub use self::arch::{PTHREAD_PRIO_NONE};
pub use self::arch::{PTHREAD_PRIO_PROTECT};
pub use self::arch::{PTHREAD_PROCESS_SHARED};
pub use self::arch::{PTHREAD_PROCESS_PRIVATE};
pub use self::arch::{PTHREAD_SCOPE_PROCESS};
pub use self::arch::{PTHREAD_SCOPE_SYSTEM};
pub use self::arch::{PTHREAD_CANCELED};
pub use self::arch::{PTHREAD_COND_INITIALIZER};
pub use self::arch::{PTHREAD_MUTEX_INITIALIZER};
pub use self::arch::{PTHREAD_RWLOCK_INITIALIZER};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

