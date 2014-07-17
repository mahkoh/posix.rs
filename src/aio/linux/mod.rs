pub use self::arch::{aiocb};
pub use self::arch::{AIO_ALLDONE};
pub use self::arch::{AIO_CANCELED};
pub use self::arch::{AIO_NOTCANCELED};
pub use self::arch::{LIO_NOP};
pub use self::arch::{LIO_NOWAIT};
pub use self::arch::{LIO_READ};
pub use self::arch::{LIO_WAIT};
pub use self::arch::{LIO_WRITE};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;
