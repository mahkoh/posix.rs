pub type t_scalar_t = ::int_t;
pub type t_uscalar_t = ::uint_t;

#[repr(C)]
pub struct bandinfo {
    pub bi_pri: ::uchar_t,
    pub bi_flag: ::int_t,
}

#[repr(C)]
pub struct strbuf {
    pub maxlen: ::int_t,
    pub len: ::int_t,
    pub buf: *mut ::char_t,
}

#[repr(C)]
pub struct strpeek {
    pub ctlbuf: strbuf,
    pub databuf: strbuf,
    pub flags: t_uscalar_t,
}

#[repr(C)]
pub struct strfdinsert {
    pub ctlbuf: strbuf,
    pub databuf: strbuf,
    pub flags: t_uscalar_t,
    pub fildes: ::int_t,
    pub offset: ::int_t,
}

#[repr(C)]
pub struct strioctl {
    pub ic_cmd: ::int_t,
    pub ic_timout: ::int_t,
    pub ic_len: ::int_t,
    pub ic_dp: *mut ::char_t,
}

#[repr(C)]
pub struct strrecvfd {
    pub fd: ::int_t,
    pub uid: ::sys::types::uid_t,
    pub gid: ::sys::types::gid_t,
    __fill: [::char_t, ..8u],
}

#[repr(C)]
pub struct str_mlist {
    pub l_name: [::char_t, ..9u],
}

#[repr(C)]
pub struct str_list {
    pub sl_nmods: ::int_t,
    pub sl_modlist: *mut str_mlist,
}

pub static I_ATMARK:    ::int_t = 21279;
pub static I_CANPUT:    ::int_t = 21282;
pub static I_CKBAND:    ::int_t = 21277;
pub static I_FDINSERT:  ::int_t = 21264;
pub static I_FIND:      ::int_t = 21259;
pub static I_FLUSH:     ::int_t = 21253;
pub static I_FLUSHBAND: ::int_t = 21276;
pub static I_GETBAND:   ::int_t = 21278;
pub static I_GETCLTIME: ::int_t = 21281;
pub static I_GETSIG:    ::int_t = 21258;
pub static I_GRDOPT:    ::int_t = 21255;
pub static I_GWROPT:    ::int_t = 21268;
pub static I_LINK:      ::int_t = 21260;
pub static I_LIST:      ::int_t = 21269;
pub static I_LOOK:      ::int_t = 21252;
pub static I_NREAD:     ::int_t = 21249;
pub static I_PEEK:      ::int_t = 21263;
pub static I_PLINK:     ::int_t = 21270;
pub static I_POP:       ::int_t = 21251;
pub static I_PUNLINK:   ::int_t = 21271;
pub static I_PUSH:      ::int_t = 21250;
pub static I_RECVFD:    ::int_t = 21262;
pub static I_SENDFD:    ::int_t = 21265;
pub static I_SETCLTIME: ::int_t = 21280;
pub static I_SETSIG:    ::int_t = 21257;
pub static I_SRDOPT:    ::int_t = 21254;
pub static I_STR:       ::int_t = 21256;
pub static I_SWROPT:    ::int_t = 21267;
pub static I_UNLINK:    ::int_t = 21261;
pub static FMNAMESZ:    ::int_t = 8;
pub static FLUSHR:      ::int_t = 1;
pub static FLUSHRW:     ::int_t = 3;
pub static FLUSHW:      ::int_t = 2;
pub static S_BANDURG:   ::int_t = 512;
pub static S_ERROR:     ::int_t = 16;
pub static S_HANGUP:    ::int_t = 32;
pub static S_HIPRI:     ::int_t = 2;
pub static S_INPUT:     ::int_t = 1;
pub static S_MSG:       ::int_t = 8;
pub static S_OUTPUT:    ::int_t = 4;
pub static S_RDBAND:    ::int_t = 128;
pub static S_RDNORM:    ::int_t = 64;
pub static S_WRBAND:    ::int_t = 256;
pub static S_WRNORM:    ::int_t = 4;
pub static RS_HIPRI:    ::int_t = 1;
pub static RMSGD:       ::int_t = 1;
pub static RMSGN:       ::int_t = 2;
pub static RNORM:       ::int_t = 0;
pub static RPROTDAT:    ::int_t = 4;
pub static RPROTDIS:    ::int_t = 8;
pub static RPROTNORM:   ::int_t = 16;
pub static SNDZERO:     ::int_t = 1;
pub static ANYMARK:     ::int_t = 1;
pub static LASTMARK:    ::int_t = 2;
pub static MORECTL:     ::int_t = 1;
pub static MOREDATA:    ::int_t = 2;
pub static MSG_ANY:     ::int_t = 2;
pub static MSG_BAND:    ::int_t = 4;
pub static MSG_HIPRI:   ::int_t = 1;
