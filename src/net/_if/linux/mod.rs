pub use self::arch::{if_nameindex};
pub use self::arch::{IF_NAMESIZE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
