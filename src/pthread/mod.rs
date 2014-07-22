pub use self::os::{PTHREAD_BARRIER_SERIAL_THREAD};
pub use self::os::{PTHREAD_CANCEL_ASYNCHRONOUS};
pub use self::os::{PTHREAD_CANCEL_ENABLE};
pub use self::os::{PTHREAD_CANCEL_DEFERRED};
pub use self::os::{PTHREAD_CANCEL_DISABLE};
pub use self::os::{PTHREAD_CREATE_DETACHED};
pub use self::os::{PTHREAD_CREATE_JOINABLE};
pub use self::os::{PTHREAD_EXPLICIT_SCHED};
pub use self::os::{PTHREAD_INHERIT_SCHED};
pub use self::os::{PTHREAD_MUTEX_DEFAULT};
pub use self::os::{PTHREAD_MUTEX_ERRORCHECK};
pub use self::os::{PTHREAD_MUTEX_NORMAL};
pub use self::os::{PTHREAD_MUTEX_RECURSIVE};
pub use self::os::{PTHREAD_MUTEX_ROBUST};
pub use self::os::{PTHREAD_MUTEX_STALLED};
pub use self::os::{PTHREAD_ONCE_INIT};
pub use self::os::{PTHREAD_PRIO_INHERIT};
pub use self::os::{PTHREAD_PRIO_NONE};
pub use self::os::{PTHREAD_PRIO_PROTECT};
pub use self::os::{PTHREAD_PROCESS_SHARED};
pub use self::os::{PTHREAD_PROCESS_PRIVATE};
pub use self::os::{PTHREAD_SCOPE_PROCESS};
pub use self::os::{PTHREAD_SCOPE_SYSTEM};
pub use self::os::{PTHREAD_CANCELED};
pub use self::os::{PTHREAD_COND_INITIALIZER};
pub use self::os::{PTHREAD_MUTEX_INITIALIZER};
pub use self::os::{PTHREAD_RWLOCK_INITIALIZER};

use attr        = sys::types::pthread_attr_t;
use barrierattr = sys::types::pthread_barrierattr_t;
use barrier     = sys::types::pthread_barrier_t;
use condattr    = sys::types::pthread_condattr_t;
use cond        = sys::types::pthread_cond_t;
use key         = sys::types::pthread_key_t;
use mutexattr   = sys::types::pthread_mutexattr_t;
use mutex       = sys::types::pthread_mutex_t;
use _once       = sys::types::pthread_once_t;
use rwlockattr  = sys::types::pthread_rwlockattr_t;
use rwlock      = sys::types::pthread_rwlock_t;
use spinlock    = sys::types::pthread_spinlock_t;
use thread      = sys::types::pthread_t;

use {uint_t, int_t, void_t, size_t};
use sys::types::{clockid_t};
use sched::{sched_param};
use time::{timespec};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn pthread_atfork(prepare: Option<extern fn()>, parent: Option<extern fn()>,
                      child: Option<extern fn()>) -> int_t {
    extern { fn pthread_atfork(prepare: Option<extern fn()>, parent: Option<extern fn()>,
                               child: Option<extern fn()>) -> int_t; }
    unsafe { pthread_atfork(prepare, parent, child) }
}

pub fn pthread_attr_destroy(attr: &mut attr) -> int_t {
    extern { fn pthread_attr_destroy(attr: *mut attr) -> int_t; }
    unsafe { pthread_attr_destroy(attr) }
}

pub fn pthread_attr_getdetachstate(attr: &attr, detachstate: &mut int_t) -> int_t {
    extern { fn pthread_attr_getdetachstate(attr: *const attr,
                                            detachstate: *mut int_t) -> int_t; }
    unsafe { pthread_attr_getdetachstate(attr as *const _, detachstate as *mut _) }
}

pub fn pthread_attr_getguardsize(attr: &attr, guardsize: &mut size_t) -> int_t {
    extern { fn pthread_attr_getguardsize(attr: *const attr,
                                          guardsize: *mut size_t) -> int_t; }
    unsafe { pthread_attr_getguardsize(attr as *const _, guardsize as *mut _) }
}

pub fn pthread_attr_getinheritsched(attr: &attr, inherit: &mut int_t) -> int_t {
    extern { fn pthread_attr_getinheritsched(attr: *const attr,
                                             inherit: *mut int_t) -> int_t; }
    unsafe { pthread_attr_getinheritsched(attr as *const _, inherit as *mut _) }
}

