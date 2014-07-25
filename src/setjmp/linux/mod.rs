pub use self::arch::{jmp_buf};
pub use self::arch::{sigjmp_buf};
pub use self::arch::{sigsetjmp};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

