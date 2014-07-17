pub use self::arch::{lconv};
pub use self::arch::{locale_t};
pub use self::arch::{LC_ALL};
pub use self::arch::{LC_COLLATE};
pub use self::arch::{LC_CTYPE};
pub use self::arch::{LC_MESSAGES};
pub use self::arch::{LC_MONETARY};
pub use self::arch::{LC_NUMERIC};
pub use self::arch::{LC_TIME};
pub use self::arch::{LC_COLLATE_MASK};
pub use self::arch::{LC_CTYPE_MASK};
pub use self::arch::{LC_MESSAGES_MASK};
pub use self::arch::{LC_MONETARY_MASK};
pub use self::arch::{LC_NUMERIC_MASK};
pub use self::arch::{LC_TIME_MASK};
pub use self::arch::{LC_ALL_MASK};
pub use self::arch::{LC_GLOBAL_LOCALE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
