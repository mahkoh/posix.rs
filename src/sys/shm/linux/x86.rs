pub type shmatt_t = ::ulong_t;
#[repr(C)]
pub struct shmid_ds {
    pub shm_perm: ::sys::ipc::ipc_perm,
    pub shm_segsz: ::size_t,
    pub shm_atime: ::sys::types::time_t,
    __glibc_reserved1: [u32, ..1],
    pub shm_dtime: ::sys::types::time_t,
    __glibc_reserved2: [u32, ..1],
    pub shm_ctime: ::sys::types::time_t,
    __glibc_reserved3: [u32, ..1],
    pub shm_cpid: ::sys::types::pid_t,
    pub shm_lpid: ::sys::types::pid_t,
    pub shm_nattch: shmatt_t,
    __glibc_reserved4: [u32, ..1],
    __glibc_reserved5: [u32, ..1],
}
new!(shmid_ds)
pub static SHM_RDONLY: ::int_t = 0o10000;
pub static SHM_RND: ::int_t = 0o20000;

pub fn SHMLBA() -> ::int_t {
    extern { fn __getpagesize() -> ::int_t; }
    unsafe { __getpagesize() }
}
