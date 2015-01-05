pub use self::os::{socklen_t};
pub use self::os::{sa_family_t};
pub use self::os::{SOCK_STREAM};
pub use self::os::{SOCK_DGRAM};
pub use self::os::{SOCK_RAW};
pub use self::os::{SOCK_SEQPACKET};
pub use self::os::{SOL_SOCKET};
pub use self::os::{sockaddr};
pub use self::os::{sockaddr_storage};
pub use self::os::{MSG_OOB};
pub use self::os::{MSG_PEEK};
pub use self::os::{MSG_DONTROUTE};
pub use self::os::{MSG_CTRUNC};
pub use self::os::{MSG_TRUNC};
pub use self::os::{MSG_EOR};
pub use self::os::{MSG_WAITALL};
pub use self::os::{MSG_NOSIGNAL};
pub use self::os::{msghdr};
pub use self::os::{cmsghdr};
pub use self::os::{SCM_RIGHTS};
pub use self::os::{SO_ACCEPTCONN};
pub use self::os::{SO_BROADCAST};
pub use self::os::{SO_DEBUG};
pub use self::os::{SO_DONTROUTE};
pub use self::os::{SO_ERROR};
pub use self::os::{SO_KEEPALIVE};
pub use self::os::{SO_LINGER};
pub use self::os::{SO_OOBINLINE};
pub use self::os::{SO_RCVBUF};
pub use self::os::{SO_RCVLOWAT};
pub use self::os::{SO_RCVTIMEO};
pub use self::os::{SO_REUSEADDR};
pub use self::os::{SO_SNDBUF};
pub use self::os::{SO_SNDLOWAT};
pub use self::os::{SO_SNDTIMEO};
pub use self::os::{SO_TYPE};
pub use self::os::{SOMAXCONN};
pub use self::os::{linger};
pub use self::os::{SHUT_RD};
pub use self::os::{SHUT_WR};
pub use self::os::{SHUT_RDWR};
pub use self::os::{AF_INET};
pub use self::os::{AF_INET6};
pub use self::os::{AF_UNIX};
pub use self::os::{AF_UNSPEC};
pub use self::os::{CMSG_NXTHDR};
pub use self::os::{CMSG_FIRSTHDR};
pub use self::os::{CMSG_DATA};

