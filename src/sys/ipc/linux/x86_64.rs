#[repr(C)]
#[deriving(Copy)]
pub struct ipc_perm {
    __key: ::sys::types::key_t,
    pub uid: ::sys::types::uid_t,
    pub gid: ::sys::types::gid_t,
    pub cuid: ::sys::types::uid_t,
    pub cgid: ::sys::types::gid_t,
    pub mode: ::ushort_t,
    __pad1: ::ushort_t,
    __seq: ::ushort_t,
    __pad2: ::ushort_t,
    __glibc_reserved1: ::ulong_t,
    __glibc_reserved2: ::ulong_t,
}

new!(ipc_perm);

pub const IPC_CREAT:   ::int_t = 512;
pub const IPC_EXCL:    ::int_t = 1024;
pub const IPC_NOWAIT:  ::int_t = 2048;
pub const IPC_PRIVATE: ::int_t = 0;
pub const IPC_RMID:    ::int_t = 0;
pub const IPC_SET:     ::int_t = 1;
pub const IPC_STAT:    ::int_t = 2;
