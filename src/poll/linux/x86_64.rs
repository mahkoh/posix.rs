pub type nfds_t = ::ulong_t;

#[repr(C)]
pub struct pollfd {
    pub fd: ::int_t,
    pub events: ::short_t,
    pub revents: ::short_t,
}

new!(pollfd)

pub const POLLIN:     ::short_t = 1;
pub const POLLRDNORM: ::short_t = 64;
pub const POLLRDBAND: ::short_t = 128;
pub const POLLPRI:    ::short_t = 2;
pub const POLLOUT:    ::short_t = 4;
pub const POLLWRNORM: ::short_t = 256;
pub const POLLWRBAND: ::short_t = 512;
pub const POLLERR:    ::short_t = 8;
pub const POLLHUP:    ::short_t = 16;
pub const POLLNVAL:   ::short_t = 32;
