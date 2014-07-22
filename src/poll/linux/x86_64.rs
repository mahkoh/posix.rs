pub type nfds_t = ::ulong_t;

#[repr(C)]
pub struct pollfd {
    pub fd: ::int_t,
    pub events: ::short_t,
    pub revents: ::short_t,
}

new!(pollfd)

pub static POLLIN:     ::short_t = 1;
pub static POLLRDNORM: ::short_t = 64;
pub static POLLRDBAND: ::short_t = 128;
pub static POLLPRI:    ::short_t = 2;
pub static POLLOUT:    ::short_t = 4;
pub static POLLWRNORM: ::short_t = 256;
pub static POLLWRBAND: ::short_t = 512;
pub static POLLERR:    ::short_t = 8;
pub static POLLHUP:    ::short_t = 16;
pub static POLLNVAL:   ::short_t = 32;
