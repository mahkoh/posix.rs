use {char_t, int_t, void_t, NTStr};
use netinet::_in::{in_addr_t, in_addr, in6_addr};
use sys::socket::{socklen_t, AF_INET6, AF_INET};

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

pub fn inet_addr<T: NTStr>(cp: &T) -> in_addr_t {
    extern { fn inet_addr(cp: *const char_t) -> in_addr_t; }
    unsafe { inet_addr(cp.as_ptr()) }
}

extern {
    pub fn inet_ntoa(_in: in_addr) -> *mut char_t;
}

pub enum InAddr<'a> {
    Inet4(&'a in_addr),
    Inet6(&'a in6_addr),
}

pub fn inet_ntop(addr: InAddr, buf: &mut [u8]) {
    extern { fn inet_ntop(af: int_t, cp: *const void_t,
                          buf: *mut char_t, len: socklen_t) ->
                          *const char_t; }
    unsafe {
        match addr {
            InAddr::Inet4(v) => inet_ntop(AF_INET, v as *const _ as *const _,
                                          buf.as_mut_ptr() as *mut char_t,
                                          buf.len() as socklen_t),
            InAddr::Inet6(v) => inet_ntop(AF_INET6, v as *const _ as *const _,
                                          buf.as_mut_ptr() as *mut char_t,
                                          buf.len() as socklen_t),
        };
    }
}

pub enum MutInAddr<'a> {
    MutInet4(&'a mut in_addr),
    MutInet6(&'a mut in6_addr),
}

pub fn inet_pton(dst: &MutInAddr, src: &::NTStr) -> int_t {
    extern {
        fn inet_pton(af: int_t, cp: *const char_t, buf: *mut void_t) -> int_t;
    }
    unsafe {
        match *dst {
            MutInAddr::MutInet4(ref v) => inet_pton(AF_INET, src.as_ptr(),
                                                    *v as *mut _ as *mut _),
            MutInAddr::MutInet6(ref v) => inet_pton(AF_INET6, src.as_ptr(),
                                                    *v as *mut _ as *mut _),
        }
    }
}

#[cfg(test)]
mod tests {
    use std::num::Int;

    #[test]
    pub fn ntohl() {
        assert_eq!(Int::from_be(7531), super::ntohl(7531))
    }

    #[test]
    pub fn htons() {
        assert_eq!(7531u16.to_be(), super::htons(7531))
    }

    #[test]
    pub fn htonl() {
        assert_eq!(7531u32.to_be(), super::htonl(7531))
    }

    #[test]
    pub fn ntohs() {
        assert_eq!(Int::from_be(7531), super::ntohs(7531))
    }

    #[test]
    pub fn inet_addr() {
        use AsNTStr;
        let ip = super::inet_addr(&b"123.124.125.126\x00".as_nt_str());
        let ip2 = unsafe { ::std::mem::transmute([123u8,124,125,126]) };
        assert_eq!(ip, ip2)
    }

    #[test]
    pub fn inet_ntop() {
        // TODO
    }

    #[test]
    pub fn inet_pton() {
        // TODO
    }
}
