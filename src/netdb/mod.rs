pub use self::os::{netent};
pub use self::os::{hostent};
pub use self::os::{servent};
pub use self::os::{protoent};
pub use self::os::{addrinfo};
pub use self::os::{IPPORT_RESERVED};
pub use self::os::{AI_PASSIVE};
pub use self::os::{AI_CANONNAME};
pub use self::os::{AI_NUMERICHOST};
pub use self::os::{AI_NUMERICSERV};
pub use self::os::{AI_V4MAPPED};
pub use self::os::{AI_ALL};
pub use self::os::{AI_ADDRCONFIG};
pub use self::os::{NI_NOFQDN};
pub use self::os::{NI_NUMERICHOST};
pub use self::os::{NI_NAMEREQD};
pub use self::os::{NI_NUMERICSERV};
pub use self::os::{NI_DGRAM};
pub use self::os::{EAI_AGAIN};
pub use self::os::{EAI_BADFLAGS};
pub use self::os::{EAI_FAIL};
pub use self::os::{EAI_FAMILY};
pub use self::os::{EAI_MEMORY};
pub use self::os::{EAI_NONAME};
pub use self::os::{EAI_SERVICE};
pub use self::os::{EAI_SOCKTYPE};
pub use self::os::{EAI_SYSTEM};
pub use self::os::{EAI_OVERFLOW};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn endhostent() {
    extern { fn endhostent(); }
    unsafe { endhostent() }
}

pub fn endnetent() {
    extern { fn endnetent(); }
    unsafe { endnetent() }
}

pub fn endprotoent() {
    extern { fn endprotoent(); }
    unsafe { endprotoent() }
}

pub fn endservent() {
    extern { fn endservent(); }
    unsafe { endservent() }
}

pub fn freeaddrinfo(ai: *mut addrinfo) {
    extern { fn freeaddrinfo(ai: *mut addrinfo); }
    unsafe { freeaddrinfo(ai) }
}

pub fn gai_strerror(ecode: ::int_t) -> *const ::char_t {
    extern { fn gai_strerror(ecode: ::int_t) -> *const ::char_t; }
    unsafe { gai_strerror(ecode) }
}

pub fn getaddrinfo<T: ::NTStr, U: ::NTStr>
                   (name: &T, service: &U, req: &addrinfo, pai: &mut *mut addrinfo) -> ::int_t {
    extern { fn getaddrinfo(name: *const ::char_t, service: *const ::char_t,
                            req: *const addrinfo, pai: *mut *mut addrinfo) -> ::int_t; }
    unsafe { getaddrinfo(name.as_ptr(), service.as_ptr(), req as *const _,
                         pai as *mut _) }
}

pub fn getnameinfo(sa: &[u8], host: Option<&mut [u8]>, serv: Option<&mut [u8]>,
                   flags: ::int_t) -> ::int_t {
    extern { fn getnameinfo(sa: *const ::sys::socket::sockaddr, salen: ::sys::socket::socklen_t,
                            host: *mut ::char_t, hostlen: ::sys::socket::socklen_t,
                            serv: *mut ::char_t, servlen: ::sys::socket::socklen_t,
                            flags: ::int_t) -> ::int_t; }
    let host_len = host.as_ref().map(|v| v.len() as ::sys::socket::socklen_t).unwrap_or(0);
    let serv_len = serv.as_ref().map(|v| v.len() as ::sys::socket::socklen_t).unwrap_or(0);
    let host_ptr = host.map(|v| v.as_mut_ptr() as *mut _).unwrap_or(0 as *mut _);
    let serv_ptr = serv.map(|v| v.as_mut_ptr() as *mut _).unwrap_or(0 as *mut _);
    if host_len + serv_len == 0 {
        return EAI_OVERFLOW;
    }
    unsafe { getnameinfo(sa.as_ptr() as *const _, sa.len() as ::sys::socket::socklen_t,
                         host_ptr, host_len, serv_ptr, serv_len, flags) }
}

pub fn getnetbyaddr(net: u32, ty: ::int_t) -> &'static mut netent {
    extern { fn getnetbyaddr(net: u32, ty: ::int_t) -> *mut netent; }
    unsafe { ::std::mem::transmute(getnetbyaddr(net, ty)) }
}

pub fn getnetbyname<T: ::NTStr>(name: &T) -> &'static mut netent {
    extern { fn getnetbyname(name: *const ::char_t) -> *mut netent; }
    unsafe { ::std::mem::transmute(getnetbyname(name.as_ptr())) }
}

pub fn getprotobyname<T: ::NTStr>(name: &T) -> &'static mut protoent {
    extern { fn getprotobyname(name: *const ::char_t) -> *mut protoent; }
    unsafe { ::std::mem::transmute(getprotobyname(name.as_ptr())) }
}

pub fn getprotobynumber(proto: ::int_t) -> &'static mut protoent {
    extern { fn getprotobynumber(proto: ::int_t) -> *mut protoent; }
    unsafe { ::std::mem::transmute(getprotobynumber(proto)) }
}

pub fn getservbyname<T: ::NTStr, U: ::NTStr>(name: &T,
                                             proto: &U) -> &'static mut servent {
    extern { fn getservbyname(name: *const ::char_t,
                              proto: *const ::char_t) -> *mut servent; }
    unsafe { ::std::mem::transmute(getservbyname(name.as_ptr(), proto.as_ptr())) }
}

extern "C" {
    pub fn gethostent() -> *mut hostent;
    pub fn getnetent() -> *mut netent;
    pub fn getprotoent() -> *mut protoent;
    pub fn getservent() -> *mut servent;
    pub fn sethostent(stay_open: ::int_t);
    pub fn setnetent(stay_open: ::int_t);
    pub fn setprotoent(stay_open: ::int_t);
    pub fn setservent(stay_open: ::int_t);
}
