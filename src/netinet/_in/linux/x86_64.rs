use std::num::Int;

pub type in_port_t = u16;
pub type in_addr_t = u32;

pub const INADDR_ANY:       in_addr_t = 0x00000000;
pub const INADDR_BROADCAST: in_addr_t = 0xffffffff;

#[repr(C)]
#[derive(Copy)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}

new!(in_addr);

impl ::AsSlice for in_addr { }
impl ::AsMutSlice for in_addr { }

pub const IPPROTO_IP:   ::int_t = 0;
pub const IPPROTO_ICMP: ::int_t = 1;
pub const IPPROTO_TCP:  ::int_t = 6;
pub const IPPROTO_UDP:  ::int_t = 17;
pub const IPPROTO_IPV6: ::int_t = 41;
pub const IPPROTO_RAW:  ::int_t = 255;

pub const IPV6_JOIN_GROUP:     ::int_t = 20;
pub const IPV6_LEAVE_GROUP:    ::int_t = 21;
pub const IPV6_MULTICAST_HOPS: ::int_t = 18;
pub const IPV6_MULTICAST_IF:   ::int_t = 17;
pub const IPV6_MULTICAST_LOOP: ::int_t = 19;
pub const IPV6_UNICAST_HOPS:   ::int_t = 16;
pub const IPV6_V6ONLY:         ::int_t = 26;

#[repr(C)]
#[derive(Copy)]
pub struct in6_addr {
    pub data: [u32; 4u],
}

new!(in6_addr);

impl ::AsSlice for in6_addr { }
impl ::AsMutSlice for in6_addr { }

pub fn IN6_IS_ADDR_UNSPECIFIED(a: &in6_addr) -> bool {
    a.data.as_slice() == [0; 4].as_slice()
}

pub fn IN6_IS_ADDR_LOOPBACK(a: &in6_addr) -> bool {
    a.data.as_slice() == [0, 0, 0, 1u32.to_be()].as_slice()
}

pub fn IN6_IS_ADDR_MULTICAST(a: &in6_addr) -> bool {
    a.addr8()[0] == 0xff
}

pub fn IN6_IS_ADDR_LINKLOCAL(a: &in6_addr) -> bool {
    a.data[0] & 0xffc00000u32.to_be() == 0xfe800000u32.to_be()
}

pub fn IN6_IS_ADDR_SITELOCAL(a: &in6_addr) -> bool {
    a.data[0] & 0xffc00000u32.to_be() == 0xfec00000u32.to_be()
}

pub fn IN6_IS_ADDR_V4MAPPED(a: &in6_addr) -> bool {
    a.data[0] == 0 && a.data[1] == 0 && a.data[2] == 0xffffu32.to_be()
}

pub fn IN6_IS_ADDR_V4COMPAT(a: &in6_addr) -> bool {
    a.data[0] == 0 && a.data[1] == 0 && a.data[2] == 0 && Int::from_be(a.data[3]) > 1
}

pub fn IN6_IS_ADDR_MC_NODELOCAL(a: &in6_addr) -> bool {
	IN6_IS_ADDR_MULTICAST(a) && (a.addr8()[1] & 0xf) == 0x1
}

pub fn IN6_IS_ADDR_MC_LINKLOCAL(a: &in6_addr) -> bool {
	IN6_IS_ADDR_MULTICAST(a) && (a.addr8()[1] & 0xf) == 0x2
}

pub fn IN6_IS_ADDR_MC_SITELOCAL(a: &in6_addr) -> bool {
    IN6_IS_ADDR_MULTICAST(a) && (a.addr8()[1] & 0xf) == 0x5
}

pub fn IN6_IS_ADDR_MC_ORGLOCAL(a: &in6_addr) -> bool {
	IN6_IS_ADDR_MULTICAST(a) && (a.addr8()[1] & 0xf) == 0x8
}

pub fn IN6_IS_ADDR_MC_GLOBAL(a: &in6_addr) -> bool {
	IN6_IS_ADDR_MULTICAST(a) && (a.addr8()[1] & 0xf) == 0xe
}

pub const IN6ADDR_ANY_INIT: in6_addr = in6_addr { data: [0; 4] };

impl in6_addr {
    pub fn addr8(&self) -> &[u8; 16u] {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn addr16(&self) -> &[u16; 8u] {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn addr32(&self) -> &[u32; 4u] {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn addr8_mut(&mut self) -> &mut [u8; 16u] {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn addr16_mut(&mut self) -> &mut [u16; 8u] {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn addr32_mut(&mut self) -> &mut [u32; 4u] {
        unsafe { ::std::mem::transmute(self) }
    }
}

#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_in {
    pub sin_family: ::sys::socket::sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [::uchar_t; 8u],
}

new!(sockaddr_in);

impl ::AsSlice for sockaddr_in { }
impl ::AsMutSlice for sockaddr_in { }

#[repr(C)]
#[derive(Copy)]
pub struct sockaddr_in6 {
    pub sin6_family: ::sys::socket::sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: u32,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: u32,
}

new!(sockaddr_in6);

impl ::AsSlice for sockaddr_in6 { }
impl ::AsMutSlice for sockaddr_in6 { }

#[repr(C)]
#[derive(Copy)]
pub struct ipv6_mreq {
    pub ipv6mr_multiaddr: in6_addr,
    pub ipv6mr_interface: ::uint_t,
}

new!(ipv6_mreq);
