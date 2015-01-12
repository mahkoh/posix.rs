pub type dev_t = u64;
pub type uid_t = u32;
pub type gid_t = u32;
pub type ino_t = u64;
pub type mode_t = u32;
pub type nlink_t = u64;
pub type off_t = i64;
pub type pid_t = i32;
pub type clock_t = i64;
pub type id_t = u32;
pub type time_t = i64;
pub type suseconds_t = i64;
pub type key_t = i32;
pub type clockid_t = i32;
pub type timer_t = *mut i8;
pub type blksize_t = i64;
pub type blkcnt_t = i64;
pub type fsblkcnt_t = u64;
pub type fsfilcnt_t = u64;
pub type pthread_t = u64;

#[repr(C)]
#[derive(Copy)]
pub struct pthread_attr_t {
    _data: [u64; 7us],
}

new!(pthread_attr_t);

#[repr(C)]
#[derive(Copy)]
pub struct pthread_mutex_t {
    _data: [u64; 5us],
}

new!(pthread_mutex_t);

#[repr(C)]
#[derive(Copy)]
pub struct pthread_mutexattr_t {
    _data: [u32; 1us],
}

new!(pthread_mutexattr_t);

#[repr(C)]
#[derive(Copy)]
pub struct pthread_cond_t {
    _data: [u64; 6us],
}

new!(pthread_cond_t);

#[repr(C)]
#[derive(Copy)]
pub struct pthread_condattr_t {
    _data: [u32; 1us],
}

new!(pthread_condattr_t);

pub type pthread_key_t = u32;
pub type pthread_once_t = i32;

#[repr(C)]
#[derive(Copy)]
pub struct pthread_rwlock_t {
    _data: [u64; 7us],
}

new!(pthread_rwlock_t);

#[repr(C)]
#[derive(Copy)]
pub struct pthread_rwlockattr_t {
    _data: [u64; 1us],
}

new!(pthread_rwlockattr_t);

pub type pthread_spinlock_t = i32;

#[repr(C)]
#[derive(Copy)]
pub struct pthread_barrier_t {
    _data: [u64; 4us],
}

new!(pthread_barrier_t);

#[repr(C)]
#[derive(Copy)]
pub struct pthread_barrierattr_t {
    _data: [u32; 1us],
}

new!(pthread_barrierattr_t);