pub fn pthread_attr_getschedparam(attr: &attr, param: &mut sched_param) -> int_t {
    extern { fn pthread_attr_getschedparam(attr: *const attr,
                                           param: *mut sched_param) -> int_t; }
    unsafe { pthread_attr_getschedparam(attr as *const _, param as *mut _) }
}

pub fn pthread_attr_getschedpolicy(attr: &attr, policy: &mut int_t) -> int_t {
    extern { fn pthread_attr_getschedpolicy(attr: *const attr,
                                            policy: *mut int_t) -> int_t; }
    unsafe { pthread_attr_getschedpolicy(attr as *const _, policy as *mut _) }
}

pub fn pthread_attr_getscope(attr: &attr, scope: &mut int_t) -> int_t {
    extern { fn pthread_attr_getscope(attr: *const attr,
                                      scope: *mut int_t) -> int_t; }
    unsafe { pthread_attr_getscope(attr as *const _, scope as *mut _) }
}

pub fn pthread_attr_getstacksize(attr: &attr, stacksize: &mut size_t) -> int_t {
    extern { fn pthread_attr_getstacksize(attr: *const attr,
                                          stacksize: *mut size_t) -> int_t; }
    unsafe { pthread_attr_getstacksize(attr as *const _, stacksize as *mut _) }
}

pub fn pthread_attr_init(attr: &mut attr) -> int_t {
    extern { fn pthread_attr_init(attr: *mut attr) -> int_t; }
    unsafe { pthread_attr_init(attr as *mut _) }
}

pub fn pthread_attr_setdetachstate(attr: &mut attr, detachstate: int_t) -> int_t {
    extern { fn pthread_attr_setdetachstate(attr: *mut attr,
                                            detachstate: int_t) -> int_t; }
    unsafe { pthread_attr_setdetachstate(attr as *mut _, detachstate) }
}

pub fn pthread_attr_setguardsize(attr: &mut attr, guardsize: size_t) -> int_t {
    extern { fn pthread_attr_setguardsize(attr: *mut attr,
                                          guardsize: size_t) -> int_t; }
    unsafe { pthread_attr_setguardsize(attr as *mut _, guardsize) }
}

pub fn pthread_attr_setinheritsched(attr: &mut attr, inherit: int_t) -> int_t {
    extern { fn pthread_attr_setinheritsched(attr: *mut attr, inherit: int_t) -> int_t; }
    unsafe { pthread_attr_setinheritsched(attr as *mut _, inherit) }
}

pub fn pthread_attr_setschedparam(attr: &mut attr, param: &sched_param) -> int_t {
    extern { fn pthread_attr_setschedparam(attr: *mut attr,
                                           param: *const sched_param) -> int_t; }
    unsafe { pthread_attr_setschedparam(attr as *mut _, param as *const _) }
}

pub fn pthread_attr_setschedpolicy(attr: &mut attr, policy: int_t) -> int_t {
    extern { fn pthread_attr_setschedpolicy(attr: *mut attr,
                                            policy: int_t) -> int_t; }
    unsafe { pthread_attr_setschedpolicy(attr as *mut _, policy) }
}

pub fn pthread_attr_setscope(attr: &mut attr, scope: int_t) -> int_t {
    extern { fn pthread_attr_setscope(attr: *mut attr, scope: int_t) -> int_t; }
    unsafe { pthread_attr_setscope(attr as *mut _, scope) }
}

pub fn pthread_attr_setstacksize(attr: &mut attr, stacksize: size_t) -> int_t {
    extern { fn pthread_attr_setstacksize(attr: *mut attr,
                                          stacksize: size_t) -> int_t; }
    unsafe { pthread_attr_setstacksize(attr as *mut _, stacksize) }
}

pub fn pthread_barrier_destroy(barrier: &mut barrier) -> int_t {
    extern { fn pthread_barrier_destroy(barrier: *mut barrier) -> int_t; }
    unsafe { pthread_barrier_destroy(barrier as *mut _) }
}


pub fn pthread_barrier_init(barrier: &mut barrier, attr: &barrierattr,
                                count: uint_t) -> int_t {
    extern { fn pthread_barrier_init(barrier: *mut barrier, attr: *const barrierattr,
                                     count: uint_t) -> int_t; }
    unsafe { pthread_barrier_init(barrier as *mut _, attr as *const _, count) }
}

