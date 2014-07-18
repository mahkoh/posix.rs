pub use self::os::{sem_t};
pub use self::os::{SEM_FAILED};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn sem_close(sem: *mut sem_t) -> ::int_t {
    extern { fn sem_close(sem: *mut sem_t) -> ::int_t; }
    unsafe { sem_close(sem) }
}

pub fn sem_destroy(sem: *mut sem_t) -> ::int_t {
    extern { fn sem_destroy(sem: *mut sem_t) -> ::int_t; }
    unsafe { sem_destroy(sem) }
}

pub fn sem_getvalue(sem: *mut sem_t, sval: &mut ::int_t) -> ::int_t {
    extern { fn sem_getvalue(sem: *mut sem_t, sval: *mut ::int_t) -> ::int_t; }
    unsafe { sem_getvalue(sem, sval as *mut _) }
}

pub fn sem_init(sem: *mut sem_t, pshared: ::int_t, value: ::uint_t) -> ::int_t {
    extern { fn sem_init(sem: *mut sem_t, pshared: ::int_t, value: ::uint_t) -> ::int_t; }
    unsafe { sem_init(sem, pshared, value) }
}

pub fn sem_open<T: ::NTStr>(name: &T, oflag: ::int_t, mode: ::sys::types::mode_t,
                            value: ::uint_t) -> *mut sem_t {
    extern { fn sem_open(name: *const ::char_t, oflag: ::int_t, ...) -> *mut sem_t; }
    unsafe { sem_open(name.as_ptr(), oflag, mode, value) }
}

pub fn sem_post(sem: *mut sem_t) -> ::int_t {
    extern { fn sem_post(sem: *mut sem_t) -> ::int_t; }
    unsafe { sem_post(sem) }
}

pub fn sem_timedwait(sem: *mut sem_t, abstime: &::time::timespec) -> ::int_t {
    extern { fn sem_timedwait(sem: *mut sem_t,
                              abstime: *const ::time::timespec) -> ::int_t; }
    unsafe { sem_timedwait(sem, abstime as *const _) }
}

pub fn sem_trywait(sem: *mut sem_t) -> ::int_t {
    extern { fn sem_trywait(sem: *mut sem_t) -> ::int_t; }
    unsafe { sem_trywait(sem) }
}

pub fn sem_unlink<T: ::NTStr>(name: &T) -> ::int_t {
    extern { fn sem_unlink(name: *const ::char_t) -> ::int_t; }
    unsafe { sem_unlink(name.as_ptr()) }
}

pub fn sem_wait(sem: *mut sem_t) -> ::int_t {
    extern { fn sem_wait(sem: *mut sem_t) -> ::int_t; }
    unsafe { sem_wait(sem) }
}
