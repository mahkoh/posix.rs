pub type msgqnum_t = ::ulong_t;
pub type msglen_t = ::ulong_t;

#[repr(C)]
pub struct msqid_ds {
    pub msg_perm: ::sys::ipc::ipc_perm,
    pub msg_stime: ::sys::types::time_t,
    pub msg_rtime: ::sys::types::time_t,
    pub msg_ctime: ::sys::types::time_t,
    __msg_cbytes: ::ulong_t,
    pub msg_qnum: msgqnum_t,
    pub msg_qbytes: msglen_t,
    pub msg_lspid: ::sys::types::pid_t,
    pub msg_lrpid: ::sys::types::pid_t,
    __glibc_reserved4: ::ulong_t,
    __glibc_reserved5: ::ulong_t,
}

#[repr(C)]
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

pub static MSG_NOERROR: ::int_t = 4096;