pub fn pthread_barrier_wait(barrier: &mut barrier) -> int_t {
    extern { fn pthread_barrier_wait(barrier: *mut barrier) -> int_t; }
    unsafe { pthread_barrier_wait(barrier as *mut _) }
}

pub fn pthread_barrierattr_destroy(attr: &mut barrierattr) -> int_t {
    extern { fn pthread_barrierattr_destroy(attr: *mut barrierattr) -> int_t; }
    unsafe { pthread_barrierattr_destroy(attr as *mut _) }
}


pub fn pthread_barrierattr_getpshared(attr: &barrierattr,
                                      pshared: &mut int_t) -> int_t {
    extern { fn pthread_barrierattr_getpshared(attr: *const barrierattr,
                                               pshared: *mut int_t) -> int_t; }
    unsafe { pthread_barrierattr_getpshared(attr as *const _, pshared as *mut _) }
}

pub fn pthread_barrierattr_init(attr: &mut barrierattr) -> int_t {
    extern { fn pthread_barrierattr_init(attr: *mut barrierattr) -> int_t; }
    unsafe { pthread_barrierattr_init(attr as *mut _) }
}

pub fn pthread_barrierattr_setpshared(attr: &mut barrierattr,
                                      pshared: int_t) -> int_t {
    extern { fn pthread_barrierattr_setpshared(attr: *mut barrierattr,
                                               pshared: int_t) -> int_t; }
    unsafe { pthread_barrierattr_setpshared(attr as *mut _, pshared) }
}

pub fn pthread_cancel(th: thread) -> int_t {
    extern { fn pthread_cancel(th: thread) -> int_t; }
    unsafe { pthread_cancel(th) }
}

pub fn pthread_cond_broadcast(cond: &mut cond) -> int_t {
    extern { fn pthread_cond_broadcast(cond: *mut cond) -> int_t; }
    unsafe { pthread_cond_broadcast(cond as *mut _) }
}

pub fn pthread_cond_destroy(cond: &mut cond) -> int_t {
    extern { fn pthread_cond_destroy(cond: *mut cond) -> int_t; }
    unsafe { pthread_cond_destroy(cond) }
}

pub fn pthread_cond_init(cond: &mut cond, cond_attr: &condattr) -> int_t {
    extern { fn pthread_cond_init(cond: *mut cond,
                                  cond_attr: *const condattr) -> int_t; }
    unsafe { pthread_cond_init(cond as *mut _, cond_attr as *const _) }
}

pub fn pthread_cond_signal(cond: &mut cond) -> int_t {
    extern { fn pthread_cond_signal(cond: *mut cond) -> int_t; }
    unsafe { pthread_cond_signal(cond as *mut _) }
}

pub fn pthread_cond_timedwait(cond: &mut cond, mutex: &mut mutex,
                                  abstime: &timespec) -> int_t {
    extern { fn pthread_cond_timedwait(cond: *mut cond, mutex: *mut mutex,
                                       abstime: *const timespec) -> int_t; }
    unsafe { pthread_cond_timedwait(cond as *mut _, mutex as *mut _,
                                    abstime as  *const _) }
}

pub fn pthread_cond_wait(cond: &mut cond, mutex: &mut mutex) -> int_t {
    extern { fn pthread_cond_wait(cond: *mut cond, mutex: *mut mutex) -> int_t; }
    unsafe { pthread_cond_wait(cond as *mut _, mutex as *mut _) }
}

pub fn pthread_condattr_destroy(attr: &mut condattr) -> int_t {
    extern { fn pthread_condattr_destroy(attr: *mut condattr) -> int_t; }
    unsafe { pthread_condattr_destroy(attr as *mut _) }
}

pub fn pthread_condattr_getclock(attr: &condattr, clock_id: &mut clockid_t) -> int_t {
    extern { fn pthread_condattr_getclock(attr: *const condattr,
                                          clock_id: *mut clockid_t) -> int_t; }
    unsafe { pthread_condattr_getclock(attr as *const _, clock_id as *mut _) }
}


