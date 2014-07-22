pub use self::os::{jmp_buf};
pub use self::os::{sigjmp_buf};
pub use self::os::{sigsetjmp};

use {int_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

extern {
    pub fn setjmp(env: jmp_buf) -> int_t;
    pub fn _setjmp(env: *mut jmp_buf) -> int_t;
    pub fn longjmp(env: *mut jmp_buf, val: int_t);
    pub fn _longjmp(env: *mut jmp_buf, val: int_t);
    pub fn siglongjmp(env: sigjmp_buf, val: int_t);
}
