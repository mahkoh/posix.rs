pub use self::os::{nfds_t};
pub use self::os::{pollfd};
pub use self::os::{POLLIN};
pub use self::os::{POLLRDNORM};
pub use self::os::{POLLRDBAND};
pub use self::os::{POLLPRI};
pub use self::os::{POLLOUT};
pub use self::os::{POLLWRNORM};
pub use self::os::{POLLWRBAND};
pub use self::os::{POLLERR};
pub use self::os::{POLLHUP};
pub use self::os::{POLLNVAL};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn poll(fds: &mut [pollfd], timeout: ::int_t) -> ::int_t {
    extern { fn poll(fds: *mut pollfd, nfds: nfds_t, timeout: ::int_t) -> ::int_t; }
    unsafe { poll(fds.as_mut_ptr(), fds.len() as nfds_t, timeout) }
}