pub fn pthread_condattr_getpshared(attr: &condattr, pshared: &mut int_t) -> int_t {
    extern { fn pthread_condattr_getpshared(attr: *const condattr,
                                            pshared: *mut int_t) -> int_t; }
    unsafe { pthread_condattr_getpshared(attr as *const _, pshared as *mut _) }
}

pub fn pthread_condattr_init(attr: &mut condattr) -> int_t {
    extern { fn pthread_condattr_init(attr: *mut condattr) -> int_t; }
    unsafe { pthread_condattr_init(attr as *mut _) }
}

pub fn pthread_condattr_setclock(attr: &mut condattr, clock_id: clockid_t) -> int_t {
    extern { fn pthread_condattr_setclock(attr: *mut condattr, clock_id: clockid_t) -> int_t; }
    unsafe { pthread_condattr_setclock(attr as *mut _, clock_id) }
}

pub fn pthread_condattr_setpshared(attr: &mut condattr, pshared: int_t) -> int_t {
    extern { fn pthread_condattr_setpshared(attr: *mut condattr, pshared: int_t) -> int_t; }
    unsafe { pthread_condattr_setpshared(attr as *mut _, pshared) }
}

pub fn pthread_detach(th: thread) -> int_t {
    extern { fn pthread_detach(th: thread) -> int_t; }
    unsafe { pthread_detach(th) }
}

pub fn pthread_equal(thread1: thread, thread2: thread) -> int_t {
    extern { fn pthread_equal(thread1: thread, thread2: thread) -> int_t; }
    unsafe { pthread_equal(thread1, thread2) }
}

pub fn pthread_getcpuclockid(thread_id: thread, clock_id: &mut clockid_t) -> int_t {
    extern { fn pthread_getcpuclockid(thread_id: thread, clock_id: *mut clockid_t) -> int_t; }
    unsafe { pthread_getcpuclockid(thread_id, clock_id as *mut _) }
}

pub fn pthread_getschedparam(target_thread: thread, policy: &mut int_t,
                             param: &mut sched_param) -> int_t {
    extern { fn pthread_getschedparam(target_thread: thread, policy: *mut int_t,
                                      param: *mut sched_param) -> int_t; }
    unsafe { pthread_getschedparam(target_thread, policy as *mut _, param as *mut _) }
}

pub fn pthread_key_create(key: &mut key,
                          destr_function: extern fn (arg1: *mut void_t)) -> int_t {
    extern { fn pthread_key_create(key: *mut key,
                                   destr_function: extern fn (arg1: *mut void_t)) -> int_t; }
    unsafe { pthread_key_create(key as *mut _, destr_function) }
}

pub fn pthread_key_delete(key: key) -> int_t {
    extern { fn pthread_key_delete(key: key) -> int_t; }
    unsafe { pthread_key_delete(key) }
}

pub fn pthread_mutex_consistent(mutex: &mut mutex) -> int_t {
    extern { fn pthread_mutex_consistent(mutex: *mut mutex) -> int_t; }
    unsafe { pthread_mutex_consistent(mutex as *mut _) }
}

pub fn pthread_mutex_destroy(mutex: &mut mutex) -> int_t {
    extern { fn pthread_mutex_destroy(mutex: *mut mutex) -> int_t; }
    unsafe { pthread_mutex_destroy(mutex as *mut _) }
}

pub fn pthread_mutex_getprioceiling(mutex: &mutex, prioceiling: &mut int_t) -> int_t {
    extern { fn pthread_mutex_getprioceiling(mutex: *const mutex,
                                             prioceiling: *mut int_t) -> int_t; }
    unsafe { pthread_mutex_getprioceiling(mutex as *const _, prioceiling as *mut _) }
}

pub fn pthread_mutex_init(mutex: &mut mutex, mutexattr: &mutexattr) -> int_t {
    extern { fn pthread_mutex_init(mutex: *mut mutex,
                                   mutexattr: *const mutexattr) -> int_t; }
    unsafe { pthread_mutex_init(mutex as *mut _, mutexattr as *const _) }
}

pub fn pthread_mutex_lock(mutex: &mut mutex) -> int_t {
    extern { fn pthread_mutex_lock(mutex: *mut mutex) -> int_t; }
    unsafe { pthread_mutex_lock(mutex as *mut mutex) }
}

