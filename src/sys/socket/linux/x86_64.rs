pub type socklen_t = ::uint_t;
pub type sa_family_t = ::ushort_t;

pub const SOCK_STREAM:    ::uint_t = 1;
pub const SOCK_DGRAM:     ::uint_t = 2;
pub const SOCK_RAW:       ::uint_t = 3;
pub const SOCK_SEQPACKET: ::uint_t = 5;

pub const SOL_SOCKET: ::int_t = 1;

#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [::char_t, ..14u],
}

impl ::AsSlice for sockaddr { }
impl ::AsMutSlice for sockaddr { }

new!(sockaddr);

#[repr(C)]
pub struct sockaddr_storage {
    pub ss_family: sa_family_t,
    __ss_align: ::ulong_t,
    __ss_padding: [::char_t, ..112u],
}

new!(sockaddr_storage);

impl ::AsSlice    for sockaddr_storage { }
impl ::AsMutSlice for sockaddr_storage { }

pub const MSG_OOB: ::uint_t = 1;
pub const MSG_PEEK: ::uint_t = 2;
pub const MSG_DONTROUTE: ::uint_t = 4;
pub const MSG_CTRUNC: ::uint_t = 8;
pub const MSG_TRUNC: ::uint_t = 32;
pub const MSG_EOR: ::uint_t = 128;
pub const MSG_WAITALL: ::uint_t = 256;
pub const MSG_NOSIGNAL: ::uint_t = 16384;

#[repr(C)]
pub struct msghdr {
    pub msg_name: *mut ::void_t,
    pub msg_namelen: socklen_t,
    pub msg_iov: *mut ::sys::uio::iovec,
    pub msg_iovlen: ::size_t,
    pub msg_control: *mut ::void_t,
    pub msg_controllen: ::size_t,
    pub msg_flags: ::int_t,
}

new!(msghdr);

pub fn CMSG_NXTHDR<'a>(msghdr: &'a mut msghdr,
                       cmsghdr: &mut cmsghdr) -> Option<&'a mut cmsghdr> {
    extern { fn __cmsg_nxthdr(mhdr: *mut msghdr, cmsg: *mut cmsghdr) -> *mut cmsghdr; }
    match unsafe { __cmsg_nxthdr(msghdr as *mut _, cmsghdr as *mut _) as uint } {
        0 => None,
        n => unsafe { Some(::std::mem::transmute(n as *mut cmsghdr)) },
    }
}

pub fn CMSG_FIRSTHDR<'a>(hdr: &'a msghdr) -> Option<&'a cmsghdr> {
    if hdr.msg_controllen as uint >= ::std::mem::size_of::<cmsghdr>() {
        unsafe { Some(::std::mem::transmute(hdr.msg_control)) }
    } else {
        None
    }
}

#[repr(C)]
pub struct cmsghdr {
    pub cmsg_len: ::size_t,
    pub cmsg_level: ::int_t,
    pub cmsg_type: ::int_t,
    __cmsg_data: [u8, ..0],
}

new!(cmsghdr);

pub fn CMSG_DATA(hdr: &mut cmsghdr) -> *mut ::uchar_t {
    hdr.__cmsg_data.as_mut_ptr()
}

pub const SCM_RIGHTS: ::uint_t = 1;

pub const SO_ACCEPTCONN: ::int_t = 30;
pub const SO_BROADCAST:  ::int_t = 6;
pub const SO_DEBUG:      ::int_t = 1;
pub const SO_DONTROUTE:  ::int_t = 5;
pub const SO_ERROR:      ::int_t = 4;
pub const SO_KEEPALIVE:  ::int_t = 9;
pub const SO_LINGER:     ::int_t = 13;
pub const SO_OOBINLINE:  ::int_t = 10;
pub const SO_RCVBUF:     ::int_t = 8;
pub const SO_RCVLOWAT:   ::int_t = 18;
pub const SO_RCVTIMEO:   ::int_t = 20;
pub const SO_REUSEADDR:  ::int_t = 2;
pub const SO_SNDBUF:     ::int_t = 7;
pub const SO_SNDLOWAT:   ::int_t = 19;
pub const SO_SNDTIMEO:   ::int_t = 21;
pub const SO_TYPE:       ::int_t = 3;

pub const SOMAXCONN: ::int_t = 128;

#[repr(C)]
pub struct linger {
    pub l_onoff: ::int_t,
    pub l_linger: ::int_t,
}

new!(linger);

pub const SHUT_RD:   ::int_t = 0;
pub const SHUT_WR:   ::int_t = 1;
pub const SHUT_RDWR: ::int_t = 2;

pub const AF_INET:   ::int_t = 2;
pub const AF_INET6:  ::int_t = 10;
pub const AF_UNIX:   ::int_t = 1;
pub const AF_UNSPEC: ::int_t = 0;
