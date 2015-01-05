pub type shmatt_t = ::ulong_t;

#[repr(C)]
#[derive(Copy)]
pub struct shmid_ds {
    pub shm_perm: ::sys::ipc::ipc_perm,
    pub shm_segsz: ::size_t,
    pub shm_atime: ::sys::types::time_t,
    pub shm_dtime: ::sys::types::time_t,
    pub shm_ctime: ::sys::types::time_t,
    pub shm_cpid:  ::sys::types::pid_t,
    pub shm_lpid:  ::sys::types::pid_t,
    pub shm_nattch: shmatt_t,
    __glibc_reserved4: ::ulong_t,
    __glibc_reserved5: ::ulong_t,
}

new!(shmid_ds);

pub const SHM_RDONLY: ::int_t = 4096;
pub const SHM_RND:    ::int_t = 8192;

pub fn SHMLBA() -> ::int_t {
    extern { fn __getpagesize() -> ::int_t; }
    unsafe { __getpagesize() }
}