use {int_t, size_t, void_t, ssize_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn accept(fd: int_t, addr: &mut [u8]) -> int_t {
    extern { 
        fn accept(fd: int_t, addr: *mut sockaddr, addrlen: *mut socklen_t) -> int_t;
    }
    let mut len = addr.len() as socklen_t;
    unsafe { accept(fd, addr.as_mut_ptr() as *mut _, &mut len as *mut _) }
}

pub fn bind(fd: int_t, addr: &[u8]) -> int_t {
    extern { fn bind(fd: int_t, addr: *const sockaddr, len: socklen_t) -> int_t; }
    unsafe { bind(fd, addr.as_ptr() as *const _, addr.len() as socklen_t) }
}

pub fn connect(fd: int_t, addr: &[u8]) -> int_t {
    extern { fn connect(fd: int_t, addr: *const sockaddr, len: socklen_t) -> int_t; }
    unsafe { connect(fd, addr.as_ptr() as *const _, addr.len() as socklen_t) }
}

pub fn getpeername(fd: int_t, addr: &mut [u8]) -> int_t {
    extern {
        fn getpeername(fd: int_t, addr: *mut sockaddr, len: *mut socklen_t) -> int_t;
    }
    let mut len = addr.len() as socklen_t;
    unsafe { getpeername(fd, addr.as_mut_ptr() as *mut _, &mut len as *mut _) }
}

pub fn getsockname(fd: int_t, addr: &mut [u8]) -> int_t {
    extern {
        fn getsockname(fd: int_t, addr: *mut sockaddr, len: *mut socklen_t) -> int_t;
    }
    let mut len = addr.len() as socklen_t;
    unsafe { getsockname(fd, addr.as_mut_ptr() as *mut _, &mut len as *mut _) }
}

pub fn getsockopt(fd: int_t, level: int_t, optname: int_t,
                  optval: &mut [u8]) -> int_t {
    extern {
        fn getsockopt(fd: int_t, level: int_t, optname: int_t,
                      optval: *mut void_t, optlen: *mut socklen_t) -> int_t;
    }
    let mut len = optval.len() as socklen_t;
    unsafe { getsockopt(fd, level, optname, optval.as_mut_ptr() as *mut _,
                        &mut len as *mut _) }
}

pub fn listen(fd: int_t, backlog: int_t) -> int_t {
    extern { fn listen(fd: int_t, n: int_t) -> int_t; }
    unsafe { listen(fd, backlog) }
}

pub fn recv(fd: int_t, buf: &mut [u8], flags: int_t) -> ssize_t {
    extern {
        fn recv(fd: int_t, buf: *mut void_t, n: size_t,
                flags: int_t) -> ssize_t;
    }
    unsafe { recv(fd, buf.as_mut_ptr() as *mut _, buf.len() as size_t, flags) }
}

pub fn recvfrom(fd: int_t, buf: &mut [u8], flags: int_t,
                addr: &mut [u8]) -> ssize_t {
    extern {
        fn recvfrom(fd: int_t, buf: *mut void_t, n: size_t,
                    flags: int_t, addr: *mut sockaddr,
                    addr_len: *mut socklen_t) -> ssize_t;
    }
    let mut len = addr.len() as socklen_t;
    unsafe { recvfrom(fd, buf.as_mut_ptr() as *mut _, buf.len() as size_t, flags,
                      addr.as_mut_ptr() as *mut _, &mut len as *mut _) }
}

pub fn send(fd: int_t, buf: &[u8], flags: int_t) -> ssize_t {
    extern {
        fn send(fd: int_t, buf: *const void_t, n: size_t,
                flags: int_t) -> ssize_t;
    }
    unsafe { send(fd, buf.as_ptr() as *const _, buf.len() as size_t, flags) }
}

pub fn sendto(fd: int_t, buf: &[u8], flags: int_t, addr: &[u8]) -> ssize_t {
    extern {
        fn sendto(fd: int_t, buf: *const void_t, n: size_t, flags: int_t,
                  addr: *const sockaddr, addr_len: socklen_t) -> ssize_t;
    }
    unsafe { sendto(fd, buf.as_ptr() as *const _, buf.len() as size_t, flags,
                    addr.as_ptr() as *const _, addr.len() as socklen_t) }
}

pub fn setsockopt(fd: int_t, level: int_t, optname: int_t,
                  optval: &[u8]) -> int_t {
    extern {
        fn setsockopt(fd: int_t, level: int_t, optname: int_t,
                      optval: *const void_t, optlen: socklen_t) -> int_t;
    }
    unsafe { setsockopt(fd, level, optname, optval.as_ptr() as *const _,
                        optval.len() as socklen_t) }
}

pub fn shutdown(fd: int_t, how: int_t) -> int_t {
    extern { fn shutdown(fd: int_t, how: int_t) -> int_t; }
    unsafe { shutdown(fd, how) }
}

pub fn sockatmark(fd: int_t) -> int_t {
    extern { fn sockatmark(fd: int_t) -> int_t; }
    unsafe { sockatmark(fd) }
}

pub fn socket(domain: int_t, ty: int_t, protocol: int_t) -> int_t {
    extern { fn socket(domain: int_t, ty: int_t, protocol: int_t) -> int_t; }
    unsafe { socket(domain, ty, protocol) }
}

pub fn socketpair(domain: int_t, ty: int_t, protocol: int_t,
                  fds: &mut [int_t; 2]) -> int_t {
    extern {
        fn socketpair(domain: int_t, ty: int_t, protocol: int_t,
                      fds: *mut int_t) -> int_t;
    }
    unsafe { socketpair(domain, ty, protocol, fds.as_mut_ptr()) }
}

extern "C" {
    pub fn recvmsg(fd: int_t, message: *mut msghdr,
                   flags: int_t) -> ssize_t;
    pub fn sendmsg(fd: int_t, message: *const msghdr,
                   flags: int_t) -> ssize_t;
}
