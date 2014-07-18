pub use self::os::{msgqnum_t};
pub use self::os::{msglen_t};
pub use self::os::{msqid_ds};
pub use self::os::{msginfo};
pub use self::os::{MSG_NOERROR};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn msgctl(msqid: ::int_t, cmd: ::int_t, buf: &mut msqid_ds) -> ::int_t {
    extern { fn msgctl(msqid: ::int_t, cmd: ::int_t, buf: *mut msqid_ds) -> ::int_t; }
    unsafe { msgctl(msqid, cmd, buf as *mut _) }
}

pub fn msgget(key: ::sys::types::key_t, msgflg: ::int_t) -> ::int_t {
    extern { fn msgget(key: ::sys::types::key_t, msgflg: ::int_t) -> ::int_t; }
    unsafe { msgget(key, msgflg) }
}

pub fn msgrcv(msqid: ::int_t, msgp: &mut [u8], msgtyp: ::long_t,
              msgflg: ::int_t) -> ::ssize_t {
    extern { fn msgrcv(msqid: ::int_t, msgp: *mut ::void_t, msgsz: ::size_t, msgtyp: ::long_t,
                  msgflg: ::int_t) -> ::ssize_t; }
    if msgp.len() < 1 {
        return -1;
    }
    unsafe { msgrcv(msqid, msgp.as_mut_ptr() as *mut _, msgp.len() as ::size_t - 1,
                    msgtyp, msgflg) }
}

pub fn msgsnd(msqid: ::int_t, msgp: &[u8], msgflg: ::int_t) -> ::int_t {
    extern { fn msgsnd(msqid: ::int_t, msgp: *const ::void_t, msgsz: ::size_t,
                       msgflg: ::int_t) -> ::int_t; }
    if msgp.len() < 1 {
        return -1;
    }
    unsafe { msgsnd(msqid, msgp.as_ptr() as *const _, msgp.len() as ::size_t - 1,
                    msgflg) }
}
