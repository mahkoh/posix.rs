#[repr(C)]
pub struct ipc_perm {
    __key: [u32, ..1],
    pub uid: ::sys::types::uid_t,
    pub gid: ::sys::types::gid_t,
    pub cuid: ::sys::types::uid_t,
    pub cgid: ::sys::types::gid_t,
    pub mode: ::ushort_t,
    __pad1: [u16, ..1],
    __seq: [u16, ..1],
    __pad2: [u16, ..1],
    __glibc_reserved1: [u32, ..1],
    __glibc_reserved2: [u32, ..1],
}
new!(ipc_perm);
pub const IPC_CREAT: ::int_t = 0o1000;
pub const IPC_EXCL: ::int_t = 0o2000;
pub const IPC_NOWAIT: ::int_t = 0o4000;
pub const IPC_PRIVATE: ::int_t = 0;
pub const IPC_RMID: ::int_t = 0;
pub const IPC_SET: ::int_t = 1;
pub const IPC_STAT: ::int_t = 2;
