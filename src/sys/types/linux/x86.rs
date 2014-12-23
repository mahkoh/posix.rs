pub type ino_t = ::ulong_t;
pub type dev_t = ::ulonglong_t;
pub type gid_t = ::uint_t;
pub type mode_t = ::uint_t;
pub type nlink_t = ::uint_t;
pub type uid_t = ::uint_t;
pub type off_t = ::long_t;
pub type pid_t = ::int_t;
pub type id_t = ::uint_t;
pub type key_t = ::int_t;
pub type clock_t = ::long_t;
pub type time_t = ::long_t;
pub type clockid_t = ::int_t;
pub type timer_t = *mut ::void_t;
pub type suseconds_t = ::long_t;
pub type blksize_t = ::long_t;
pub type blkcnt_t = ::long_t;
pub type fsblkcnt_t = ::ulong_t;
pub type fsfilcnt_t = ::ulong_t;
pub type pthread_t = ::ulong_t;
#[repr(C)]
pub struct pthread_attr_t {
    __size: [u8, ..36],
    __align: [u32, ..1],
}
new!(pthread_attr_t);
#[repr(C)]
pub struct pthread_mutex_t {
    __data: [u32, ..6],
    __size: [u8, ..24],
    __align: [u32, ..1],
}
new!(pthread_mutex_t);
#[repr(C)]
pub struct pthread_mutexattr_t {
    __size: [u8, ..4],
    __align: [u32, ..1],
}
new!(pthread_mutexattr_t);
#[repr(C)]
pub struct pthread_cond_t {
    __data: [u32, ..11],
    __size: [u8, ..48],
    __align: [u32, ..2],
}
new!(pthread_cond_t);
#[repr(C)]
pub struct pthread_condattr_t {
    __size: [u8, ..4],
    __align: [u32, ..1],
}
new!(pthread_condattr_t);
pub type pthread_key_t = ::uint_t;
pub type pthread_once_t = ::int_t;
#[repr(C)]
pub struct pthread_rwlock_t {
    __data: [u32, ..8],
    __size: [u8, ..32],
    __align: [u32, ..1],
}
new!(pthread_rwlock_t);
#[repr(C)]
pub struct pthread_rwlockattr_t {
    __size: [u8, ..8],
    __align: [u32, ..1],
}
new!(pthread_rwlockattr_t);
pub type pthread_spinlock_t = ::int_t;
#[repr(C)]
pub struct pthread_barrier_t {
    __size: [u8, ..20],
    __align: [u32, ..1],
}
new!(pthread_barrier_t);
#[repr(C)]
pub struct pthread_barrierattr_t {
    __size: [u8, ..4],
    __align: [u32, ..1],
}
new!(pthread_barrierattr_t);
