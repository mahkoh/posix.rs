pub use self::os::{utmpx};
pub use self::os::{EMPTY};
pub use self::os::{BOOT_TIME};
pub use self::os::{OLD_TIME};
pub use self::os::{NEW_TIME};
pub use self::os::{USER_PROCESS};
pub use self::os::{INIT_PROCESS};
pub use self::os::{LOGIN_PROCESS};
pub use self::os::{DEAD_PROCESS};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

extern "C" {
    pub fn setutxent();
    pub fn endutxent();
    pub fn getutxent() -> *mut utmpx;
    pub fn getutxid(id: *const utmpx) -> *mut utmpx;
    pub fn getutxline(line: *const utmpx) -> *mut utmpx;
    pub fn pututxline(utmpx: *const utmpx) -> *mut utmpx;
}
