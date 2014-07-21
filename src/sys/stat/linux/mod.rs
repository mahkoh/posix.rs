pub use self::arch::{stat};
pub use self::arch::{S_IFMT};
pub use self::arch::{S_IFBLK};
pub use self::arch::{S_IFCHR};
pub use self::arch::{S_IFIFO};
pub use self::arch::{S_IFREG};
pub use self::arch::{S_IFDIR};
pub use self::arch::{S_IFLNK};
pub use self::arch::{S_IFSOCK};
pub use self::arch::{S_ISBLK};
pub use self::arch::{S_ISCHR};
pub use self::arch::{S_ISDIR};
pub use self::arch::{S_ISFIFO};
pub use self::arch::{S_ISREG};
pub use self::arch::{S_ISLNK};
pub use self::arch::{S_ISSOCK};
pub use self::arch::{S_TYPEISMQ};
pub use self::arch::{S_TYPEISSEM};
pub use self::arch::{S_TYPEISSHM};
pub use self::arch::{UTIME_NOW};
pub use self::arch::{UTIME_OMIT};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
