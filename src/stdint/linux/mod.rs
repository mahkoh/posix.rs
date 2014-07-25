pub use self::arch::{int8_t};
pub use self::arch::{int16_t};
pub use self::arch::{int32_t};
pub use self::arch::{int64_t};
pub use self::arch::{uint8_t};
pub use self::arch::{uint16_t};
pub use self::arch::{uint32_t};
pub use self::arch::{uint64_t};
pub use self::arch::{int_least8_t};
pub use self::arch::{int_least16_t};
pub use self::arch::{int_least32_t};
pub use self::arch::{int_least64_t};
pub use self::arch::{uint_least8_t};
pub use self::arch::{uint_least16_t};
pub use self::arch::{uint_least32_t};
pub use self::arch::{uint_least64_t};
pub use self::arch::{int_fast8_t};
pub use self::arch::{int_fast16_t};
pub use self::arch::{int_fast32_t};
pub use self::arch::{int_fast64_t};
pub use self::arch::{uint_fast8_t};
pub use self::arch::{uint_fast16_t};
pub use self::arch::{uint_fast32_t};
pub use self::arch::{uint_fast64_t};
pub use self::arch::{intptr_t};
pub use self::arch::{uintptr_t};
pub use self::arch::{intmax_t};
pub use self::arch::{uintmax_t};

#[cfg(target_arch = "x86_64")]
#[path = "x86_64.rs"]
mod arch;

#[cfg(target_arch = "x86")]
#[path = "x86.rs"]
mod arch;

