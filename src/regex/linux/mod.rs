pub use self::arch::{regex_t};
pub use self::arch::{regoff_t};
pub use self::arch::{regmatch_t};
pub use self::arch::{REG_EXTENDED};
pub use self::arch::{REG_ICASE};
pub use self::arch::{REG_NOSUB};
pub use self::arch::{REG_NEWLINE};
pub use self::arch::{REG_NOTBOL};
pub use self::arch::{REG_NOTEOL};
pub use self::arch::{REG_NOMATCH};
pub use self::arch::{REG_BADPAT};
pub use self::arch::{REG_ECOLLATE};
pub use self::arch::{REG_ECTYPE};
pub use self::arch::{REG_EESCAPE};
pub use self::arch::{REG_ESUBREG};
pub use self::arch::{REG_EBRACK};
pub use self::arch::{REG_EPAREN};
pub use self::arch::{REG_EBRACE};
pub use self::arch::{REG_BADBR};
pub use self::arch::{REG_ERANGE};
pub use self::arch::{REG_ESPACE};
pub use self::arch::{REG_BADRPT};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