pub fn pthread_mutex_setprioceiling(mutex: &mut mutex, prioceiling: int_t,
                                    old_ceiling: &mut int_t) -> int_t {
    extern { fn pthread_mutex_setprioceiling(mutex: *mut mutex, prioceiling: int_t,
                                             old_ceiling: *mut int_t) -> int_t; }
    unsafe { pthread_mutex_setprioceiling(mutex as *mut _, prioceiling,
                                             old_ceiling as *mut _) }
}

pub fn muteximedlock(mutex: &mut mutex, abstime: &timespec) -> int_t {
    extern { fn muteximedlock(mutex: *mut mutex, abstime: *const timespec) -> int_t; }
    unsafe { muteximedlock(mutex as *mut _, abstime as *const _) }
}

pub fn mutexrylock(mutex: &mut mutex) -> int_t {
    extern { fn mutexrylock(mutex: *mut mutex) -> int_t; }
    unsafe { mutexrylock(mutex as *mut _) }
}

pub fn pthread_mutex_unlock(mutex: &mut mutex) -> int_t {
    extern { fn pthread_mutex_unlock(mutex: *mut mutex) -> int_t; }
    unsafe { pthread_mutex_unlock(mutex as *mut _) }
}

pub fn pthread_mutexattr_destroy(attr: &mut mutexattr) -> int_t {
    extern { fn pthread_mutexattr_destroy(attr: *mut mutexattr) -> int_t; }
    unsafe { pthread_mutexattr_destroy(attr as *mut _) }
}

pub fn pthread_mutexattr_getprioceiling(attr: &mutexattr,
                                        prioceiling: *mut int_t) -> int_t {
    extern { fn pthread_mutexattr_getprioceiling(attr: *const mutexattr,
                                                 prioceiling: *mut int_t) -> int_t; }
    unsafe { pthread_mutexattr_getprioceiling(attr as *const _, prioceiling as *mut _) }
}

pub fn pthread_mutexattr_getprotocol(attr: &mutexattr,
                                     protocol: &mut int_t) -> int_t {
    extern { fn pthread_mutexattr_getprotocol(attr: *const mutexattr,
                                              protocol: *mut int_t) -> int_t; }
    unsafe { pthread_mutexattr_getprotocol(attr as *const _, protocol as *mut _) }
}

pub fn pthread_mutexattr_getpshared(attr: &mutexattr, pshared: &mut int_t) -> int_t {
    extern { fn pthread_mutexattr_getpshared(attr: *const mutexattr,
                                             pshared: *mut int_t) -> int_t; }
    unsafe { pthread_mutexattr_getpshared(attr as *const _, pshared as *mut _) }
}

pub fn pthread_mutexattr_getrobust(attr: &mutexattr,
                                   robustness: &mut int_t) -> int_t {
    extern { fn pthread_mutexattr_getrobust(attr: *const mutexattr,
                                            robustness: *mut int_t) -> int_t; }
    unsafe { pthread_mutexattr_getrobust(attr as *const _, robustness as *mut _) }
}

pub fn pthread_mutexattr_gettype(attr: &mutexattr, kind: &mut int_t) -> int_t {
    extern { fn pthread_mutexattr_gettype(attr: *const mutexattr,
                                          kind: *mut int_t) -> int_t; }
    unsafe { pthread_mutexattr_gettype(attr as *const _, kind as *mut _) }
}

pub fn pthread_mutexattr_init(attr: &mut mutexattr) -> int_t {
    extern { fn pthread_mutexattr_init(attr: *mut mutexattr) -> int_t; }
    unsafe { pthread_mutexattr_init(attr as *mut _) }
}

pub fn pthread_mutexattr_setprioceiling(attr: &mut mutexattr,
                                            prioceiling: int_t) -> int_t {
    extern { fn pthread_mutexattr_setprioceiling(attr: *mut mutexattr,
                                                 prioceiling: int_t) -> int_t; }
    unsafe { pthread_mutexattr_setprioceiling(attr as *mut _, prioceiling) }
}

pub fn pthread_mutexattr_setprotocol(attr: &mut mutexattr, protocol: int_t) -> int_t {
    extern { fn pthread_mutexattr_setprotocol(attr: *mut mutexattr,
                                              protocol: int_t) -> int_t; }
    unsafe { pthread_mutexattr_setprotocol(attr as *mut _, protocol) }
}

