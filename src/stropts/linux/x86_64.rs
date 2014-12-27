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

new!(strrecvfd);

#[repr(C)]
pub struct str_mlist {
    pub l_name: [::char_t, ..9u],
}

#[repr(C)]
pub struct str_list {
    pub sl_nmods: ::int_t,
    pub sl_modlist: *mut str_mlist,
}

pub const I_ATMARK:    ::int_t = 21279;
pub const I_CANPUT:    ::int_t = 21282;
pub const I_CKBAND:    ::int_t = 21277;
pub const I_FDINSERT:  ::int_t = 21264;
pub const I_FIND:      ::int_t = 21259;
pub const I_FLUSH:     ::int_t = 21253;
pub const I_FLUSHBAND: ::int_t = 21276;
pub const I_GETBAND:   ::int_t = 21278;
pub const I_GETCLTIME: ::int_t = 21281;
pub const I_GETSIG:    ::int_t = 21258;
pub const I_GRDOPT:    ::int_t = 21255;
pub const I_GWROPT:    ::int_t = 21268;
pub const I_LINK:      ::int_t = 21260;
pub const I_LIST:      ::int_t = 21269;
pub const I_LOOK:      ::int_t = 21252;
pub const I_NREAD:     ::int_t = 21249;
pub const I_PEEK:      ::int_t = 21263;
pub const I_PLINK:     ::int_t = 21270;
pub const I_POP:       ::int_t = 21251;
pub const I_PUNLINK:   ::int_t = 21271;
pub const I_PUSH:      ::int_t = 21250;
pub const I_RECVFD:    ::int_t = 21262;
pub const I_SENDFD:    ::int_t = 21265;
pub const I_SETCLTIME: ::int_t = 21280;
pub const I_SETSIG:    ::int_t = 21257;
pub const I_SRDOPT:    ::int_t = 21254;
pub const I_STR:       ::int_t = 21256;
pub const I_SWROPT:    ::int_t = 21267;
pub const I_UNLINK:    ::int_t = 21261;
pub const FMNAMESZ:    ::int_t = 8;
pub const FLUSHR:      ::int_t = 1;
pub const FLUSHRW:     ::int_t = 3;
pub const FLUSHW:      ::int_t = 2;
pub const S_BANDURG:   ::int_t = 512;
pub const S_ERROR:     ::int_t = 16;
pub const S_HANGUP:    ::int_t = 32;
pub const S_HIPRI:     ::int_t = 2;
pub const S_INPUT:     ::int_t = 1;
pub const S_MSG:       ::int_t = 8;
pub const S_OUTPUT:    ::int_t = 4;
pub const S_RDBAND:    ::int_t = 128;
pub const S_RDNORM:    ::int_t = 64;
pub const S_WRBAND:    ::int_t = 256;
pub const S_WRNORM:    ::int_t = 4;
pub const RS_HIPRI:    ::int_t = 1;
pub const RMSGD:       ::int_t = 1;
pub const RMSGN:       ::int_t = 2;
pub const RNORM:       ::int_t = 0;
pub const RPROTDAT:    ::int_t = 4;
pub const RPROTDIS:    ::int_t = 8;
pub const RPROTNORM:   ::int_t = 16;
pub const SNDZERO:     ::int_t = 1;
pub const ANYMARK:     ::int_t = 1;
pub const LASTMARK:    ::int_t = 2;
pub const MORECTL:     ::int_t = 1;
pub const MOREDATA:    ::int_t = 2;
pub const MSG_ANY:     ::int_t = 2;
pub const MSG_BAND:    ::int_t = 4;
pub const MSG_HIPRI:   ::int_t = 1;
