pub use self::os::{sched_param};
pub use self::os::{SCHED_FIFO};
pub use self::os::{SCHED_RR};
pub use self::os::{SCHED_OTHER};

use {int_t};
use sys::types::{pid_t};
use time::{timespec};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn sched_get_priority_max(algorithm: int_t) -> int_t {
    extern { fn sched_get_priority_max(algorithm: int_t) -> int_t; }
    unsafe { sched_get_priority_max(algorithm) }
}

pub fn sched_get_priority_min(algorithm: int_t) -> int_t {
    extern { fn sched_get_priority_min(algorithm: int_t) -> int_t; }
    unsafe { sched_get_priority_min(algorithm) }
}

pub fn sched_getparam(pid: pid_t, param: &mut sched_param) -> int_t {
    extern { fn sched_getparam(pid: pid_t,
                               param: *mut sched_param) -> int_t; }
    unsafe { sched_getparam(pid, param as *mut _) }
}

pub fn sched_getscheduler(pid: pid_t) -> int_t {
    extern { fn sched_getscheduler(pid: pid_t) -> int_t; }
    unsafe { sched_getscheduler(pid) }
}

pub fn sched_rr_get_interval(pid: pid_t, t: &mut timespec) -> int_t {
    extern { fn sched_rr_get_interval(pid: pid_t, t: *mut timespec) -> int_t; }
    unsafe { sched_rr_get_interval(pid, t as *mut _) }
}

pub fn sched_setparam(pid: pid_t, param: &sched_param) -> int_t {
    extern { fn sched_setparam(pid: pid_t, param: *const sched_param) -> int_t; }
    unsafe { sched_setparam(pid, param as *const _) }
}

pub fn sched_setscheduler(pid: pid_t, policy: int_t, param: &sched_param) -> int_t {
    extern { fn sched_setscheduler(pid: pid_t, policy: int_t,
                                   param: *const sched_param) -> int_t; }
    unsafe { sched_setscheduler(pid, policy, param as *const _) }
}

pub fn sched_yield() -> int_t {
    extern { fn sched_yield() -> int_t; }
    unsafe { sched_yield() }
}
