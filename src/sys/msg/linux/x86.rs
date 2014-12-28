pub type msgqnum_t = ::ulong_t;
pub type msglen_t = ::ulong_t;
#[repr(C)]
#[deriving(Copy)]
pub struct msqid_ds {
    pub msg_perm: ::sys::ipc::ipc_perm,
    pub msg_stime: ::sys::types::time_t,
    __glibc_reserved1: [u32, ..1],
    pub msg_rtime: ::sys::types::time_t,
    __glibc_reserved2: [u32, ..1],
    pub msg_ctime: ::sys::types::time_t,
    __glibc_reserved3: [u32, ..1],
    __msg_cbytes: [u32, ..1],
    pub msg_qnum: msgqnum_t,
    pub msg_qbytes: msglen_t,
    pub msg_lspid: ::sys::types::pid_t,
    pub msg_lrpid: ::sys::types::pid_t,
    __glibc_reserved4: [u32, ..1],
    __glibc_reserved5: [u32, ..1],
}
new!(msqid_ds);
#[repr(C)]
#[deriving(Copy)]
pub struct msginfo {
    pub msgpool: ::int_t,
    pub msgmap: ::int_t,
    pub msgmax: ::int_t,
    pub msgmnb: ::int_t,
    pub msgmni: ::int_t,
    pub msgssz: ::int_t,
    pub msgtql: ::int_t,
    pub msgseg: ::ushort_t,
}
new!(msginfo);
pub const MSG_NOERROR: ::int_t = 0o10000;
