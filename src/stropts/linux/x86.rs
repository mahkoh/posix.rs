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
pub static I_ATMARK:    ::int_t = (('S' as ::int_t << 8) |31);
pub static I_CANPUT:    ::int_t = (('S' as ::int_t << 8) |34);
pub static I_CKBAND:    ::int_t = (('S' as ::int_t << 8) |29);
pub static I_FDINSERT:  ::int_t = (('S' as ::int_t << 8) |16);
pub static I_FIND:      ::int_t = (('S' as ::int_t << 8) |11);
pub static I_FLUSH:     ::int_t = (('S' as ::int_t << 8) |     5);
pub static I_FLUSHBAND: ::int_t = (('S' as ::int_t << 8) |28);
pub static I_GETBAND:   ::int_t = (('S' as ::int_t << 8) |30);
pub static I_GETCLTIME: ::int_t = (('S' as ::int_t << 8) |33);
pub static I_GETSIG:    ::int_t = (('S' as ::int_t << 8) |10);
pub static I_GRDOPT:    ::int_t = (('S' as ::int_t << 8) |     7);
pub static I_GWROPT:    ::int_t = (('S' as ::int_t << 8) |20);
pub static I_LINK:      ::int_t = (('S' as ::int_t << 8) |12);
pub static I_LIST:      ::int_t = (('S' as ::int_t << 8) |21);
pub static I_LOOK:      ::int_t = (('S' as ::int_t << 8) |     4);
pub static I_NREAD:     ::int_t = (('S' as ::int_t << 8) |     1);
pub static I_PEEK:      ::int_t = (('S' as ::int_t << 8) |15);
pub static I_PLINK:     ::int_t = (('S' as ::int_t << 8) |22);
pub static I_POP:       ::int_t = (('S' as ::int_t << 8) |     3);
pub static I_PUNLINK:   ::int_t = (('S' as ::int_t << 8) |23);
pub static I_PUSH:      ::int_t = (('S' as ::int_t << 8) |     2);
pub static I_RECVFD:    ::int_t = (('S' as ::int_t << 8) |14);
pub static I_SENDFD:    ::int_t = (('S' as ::int_t << 8) |17);
pub static I_SETCLTIME: ::int_t = (('S' as ::int_t << 8) |32);
pub static I_SETSIG:    ::int_t = (('S' as ::int_t << 8) |     9);
pub static I_SRDOPT:    ::int_t = (('S' as ::int_t << 8) |     6);
pub static I_STR:       ::int_t = (('S' as ::int_t << 8) |     8);
pub static I_SWROPT:    ::int_t = (('S' as ::int_t << 8) |19);
pub static I_UNLINK:    ::int_t = (('S' as ::int_t << 8) |13);
pub static FMNAMESZ: ::int_t = 8;
pub static FLUSHR: ::int_t = 0x01;
pub static FLUSHRW: ::int_t = 0x03;
pub static FLUSHW: ::int_t = 0x02;
pub static S_BANDURG: ::int_t = 0x0200;
pub static S_ERROR: ::int_t = 0x0010;
pub static S_HANGUP: ::int_t = 0x0020;
pub static S_HIPRI: ::int_t = 0x0002;
pub static S_INPUT: ::int_t = 0x0001;
pub static S_MSG: ::int_t = 0x0008;
pub static S_OUTPUT: ::int_t = 0x0004;
pub static S_RDBAND: ::int_t = 0x0080;
pub static S_RDNORM: ::int_t = 0x0040;
pub static S_WRBAND: ::int_t = 0x0100;
pub static S_WRNORM: ::int_t = 0x0004;
pub static RS_HIPRI: ::int_t = 0x01;
pub static RMSGD: ::int_t = 0x0001;
pub static RMSGN: ::int_t = 0x0002;
pub static RNORM: ::int_t = 0x0000;
pub static RPROTDAT: ::int_t = 0x0004;
pub static RPROTDIS: ::int_t = 0x0008;
pub static RPROTNORM: ::int_t = 0x0010;
pub static SNDZERO: ::int_t = 0x001;
pub static ANYMARK: ::int_t = 0x01;
pub static LASTMARK: ::int_t = 0x02;
pub static MORECTL: ::int_t = 1;
pub static MOREDATA: ::int_t = 2;
pub static MSG_ANY: ::int_t = 0x02;
pub static MSG_BAND: ::int_t = 0x04;
pub static MSG_HIPRI: ::int_t = 0x01;
