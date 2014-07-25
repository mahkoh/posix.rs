#[repr(C)]
pub struct timeval {
    pub tv_sec: ::sys::types::time_t,
    pub tv_usec: ::sys::types::suseconds_t,
}
new!(timeval)
#[repr(C)]
pub struct fd_set {
    _bits: [u32, ..32],
}
new!(fd_set)
pub static FD_SETSIZE: ::int_t = 1024;

pub fn FD_CLR(fd: ::int_t, set: &mut fd_set) {
    let fd = fd as uint;
    set._bits[fd / 32] &= !(1 << (fd % 32));
}

pub fn FD_ISSET(fd: ::int_t, set: &mut fd_set) -> ::int_t {
    let fd = fd as uint;
    (set._bits[fd / 32] & (1 << (fd % 32)) != 0) as ::int_t
}

pub fn FD_SET(fd: ::int_t, set: &mut fd_set) {
    let fd = fd as uint;
    set._bits[fd / 32] |= 1 << (fd % 32);
}

pub fn FD_ZERO(set: &mut fd_set) {
    set._bits = [0, ..32u];
}
