pub use self::os::{ipc_perm};
pub use self::os::{IPC_CREAT};
pub use self::os::{IPC_EXCL};
pub use self::os::{IPC_NOWAIT};
pub use self::os::{IPC_PRIVATE};
pub use self::os::{IPC_RMID};
pub use self::os::{IPC_SET};
pub use self::os::{IPC_STAT};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn ftok<T: ::NTStr>(pathname: &T, proj_id: ::int_t) -> ::sys::types::key_t {
    extern { fn ftok(pathname: *const ::char_t,
                     proj_id: ::int_t) -> ::sys::types::key_t; }
    unsafe { ftok(pathname.as_ptr(), proj_id) }
}
