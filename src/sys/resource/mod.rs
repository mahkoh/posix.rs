pub use self::os::{rlim_t};
pub use self::os::{rlimit};
pub use self::os::{rusage};
pub use self::os::{PRIO_PROCESS};
pub use self::os::{PRIO_PGRP};
pub use self::os::{PRIO_USER};
pub use self::os::{RLIM_INFINITY};
pub use self::os::{RLIM_SAVED_MAX};
pub use self::os::{RLIM_SAVED_CUR};
pub use self::os::{RUSAGE_SELF};
pub use self::os::{RUSAGE_CHILDREN};
pub use self::os::{RLIMIT_CORE};
pub use self::os::{RLIMIT_CPU};
pub use self::os::{RLIMIT_DATA};
pub use self::os::{RLIMIT_FSIZE};
pub use self::os::{RLIMIT_NOFILE};
pub use self::os::{RLIMIT_STACK};
pub use self::os::{RLIMIT_AS};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn getpriority(which: ::int_t, who: ::sys::types::id_t) -> ::int_t {
    extern { fn getpriority(which: ::int_t, who: ::sys::types::id_t) -> ::int_t; }
    unsafe { getpriority(which, who) }
}

pub fn getrlimit(resource: ::int_t, rlimits: &mut rlimit) -> ::int_t {
    extern { fn getrlimit(resource: ::int_t, rlimits: *mut rlimit) -> ::int_t; }
    unsafe { getrlimit(resource, rlimits as *mut _) }
}

pub fn getrusage(who: ::int_t, usage: &mut rusage) -> ::int_t {
    extern { fn getrusage(who: ::int_t, usage: *mut rusage) -> ::int_t; }
    unsafe { getrusage(who, usage as *mut _) }
}

pub fn setrlimit(resource: ::int_t, rlimits: &rlimit) -> ::int_t {
    extern { fn setrlimit(resource: ::int_t, rlimits: *const rlimit) -> ::int_t; }
    unsafe { setrlimit(resource, rlimits as *const _) }
}

pub fn setpriority(which: ::int_t, who: ::sys::types::id_t, prio: ::int_t) -> ::int_t {
    extern { fn setpriority(which: ::int_t, who: ::sys::types::id_t,
                            prio: ::int_t) -> ::int_t; }
    unsafe { setpriority(which, who, prio) }
}
