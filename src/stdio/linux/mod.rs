pub use self::arch::{FILE};
pub use self::arch::{fpos_t};
pub use self::arch::{BUFSIZ};
pub use self::arch::{L_ctermid};
pub use self::arch::{L_tmpnam};
pub use self::arch::{_IOFBF};
pub use self::arch::{_IOLBF};
pub use self::arch::{_IONBF};
pub use self::arch::{SEEK_CUR};
pub use self::arch::{SEEK_END};
pub use self::arch::{SEEK_SET};
pub use self::arch::{FILENAME_MAX};
pub use self::arch::{FOPEN_MAX};
pub use self::arch::{TMP_MAX};
pub use self::arch::{EOF};
pub use self::arch::{stdin};
pub use self::arch::{stdout};
pub use self::arch::{stderr};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

