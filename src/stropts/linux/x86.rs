pub type t_scalar_t = ::long_t;
pub type t_uscalar_t = ::ulong_t;
#[repr(C)]
pub struct bandinfo {
    pub bi_pri: ::uchar_t,
    pub bi_flag: ::int_t,
}
new!(bandinfo)
#[repr(C)]
pub struct strbuf {
    pub maxlen: ::int_t,
    pub len: ::int_t,
    pub buf: *mut ::schar_t,
}
new!(strbuf)
#[repr(C)]
pub struct strpeek {
    pub ctlbuf: strbuf,
    pub databuf: strbuf,
    pub flags: t_uscalar_t,
}
new!(strpeek)
#[repr(C)]
pub struct strfdinsert {
    pub ctlbuf: strbuf,
    pub databuf: strbuf,
    pub flags: t_uscalar_t,
    pub fildes: ::int_t,
    pub offset: ::int_t,
}
new!(strfdinsert)
#[repr(C)]
pub struct strioctl {
    pub ic_cmd: ::int_t,
    pub ic_timout: ::int_t,
    pub ic_len: ::int_t,
    pub ic_dp: *mut ::schar_t,
}
new!(strioctl)
#[repr(C)]
pub struct strrecvfd {
    pub fd: ::int_t,
    pub uid: ::sys::types::uid_t,
    pub gid: ::sys::types::gid_t,
    __fill: [u8, ..8],
}
new!(strrecvfd)
#[repr(C)]
pub struct str_mlist {
    pub l_name: [::schar_t, ..9],
}
new!(str_mlist)
#[repr(C)]
pub struct str_list {
    pub sl_nmods: ::int_t,
    pub sl_modlist: *mut str_mlist,
}
new!(str_list)
pub const I_ATMARK:    ::int_t = (('S' as ::int_t << 8) |31);
pub const I_CANPUT:    ::int_t = (('S' as ::int_t << 8) |34);
pub const I_CKBAND:    ::int_t = (('S' as ::int_t << 8) |29);
pub const I_FDINSERT:  ::int_t = (('S' as ::int_t << 8) |16);
pub const I_FIND:      ::int_t = (('S' as ::int_t << 8) |11);
pub const I_FLUSH:     ::int_t = (('S' as ::int_t << 8) |     5);
pub const I_FLUSHBAND: ::int_t = (('S' as ::int_t << 8) |28);
pub const I_GETBAND:   ::int_t = (('S' as ::int_t << 8) |30);
pub const I_GETCLTIME: ::int_t = (('S' as ::int_t << 8) |33);
pub const I_GETSIG:    ::int_t = (('S' as ::int_t << 8) |10);
pub const I_GRDOPT:    ::int_t = (('S' as ::int_t << 8) |     7);
pub const I_GWROPT:    ::int_t = (('S' as ::int_t << 8) |20);
pub const I_LINK:      ::int_t = (('S' as ::int_t << 8) |12);
pub const I_LIST:      ::int_t = (('S' as ::int_t << 8) |21);
pub const I_LOOK:      ::int_t = (('S' as ::int_t << 8) |     4);
pub const I_NREAD:     ::int_t = (('S' as ::int_t << 8) |     1);
pub const I_PEEK:      ::int_t = (('S' as ::int_t << 8) |15);
pub const I_PLINK:     ::int_t = (('S' as ::int_t << 8) |22);
pub const I_POP:       ::int_t = (('S' as ::int_t << 8) |     3);
pub const I_PUNLINK:   ::int_t = (('S' as ::int_t << 8) |23);
pub const I_PUSH:      ::int_t = (('S' as ::int_t << 8) |     2);
pub const I_RECVFD:    ::int_t = (('S' as ::int_t << 8) |14);
pub const I_SENDFD:    ::int_t = (('S' as ::int_t << 8) |17);
pub const I_SETCLTIME: ::int_t = (('S' as ::int_t << 8) |32);
pub const I_SETSIG:    ::int_t = (('S' as ::int_t << 8) |     9);
pub const I_SRDOPT:    ::int_t = (('S' as ::int_t << 8) |     6);
pub const I_STR:       ::int_t = (('S' as ::int_t << 8) |     8);
pub const I_SWROPT:    ::int_t = (('S' as ::int_t << 8) |19);
pub const I_UNLINK:    ::int_t = (('S' as ::int_t << 8) |13);
pub const FMNAMESZ: ::int_t = 8;
pub const FLUSHR: ::int_t = 0x01;
pub const FLUSHRW: ::int_t = 0x03;
pub const FLUSHW: ::int_t = 0x02;
pub const S_BANDURG: ::int_t = 0x0200;
pub const S_ERROR: ::int_t = 0x0010;
pub const S_HANGUP: ::int_t = 0x0020;
pub const S_HIPRI: ::int_t = 0x0002;
pub const S_INPUT: ::int_t = 0x0001;
pub const S_MSG: ::int_t = 0x0008;
pub const S_OUTPUT: ::int_t = 0x0004;
pub const S_RDBAND: ::int_t = 0x0080;
pub const S_RDNORM: ::int_t = 0x0040;
pub const S_WRBAND: ::int_t = 0x0100;
pub const S_WRNORM: ::int_t = 0x0004;
pub const RS_HIPRI: ::int_t = 0x01;
pub const RMSGD: ::int_t = 0x0001;
pub const RMSGN: ::int_t = 0x0002;
pub const RNORM: ::int_t = 0x0000;
pub const RPROTDAT: ::int_t = 0x0004;
pub const RPROTDIS: ::int_t = 0x0008;
pub const RPROTNORM: ::int_t = 0x0010;
pub const SNDZERO: ::int_t = 0x001;
pub const ANYMARK: ::int_t = 0x01;
pub const LASTMARK: ::int_t = 0x02;
pub const MORECTL: ::int_t = 1;
pub const MOREDATA: ::int_t = 2;
pub const MSG_ANY: ::int_t = 0x02;
pub const MSG_BAND: ::int_t = 0x04;
pub const MSG_HIPRI: ::int_t = 0x01;
