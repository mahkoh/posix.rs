#[repr(C)]
pub struct timeval {
    pub tv_sec: ::sys::types::time_t,
    pub tv_usec: ::sys::types::suseconds_t,
}

#[repr(C)]
pub struct fd_set {
    _bits: [::long_t, ..16u],
}

pub static FD_SETSIZE: ::int_t = 1024;

pub fn FD_CLR(fd: ::int_t, set: &mut fd_set) {
    let fd = fd as uint;
    set._bits[fd / 64] &= !(1 << (fd % 64));
}

pub fn FD_ISSET(fd: ::int_t, set: &mut fd_set) -> ::int_t {
    let fd = fd as uint;
    (set._bits[fd / 64] & (1 << (fd % 64)) != 0) as ::int_t
}

pub fn FD_SET(fd: ::int_t, set: &mut fd_set) {
    let fd = fd as uint;
    set._bits[fd / 64] |= 1 << (fd % 64);
}

pub fn FD_ZERO(set: &mut fd_set) {
    set._bits = [0, ..16u];
}
