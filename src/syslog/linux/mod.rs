pub use self::arch::{LOG_PID};
pub use self::arch::{LOG_CONS};
pub use self::arch::{LOG_NDELAY};
pub use self::arch::{LOG_ODELAY};
pub use self::arch::{LOG_NOWAIT};
pub use self::arch::{LOG_KERN};
pub use self::arch::{LOG_USER};
pub use self::arch::{LOG_MAIL};
pub use self::arch::{LOG_NEWS};
pub use self::arch::{LOG_UUCP};
pub use self::arch::{LOG_DAEMON};
pub use self::arch::{LOG_AUTH};
pub use self::arch::{LOG_CRON};
pub use self::arch::{LOG_LPR};
pub use self::arch::{LOG_LOCAL0};
pub use self::arch::{LOG_LOCAL1};
pub use self::arch::{LOG_LOCAL2};
pub use self::arch::{LOG_LOCAL3};
pub use self::arch::{LOG_LOCAL4};
pub use self::arch::{LOG_LOCAL5};
pub use self::arch::{LOG_LOCAL6};
pub use self::arch::{LOG_LOCAL7};
pub use self::arch::{LOG_EMERG};
pub use self::arch::{LOG_ALERT};
pub use self::arch::{LOG_CRIT};
pub use self::arch::{LOG_ERR};
pub use self::arch::{LOG_WARNING};
pub use self::arch::{LOG_NOTICE};
pub use self::arch::{LOG_INFO};
pub use self::arch::{LOG_DEBUG};
pub use self::arch::{LOG_MASK};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

