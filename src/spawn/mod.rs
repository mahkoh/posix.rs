pub use self::os::{posix_spawnattr_t};
pub use self::os::{posix_spawn_file_actions_t};
pub use self::os::{POSIX_SPAWN_RESETIDS};
pub use self::os::{POSIX_SPAWN_SETPGROUP};
pub use self::os::{POSIX_SPAWN_SETSCHEDPARAM};
pub use self::os::{POSIX_SPAWN_SETSCHEDULER};
pub use self::os::{POSIX_SPAWN_SETSIGDEF};
pub use self::os::{POSIX_SPAWN_SETSIGMASK};

use {char_t, int_t, short_t};
use sys::types::{pid_t, mode_t};
use sched::{sched_param};
use signal::{sigset_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

extern {
    pub fn posix_spawn(pid: *mut pid_t, path: *const char_t,
                       file_actions: *const posix_spawn_file_actions_t,
                       attrp: *const posix_spawnattr_t,
                       argv: *const *mut char_t,
                       envp: *const *mut char_t) -> int_t;
    pub fn posix_spawnp(pid: *mut pid_t, file: *const char_t,
                        file_actions: *const posix_spawn_file_actions_t,
                        attrp: *const posix_spawnattr_t,
                        argv: *const *mut char_t,
                        envp: *const *mut char_t) -> int_t;
    pub fn posix_spawnattr_init(attr: *mut posix_spawnattr_t) -> int_t;
    pub fn posix_spawnattr_destroy(attr: *mut posix_spawnattr_t) -> int_t;
    pub fn posix_spawnattr_getsigdefault(attr: *const posix_spawnattr_t,
                                         sigdefault: *mut sigset_t) ->
     int_t;
    pub fn posix_spawnattr_setsigdefault(attr: *mut posix_spawnattr_t,
                                         sigdefault: *const sigset_t) ->
     int_t;
    pub fn posix_spawnattr_getsigmask(attr: *const posix_spawnattr_t,
                                      sigmask: *mut sigset_t) -> int_t;
    pub fn posix_spawnattr_setsigmask(attr: *mut posix_spawnattr_t,
                                      sigmask: *const sigset_t) -> int_t;
    pub fn posix_spawnattr_getflags(attr: *const posix_spawnattr_t,
                                    flags: *mut short_t) -> int_t;
    pub fn posix_spawnattr_setflags(_attr: *mut posix_spawnattr_t,
                                    flags: short_t) -> int_t;
    pub fn posix_spawnattr_getpgroup(attr: *const posix_spawnattr_t,
                                     pgroup: *mut pid_t) -> int_t;
    pub fn posix_spawnattr_setpgroup(attr: *mut posix_spawnattr_t,
                                     pgroup: pid_t) -> int_t;
    pub fn posix_spawnattr_getschedpolicy(attr: *const posix_spawnattr_t,
                                          schedpolicy: *mut int_t) ->
     int_t;
    pub fn posix_spawnattr_setschedpolicy(attr: *mut posix_spawnattr_t,
                                          schedpolicy: int_t) -> int_t;
    pub fn posix_spawnattr_getschedparam(attr: *const posix_spawnattr_t,
                                         schedparam:
                                             *mut sched_param) ->
     int_t;
    pub fn posix_spawnattr_setschedparam(attr: *mut posix_spawnattr_t,
                                         schedparam:
                                             *const sched_param) ->
     int_t;
    pub fn posix_spawn_file_actions_init(file_actions:
                                             *mut posix_spawn_file_actions_t)
     -> int_t;
    pub fn posix_spawn_file_actions_destroy(file_actions:
                                                *mut posix_spawn_file_actions_t)
     -> int_t;
    pub fn posix_spawn_file_actions_addopen(file_actions:
                                                *mut posix_spawn_file_actions_t,
                                            fd: int_t,
                                            path: *const char_t,
                                            oflag: int_t, mode: mode_t)
     -> int_t;
    pub fn posix_spawn_file_actions_addclose(file_actions:
                                                 *mut posix_spawn_file_actions_t,
                                             fd: int_t) -> int_t;
    pub fn posix_spawn_file_actions_adddup2(file_actions:
                                                *mut posix_spawn_file_actions_t,
                                            fd: int_t, newfd: int_t)
     -> int_t;
}
