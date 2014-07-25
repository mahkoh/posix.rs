pub use self::arch::{flock};
pub use self::arch::{F_DUPFD};
pub use self::arch::{F_DUPFD_CLOEXEC};
pub use self::arch::{F_GETFD};
pub use self::arch::{F_SETFD};
pub use self::arch::{F_GETFL};
pub use self::arch::{F_SETFL};
pub use self::arch::{F_GETLK};
pub use self::arch::{F_SETLK};
pub use self::arch::{F_SETLKW};
pub use self::arch::{F_GETOWN};
pub use self::arch::{F_SETOWN};
pub use self::arch::{FD_CLOEXEC};
pub use self::arch::{F_RDLCK};
pub use self::arch::{F_UNLCK};
pub use self::arch::{F_WRLCK};
pub use self::arch::{O_CLOEXEC};
pub use self::arch::{O_CREAT};
pub use self::arch::{O_DIRECTORY};
pub use self::arch::{O_EXCL};
pub use self::arch::{O_NOCTTY};
pub use self::arch::{O_NOFOLLOW};
pub use self::arch::{O_TRUNC};
pub use self::arch::{O_APPEND};
pub use self::arch::{O_DSYNC};
pub use self::arch::{O_NONBLOCK};
pub use self::arch::{O_RSYNC};
pub use self::arch::{O_SYNC};
pub use self::arch::{O_ACCMODE};
pub use self::arch::{O_RDONLY};
pub use self::arch::{O_RDWR};
pub use self::arch::{O_WRONLY};
pub use self::arch::{AT_FDCWD};
pub use self::arch::{AT_EACCESS};
pub use self::arch::{AT_SYMLINK_NOFOLLOW};
pub use self::arch::{AT_SYMLINK_FOLLOW};
pub use self::arch::{AT_REMOVEDIR};
pub use self::arch::{POSIX_FADV_DONTNEED};
pub use self::arch::{POSIX_FADV_NOREUSE};
pub use self::arch::{POSIX_FADV_NORMAL};
pub use self::arch::{POSIX_FADV_RANDOM};
pub use self::arch::{POSIX_FADV_SEQUENTIAL};
pub use self::arch::{POSIX_FADV_WILLNEED};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

