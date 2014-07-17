pub fn ntohl(v: u32) -> u32 {
    extern { fn ntohl(netlong: u32) -> u32; }
    unsafe { ntohl(v) }
}

pub fn htons(v: u16) -> u16 {
    extern { fn htons(hostshort: u16) -> u16; }
    unsafe { htons(v) }
}

pub fn htonl(v: u32) -> u32 {
    extern { fn htonl(hostlong: u32) -> u32; }
    unsafe { htonl(v) }
}

pub fn ntohs(v: u16) -> u16 {
    extern { fn ntohs(netshort: u16) -> u16; }
    unsafe { ntohs(v) }
}

pub fn inet_addr(cp: &::NTStr) -> ::netinet::_in::in_addr_t {
    extern { fn inet_addr(cp: *const ::char_t) -> ::netinet::_in::in_addr_t; }
    unsafe { inet_addr(cp.as_ptr()) }
}

pub fn inet_ntoa(_in: ::netinet::_in::in_addr) -> *mut ::char_t {
    extern { fn inet_ntoa(_in: ::netinet::_in::in_addr) -> *mut ::char_t; }
    unsafe { inet_ntoa(_in) }
}

pub enum InAddr<'a> {
    Inet4(&'a ::netinet::_in::in_addr),
    Inet6(&'a ::netinet::_in::in6_addr),
}

pub fn inet_ntop(addr: InAddr, buf: &mut [u8]) -> *const ::char_t {
    extern { fn inet_ntop(af: ::int_t, cp: *const ::void_t,
                          buf: *mut ::char_t, len: ::sys::socket::socklen_t) ->
                          *const ::char_t; }
    unsafe {
        match addr {
            Inet4(v) => inet_ntop(::sys::socket::AF_INET, v as *const _ as *const _,
                                  buf.as_mut_ptr() as *mut ::char_t,
                                  buf.len() as ::sys::socket::socklen_t),
            Inet6(v) => inet_ntop(::sys::socket::AF_INET6, v as *const _ as *const _,
                                  buf.as_mut_ptr() as *mut ::char_t,
                                  buf.len() as ::sys::socket::socklen_t),
        }
    }
}

pub enum MutInAddr<'a> {
    MutInet4(&'a mut ::netinet::_in::in_addr),
    MutInet6(&'a mut ::netinet::_in::in6_addr),
}

pub fn inet_pton(dst: &MutInAddr, src: &::NTStr) -> ::int_t {
    extern {
        fn inet_pton(af: ::int_t, cp: *const ::char_t, buf: *mut ::void_t) -> ::int_t;
    }
    unsafe {
        match *dst {
            MutInet4(ref v) => inet_pton(::sys::socket::AF_INET, src.as_ptr(), *v as *mut _ as *mut _),
            MutInet6(ref v) => inet_pton(::sys::socket::AF_INET6, src.as_ptr(), *v as *mut _ as *mut _),
        }
    }
}

