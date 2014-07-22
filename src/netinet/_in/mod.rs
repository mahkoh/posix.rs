pub use self::os::{in_port_t};
pub use self::os::{in_addr_t};
pub use self::os::{INADDR_ANY};
pub use self::os::{INADDR_BROADCAST};
pub use self::os::{in_addr};
pub use self::os::{IPPROTO_IP};
pub use self::os::{IPPROTO_ICMP};
pub use self::os::{IPPROTO_TCP};
pub use self::os::{IPPROTO_UDP};
pub use self::os::{IPPROTO_IPV6};
pub use self::os::{IPPROTO_RAW};
pub use self::os::{IPV6_JOIN_GROUP};
pub use self::os::{IPV6_LEAVE_GROUP};
pub use self::os::{IPV6_MULTICAST_HOPS};
pub use self::os::{IPV6_MULTICAST_IF};
pub use self::os::{IPV6_MULTICAST_LOOP};
pub use self::os::{IPV6_UNICAST_HOPS};
pub use self::os::{IPV6_V6ONLY};
pub use self::os::{in6_addr};
pub use self::os::{IN6_IS_ADDR_UNSPECIFIED};
pub use self::os::{IN6_IS_ADDR_LOOPBACK};
pub use self::os::{IN6_IS_ADDR_MULTICAST};
pub use self::os::{IN6_IS_ADDR_LINKLOCAL};
pub use self::os::{IN6_IS_ADDR_SITELOCAL};
pub use self::os::{IN6_IS_ADDR_V4MAPPED};
pub use self::os::{IN6_IS_ADDR_V4COMPAT};
pub use self::os::{IN6_IS_ADDR_MC_NODELOCAL};
pub use self::os::{IN6_IS_ADDR_MC_LINKLOCAL};
pub use self::os::{IN6_IS_ADDR_MC_SITELOCAL};
pub use self::os::{IN6_IS_ADDR_MC_ORGLOCAL};
pub use self::os::{IN6_IS_ADDR_MC_GLOBAL};
pub use self::os::{IN6ADDR_ANY_INIT};
pub use self::os::{sockaddr_in};
pub use self::os::{sockaddr_in6};
pub use self::os::{ipv6_mreq};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub static INET_ADDRSTRLEN: uint = 16;
pub static INET6_ADDRSTRLEN: uint = 46;

extern {
    pub static in6addr_any: in6_addr;
    pub static in6addr_loopback: in6_addr;
}
