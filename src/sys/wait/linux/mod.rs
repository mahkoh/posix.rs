pub use self::arch::{idtype_t};
pub use self::arch::{P_ALL};
pub use self::arch::{P_PID};
pub use self::arch::{P_PGID};
pub use self::arch::{WCONTINUED};
pub use self::arch::{WNOHANG};
pub use self::arch::{WUNTRACED};
pub use self::arch::{WEXITED};
pub use self::arch::{WNOWAIT};
pub use self::arch::{WSTOPPED};
pub use self::arch::{WEXITSTATUS};
pub use self::arch::{WIFCONTINUED};
pub use self::arch::{WIFEXITED};
pub use self::arch::{WIFSIGNALED};
pub use self::arch::{WIFSTOPPED};
pub use self::arch::{WSTOPSIG};
pub use self::arch::{WTERMSIG};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