pub fn pthread_mutexattr_setpshared(attr: &mut mutexattr, pshared: int_t) -> int_t {
    extern { fn pthread_mutexattr_setpshared(attr: *mut mutexattr,
                                             pshared: int_t) -> int_t; }
    unsafe { pthread_mutexattr_setpshared(attr as *mut _, pshared) }
}

pub fn pthread_mutexattr_setrobust(attr: &mut mutexattr, robustness: int_t) -> int_t {
    extern { fn pthread_mutexattr_setrobust(attr: *mut mutexattr,
                                            robustness: int_t) -> int_t; }
    unsafe { pthread_mutexattr_setrobust(attr as *mut _, robustness) }
}

pub fn pthread_mutexattr_settype(attr: &mut mutexattr, kind: int_t) -> int_t {
    extern { fn pthread_mutexattr_settype(attr: *mut mutexattr, kind: int_t) -> int_t; }
    unsafe { pthread_mutexattr_settype(attr as *mut _, kind) }
}

pub fn pthread_once(once_control: &mut _once, init_routine: extern fn()) -> int_t {
    extern { fn pthread_once(once_control: *mut _once, init_routine: extern fn()) -> int_t; }
    unsafe { pthread_once(once_control as *mut _, init_routine) }
}

pub fn pthread_rwlock_destroy(rwlock: &mut rwlock) -> int_t {
    extern { fn pthread_rwlock_destroy(rwlock: *mut rwlock) -> int_t; }
    unsafe { pthread_rwlock_destroy(rwlock as *mut _) }
}

pub fn pthread_rwlock_init(rwlock: &mut rwlock, attr: &rwlockattr) -> int_t {
    extern { fn pthread_rwlock_init(rwlock: *mut rwlock, attr: *const rwlockattr) -> int_t; }
    unsafe { pthread_rwlock_init(rwlock as *mut _, attr as *const _) }
}

pub fn pthread_rwlock_rdlock(rwlock: &mut rwlock) -> int_t {
    extern { fn pthread_rwlock_rdlock(rwlock: *mut rwlock) -> int_t; }
    unsafe { pthread_rwlock_rdlock(rwlock as *mut _) }
}

pub fn rwlockimedrdlock(rwlock: &mut rwlock, abstime: &timespec) -> int_t {
    extern { fn rwlockimedrdlock(rwlock: *mut rwlock, abstime: *const timespec) -> int_t; }
    unsafe { rwlockimedrdlock(rwlock as *mut _, abstime as *const _) }
}

pub fn rwlockimedwrlock(rwlock: *mut rwlock, abstime: *const timespec) -> int_t {
    extern { fn rwlockimedwrlock(rwlock: *mut rwlock, abstime: *const timespec) -> int_t; }
    unsafe { rwlockimedwrlock(rwlock as *mut _, abstime as *const _) }
}

pub fn rwlockryrdlock(rwlock: &mut rwlock) -> int_t {
    extern { fn rwlockryrdlock(rwlock: *mut rwlock) -> int_t; }
    unsafe { rwlockryrdlock(rwlock as *mut _) }
}

pub fn rwlockrywrlock(rwlock: &mut rwlock) -> int_t {
    extern { fn rwlockrywrlock(rwlock: *mut rwlock) -> int_t; }
    unsafe { rwlockrywrlock(rwlock as *mut _) }
}

pub fn pthread_rwlock_unlock(rwlock: &mut rwlock) -> int_t {
    extern { fn pthread_rwlock_unlock(rwlock: *mut rwlock) -> int_t; }
    unsafe { pthread_rwlock_unlock(rwlock as *mut _) }
}

pub fn pthread_rwlock_wrlock(rwlock: &mut rwlock) -> int_t {
    extern { fn pthread_rwlock_wrlock(rwlock: *mut rwlock) -> int_t; }
    unsafe { pthread_rwlock_wrlock(rwlock as *mut _) }
}

pub fn pthread_rwlockattr_destroy(attr: &mut rwlockattr) -> int_t {
    extern { fn pthread_rwlockattr_destroy(attr: *mut rwlockattr) -> int_t; }
    unsafe { pthread_rwlockattr_destroy(attr as *mut _) }
}

pub fn pthread_rwlockattr_getpshared(attr: &rwlockattr,
                                     pshared: &mut int_t) -> int_t {
    extern { fn pthread_rwlockattr_getpshared(attr: *const rwlockattr,
                                              pshared: *mut int_t) -> int_t; }
    unsafe { pthread_rwlockattr_getpshared(attr as *const _, pshared as *mut _) }
}

