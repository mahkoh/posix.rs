pub use self::arch::{AIO_PRIO_DELTA_MAX};
pub use self::arch::{DELAYTIMER_MAX};
pub use self::arch::{HOST_NAME_MAX};
pub use self::arch::{LOGIN_NAME_MAX};
pub use self::arch::{MQ_PRIO_MAX};
pub use self::arch::{PTHREAD_DESTRUCTOR_ITERATIONS};
pub use self::arch::{PTHREAD_KEYS_MAX};
pub use self::arch::{PTHREAD_STACK_MIN};
pub use self::arch::{RE_DUP_MAX};
pub use self::arch::{RTSIG_MAX};
pub use self::arch::{SEM_VALUE_MAX};
pub use self::arch::{TTY_NAME_MAX};
pub use self::arch::{MAX_CANON};
pub use self::arch::{MAX_INPUT};
pub use self::arch::{NAME_MAX};
pub use self::arch::{PATH_MAX};
pub use self::arch::{PIPE_BUF};
pub use self::arch::{BC_BASE_MAX};
pub use self::arch::{BC_DIM_MAX};
pub use self::arch::{BC_SCALE_MAX};
pub use self::arch::{BC_STRING_MAX};
pub use self::arch::{CHARCLASS_NAME_MAX};
pub use self::arch::{COLL_WEIGHTS_MAX};
pub use self::arch::{EXPR_NEST_MAX};
pub use self::arch::{LINE_MAX};
pub use self::arch::{NGROUPS_MAX};
pub use self::arch::{_POSIX_CLOCKRES_MIN};
pub use self::arch::{CHAR_BIT};
pub use self::arch::{CHAR_MAX};
pub use self::arch::{CHAR_MIN};
pub use self::arch::{INT_MAX};
pub use self::arch::{INT_MIN};
pub use self::arch::{LLONG_MAX};
pub use self::arch::{LLONG_MIN};
pub use self::arch::{LONG_MAX};
pub use self::arch::{LONG_MIN};
pub use self::arch::{MB_LEN_MAX};
pub use self::arch::{SCHAR_MAX};
pub use self::arch::{SCHAR_MIN};
pub use self::arch::{SHRT_MAX};
pub use self::arch::{SHRT_MIN};
pub use self::arch::{SSIZE_MAX};
pub use self::arch::{UCHAR_MAX};
pub use self::arch::{UINT_MAX};
pub use self::arch::{ULLONG_MAX};
pub use self::arch::{ULONG_MAX};
pub use self::arch::{USHRT_MAX};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

