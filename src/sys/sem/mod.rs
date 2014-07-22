pub use self::os::{semid_ds};
pub use self::os::{sembuf};
pub use self::os::{SEM_UNDO};
pub use self::os::{GETNCNT};
pub use self::os::{GETPID};
pub use self::os::{GETVAL};
pub use self::os::{GETALL};
pub use self::os::{GETZCNT};
pub use self::os::{SETVAL};
pub use self::os::{SETALL};

use {int_t, size_t};
use sys::types::{key_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn semget(key: key_t, nsems: int_t, semflg: int_t) -> int_t {
    extern { fn semget(key: key_t, nsems: int_t, semflg: int_t) -> int_t; }
    unsafe { semget(key, nsems, semflg) }
}

pub fn semop(semid: int_t, sops: &mut sembuf, nsops: size_t) -> int_t {
    extern { fn semop(semid: int_t, sops: *mut sembuf,
                         nsops: size_t) -> int_t; }
    unsafe { semop(semid, sops as *mut _, nsops) }
}

extern {
    pub fn semctl(semid: int_t, semnum: int_t, cmd: int_t, ...) -> int_t;
}