pub fn pthread_rwlockattr_init(attr: &mut rwlockattr) -> int_t {
    extern { fn pthread_rwlockattr_init(attr: *mut rwlockattr) -> int_t; }
    unsafe { pthread_rwlockattr_init(attr as *mut _) }
}

pub fn pthread_rwlockattr_setpshared(attr: &mut rwlockattr, pshared: int_t) -> int_t {
    extern { fn pthread_rwlockattr_setpshared(attr: *mut rwlockattr,
                                              pshared: int_t) -> int_t; }
    unsafe { pthread_rwlockattr_setpshared(attr as *mut _, pshared) }
}

pub fn pthread_self() -> thread {
    extern { fn pthread_self() -> thread; }
    unsafe { pthread_self() }
}

pub fn pthread_setcancelstate(state: int_t, oldstate: &mut int_t) -> int_t {
    extern { fn pthread_setcancelstate(state: int_t, oldstate: *mut int_t) -> int_t; }
    unsafe { pthread_setcancelstate(state, oldstate as *mut _) }
}

pub fn pthread_setcanceltype(ty: int_t, oldtype: &mut int_t) -> int_t {
    extern { fn pthread_setcanceltype(ty: int_t, oldtype: *mut int_t) -> int_t; }
    unsafe { pthread_setcanceltype(ty, oldtype as *mut _) }
}

pub fn pthread_setschedparam(target_thread: thread, policy: int_t,
                             param: &sched_param) -> int_t {
    extern { fn pthread_setschedparam(target_thread: thread, policy: int_t,
                                      param: *const sched_param) -> int_t; }
    unsafe { pthread_setschedparam(target_thread, policy, param as *const _) }
}

pub fn pthread_setschedprio(target_thread: thread, prio: int_t) -> int_t {
    extern { fn pthread_setschedprio(target_thread: thread, prio: int_t) -> int_t; }
    unsafe { pthread_setschedprio(target_thread, prio) }
}

pub fn pthread_spin_destroy(lock: &mut spinlock) -> int_t {
    extern { fn pthread_spin_destroy(lock: *mut spinlock) -> int_t; }
    unsafe { pthread_spin_destroy(lock as *mut _) }
}

pub fn pthread_spin_init(lock: &mut spinlock, pshared: int_t) -> int_t {
    extern { fn pthread_spin_init(lock: *mut spinlock, pshared: int_t) -> int_t; }
    unsafe { pthread_spin_init(lock as *mut _, pshared) }
}

pub fn pthread_spin_lock(lock: &mut spinlock) -> int_t {
    extern { fn pthread_spin_lock(lock: *mut spinlock) -> int_t; }
    unsafe { pthread_spin_lock(lock as *mut _) }
}

pub fn pthread_spin_trylock(lock: &mut spinlock) -> int_t {
    extern { fn pthread_spin_trylock(lock: *mut spinlock) -> int_t; }
    unsafe { pthread_spin_trylock(lock as *mut _) }
}

pub fn pthread_spin_unlock(lock: &mut spinlock) -> int_t {
    extern { fn pthread_spin_unlock(lock: *mut spinlock) -> int_t; }
    unsafe { pthread_spin_unlock(lock as *mut _) }
}

pub fn threadestcancel() {
    extern { fn threadestcancel(); }
    unsafe { threadestcancel() }
}

extern {
    pub fn pthread_attr_getstack(attr: *const attr, stackaddr: *mut *mut void_t,
                                 stacksize: *mut size_t) -> int_t;
    pub fn pthread_attr_setstack(attr: *mut attr, stackaddr: *mut void_t,
                                 stacksize: size_t) -> int_t;
    pub fn pthread_create(newthread: *mut thread, attr: *const attr,
                          start_routine: extern fn (arg1: *mut void_t) -> *mut void_t,
                          arg: *mut void_t) -> int_t;
    pub fn pthread_exit(retval: *mut void_t);
    pub fn pthread_getspecific(key: key) -> *mut void_t;
    pub fn pthread_join(th: thread, thread_return: *mut *mut void_t) -> int_t;
    pub fn pthread_setspecific(key: key, pointer: *const void_t) -> int_t;
}
