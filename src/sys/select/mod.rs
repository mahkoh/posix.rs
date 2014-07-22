pub use self::os::{timeval};
pub use self::os::{fd_set};
pub use self::os::{FD_SETSIZE};
pub use self::os::{FD_CLR};
pub use self::os::{FD_ISSET};
pub use self::os::{FD_SET};
pub use self::os::{FD_ZERO};

use {int_t};
use time::{timespec};
use signal::{sigset_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn select(nfds: int_t, readfds: Option<&mut fd_set>, writefds: Option<&mut fd_set>,
              exceptfds: Option<&mut fd_set>, timeout: Option<&mut timeval>) -> int_t {
    extern { fn select(nfds: int_t, readfds: *mut fd_set,
                       writefds: *mut fd_set, exceptfds: *mut fd_set,
                       timeout: *mut timeval) -> int_t; }
    let readfds_ptr   = readfds.map(|v|   v as *mut _).unwrap_or(0 as *mut _);
    let writefds_ptr  = writefds.map(|v|  v as *mut _).unwrap_or(0 as *mut _);
    let exceptfds_ptr = exceptfds.map(|v| v as *mut _).unwrap_or(0 as *mut _);
    let timeout_ptr   = timeout.map(|v|   v as *mut _).unwrap_or(0 as *mut _);
    unsafe { select(nfds, readfds_ptr, writefds_ptr, exceptfds_ptr, timeout_ptr) }
}

pub fn pselect(nfds: int_t, readfds: Option<&mut fd_set>, writefds: Option<&mut fd_set>,
               exceptfds: Option<&mut fd_set>, timeout: Option<&timespec>,
               sigmask: &sigset_t) -> int_t {
    extern { fn pselect(nfds: int_t, readfds: *mut fd_set,
                        writefds: *mut fd_set, exceptfds: *mut fd_set,
                        timeout: *const timespec,
                        sigmask: *const sigset_t) -> int_t; }
    let readfds_ptr   = readfds.map(|v|   v as *mut _).unwrap_or(0 as *mut _);
    let writefds_ptr  = writefds.map(|v|  v as *mut _).unwrap_or(0 as *mut _);
    let exceptfds_ptr = exceptfds.map(|v| v as *mut _).unwrap_or(0 as *mut _);
    let timeout_ptr   = timeout.map(|v|   v as *const _).unwrap_or(0 as *const _);
    unsafe { pselect(nfds, readfds_ptr, writefds_ptr, exceptfds_ptr, timeout_ptr,
                     sigmask as *const _) }
}
