pub use self::os::{timeval};
pub use self::os::{fd_set};
pub use self::os::{FD_SETSIZE};
pub use self::os::{FD_CLR};
pub use self::os::{FD_ISSET};
pub use self::os::{FD_SET};
pub use self::os::{FD_ZERO};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn select(nfds: ::int_t, readfds: &mut fd_set, writefds: &mut fd_set,
              exceptfds: &mut fd_set, timeout: &mut timeval) -> ::int_t {
    extern { fn select(nfds: ::int_t, readfds: *mut fd_set,
                       writefds: *mut fd_set, exceptfds: *mut fd_set,
                       timeout: *mut timeval) -> ::int_t; }
    unsafe { select(nfds, readfds as *mut _, writefds as *mut _, exceptfds as *mut _,
                    timeout as *mut _) }
}

pub fn pselect(nfds: ::int_t, readfds: &mut fd_set, writefds: &mut fd_set,
               exceptfds: &mut fd_set, timeout: &::time::timespec,
               sigmask: &::signal::sigset_t) -> ::int_t {
    extern { fn pselect(nfds: ::int_t, readfds: *mut fd_set,
                        writefds: *mut fd_set, exceptfds: *mut fd_set,
                        timeout: *const ::time::timespec,
                        sigmask: *const ::signal::sigset_t) -> ::int_t; }
    unsafe { pselect(nfds, readfds as *mut _, writefds as *mut _, exceptfds as *mut _,
                     timeout as *const _, sigmask as *const _) }
}
