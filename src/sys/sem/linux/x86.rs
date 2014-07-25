#[repr(C)]
pub struct semid_ds {
    pub sem_perm: ::sys::ipc::ipc_perm,
    pub sem_otime: ::sys::types::time_t,
    __glibc_reserved1: [u32, ..1],
    pub sem_ctime: ::sys::types::time_t,
    __glibc_reserved2: [u32, ..1],
    pub sem_nsems: ::ulong_t,
    __glibc_reserved3: [u32, ..1],
    __glibc_reserved4: [u32, ..1],
}
new!(semid_ds)
#[repr(C)]
pub struct sembuf {
    pub sem_num: ::ushort_t,
    pub sem_op: ::short_t,
    pub sem_flg: ::short_t,
}
new!(sembuf)
pub static SEM_UNDO: ::int_t = 0x1000;
pub static GETNCNT: ::int_t = 14;
pub static GETPID: ::int_t = 11;
pub static GETVAL: ::int_t = 12;
pub static GETALL: ::int_t = 13;
pub static GETZCNT: ::int_t = 15;
pub static SETVAL: ::int_t = 16;
pub static SETALL: ::int_t = 17;
