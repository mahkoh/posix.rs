pub use self::os::{mqd_t};
pub use self::os::{mq_attr};

use {NTStr, int_t, uint_t, char_t, size_t, ssize_t};
use signal::{sigevent};
use time::{timespec};
use sys::types::{mode_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn mq_close(mqdes: mqd_t) -> int_t {
    extern { fn mq_close(mqdes: mqd_t) -> int_t; }
    unsafe { mq_close(mqdes) }
}

pub fn mq_getattr(mqdes: mqd_t, mqstat: &mut mq_attr) -> int_t {
    extern { fn mq_getattr(mqdes: mqd_t, mqstat: *mut mq_attr) -> int_t; }
    unsafe { mq_getattr(mqdes, mqstat as *mut _) }
}

pub fn mq_notify(mqdes: mqd_t, notification: &sigevent) -> int_t {
    extern { fn mq_notify(mqdes: mqd_t,
                          notification: *const sigevent) -> int_t; }
    unsafe { mq_notify(mqdes, notification as *const _) }
}

pub fn mq_receive(mqdes: mqd_t, buf: &mut [u8],
                  msg_prio: Option<&mut uint_t>) -> ssize_t {
    extern { fn mq_receive(mqdes: mqd_t, msg_ptr: *mut char_t, msg_len: size_t,
                           msg_prio: *mut uint_t) -> ssize_t; }
    let ptr = msg_prio.map(|v| v as *mut _).unwrap_or(0 as *mut uint_t);
    unsafe { mq_receive(mqdes, buf.as_mut_ptr() as *mut _, buf.len() as size_t, ptr) }
}

pub fn mq_send(mqdes: mqd_t, buf: &[u8], prio: uint_t) -> int_t {
    extern { fn mq_send(mqdes: mqd_t, msg_ptr: *const char_t, msg_len: size_t,
                        msg_prio: uint_t) -> int_t; }
    unsafe { mq_send(mqdes, buf.as_ptr() as *const _, buf.len() as size_t, prio) }
}

pub fn mq_setattr(mqdes: mqd_t, newattr: &mq_attr,
                  oldattr: Option<&mut mq_attr>) -> int_t {
    extern { fn mq_setattr(mqdes: mqd_t, mqstat: *const mq_attr,
                           omqstat: *mut mq_attr) -> int_t; }
    let ptr = oldattr.map(|v| v as *mut _).unwrap_or(0 as *mut _);
    unsafe { mq_setattr(mqdes, newattr as *const _, ptr) }
}

pub fn mq_timedreceive(mqdes: mqd_t, buf: &mut [u8], prio: Option<&mut uint_t>,
                       timeout: &timespec) -> ssize_t {
    extern {
        fn mq_timedreceive(mqdes: mqd_t, msg_ptr: *mut char_t,
                           msg_len: size_t, msg_prio: *mut uint_t,
                           abs_timeout: *const timespec) -> ssize_t;
    }
    let ptr = prio.map(|v| v as *mut _).unwrap_or(0 as *mut uint_t);
    unsafe { mq_timedreceive(mqdes, buf.as_mut_ptr() as *mut _, buf.len() as size_t,
                             ptr, timeout as *const _) }
}

pub fn mq_timedsend(mqdes: mqd_t, buf: &[u8], prio: uint_t,
                    timeout: &timespec) -> int_t {
    extern { fn mq_timedsend(mqdes: mqd_t, msg_ptr: *const char_t,
                             msg_len: size_t, msg_prio: uint_t,
                             abs_timeout: *const timespec) -> int_t; }
    unsafe { mq_timedsend(mqdes, buf.as_ptr() as *const _, buf.len() as size_t,
                          prio, timeout as *const _) }
}

pub fn mq_unlink<T: NTStr>(name: &T) -> int_t {
    extern { fn mq_unlink(name: *const char_t) -> int_t; }
    unsafe { mq_unlink(name.as_ptr()) }
}

pub fn mq_open<T: NTStr>(name: &T, oflag: int_t, mode: Option<mode_t>,
                         attr: Option<&mut mq_attr>) -> mqd_t {
    extern { fn mq_open(name: *const char_t, oflag: int_t, ...) -> mqd_t; }
    match mode {
        Some(m) => {
            let ptr = attr.map(|v| v as *mut mq_attr).unwrap_or(0 as *mut _);
            unsafe { mq_open(name.as_ptr(), oflag, m, ptr) }
        },
        _ => unsafe { mq_open(name.as_ptr(), oflag) },
    }
}
