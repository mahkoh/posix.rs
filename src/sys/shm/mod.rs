pub use self::os::{shmatt_t};
pub use self::os::{shmid_ds};
pub use self::os::{SHM_RDONLY};
pub use self::os::{SHM_RND};
pub use self::os::{SHMLBA};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn shmctl(shmid: ::int_t, cmd: ::int_t, buf: &mut shmid_ds) -> ::int_t {
    extern { fn shmctl(shmid: ::int_t, cmd: ::int_t, buf: *mut shmid_ds) -> ::int_t; }
    unsafe { shmctl(shmid, cmd, buf as *mut _) }
}

pub fn shmget(key: ::sys::types::key_t, size: ::size_t, shmflg: ::int_t) -> ::int_t {
    extern { fn shmget(key: ::sys::types::key_t, size: ::size_t,
                       shmflg: ::int_t) -> ::int_t; }
    unsafe { shmget(key, size, shmflg) }
}

pub fn shmat(shmid: ::int_t, shmaddr: *const ::void_t, shmflg: ::int_t) -> *mut ::void_t {
    extern { fn shmat(shmid: ::int_t, shmaddr: *const ::void_t,
                      shmflg: ::int_t) -> *mut ::void_t; }
    unsafe { shmat(shmid, shmaddr, shmflg) }
}

pub fn shmdt(shmaddr: *const ::void_t) -> ::int_t {
    extern { fn shmdt(shmaddr: *const ::void_t) -> ::int_t; }
    unsafe { shmdt(shmaddr) }
}
