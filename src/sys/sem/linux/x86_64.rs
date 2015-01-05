#[repr(C)]
#[derive(Copy)]
pub struct semid_ds {
    pub sem_perm: ::sys::ipc::ipc_perm,
    pub sem_otime: ::sys::types::time_t,
    __glibc_reserved1: ::ulong_t,
    pub sem_ctime: ::sys::types::time_t,
    __glibc_reserved2: ::ulong_t,
    pub sem_nsems: ::ulong_t,
    __glibc_reserved3: ::ulong_t,
    __glibc_reserved4: ::ulong_t,
}

new!(semid_ds);

#[repr(C)]
#[derive(Copy)]
pub struct sembuf {
    pub sem_num: ::ushort_t,
    pub sem_op: ::short_t,
    pub sem_flg: ::short_t,
}

new!(sembuf);

pub const SEM_UNDO: ::int_t = 4096;
pub const GETNCNT:  ::int_t = 14;
pub const GETPID:   ::int_t = 11;
pub const GETVAL:   ::int_t = 12;
pub const GETALL:   ::int_t = 13;
pub const GETZCNT:  ::int_t = 15;
pub const SETVAL:   ::int_t = 16;
pub const SETALL:   ::int_t = 17;
