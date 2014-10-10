pub type nfds_t = ::ulong_t;
#[repr(C)]
pub struct pollfd {
    pub fd: ::int_t,
    pub events: ::short_t,
    pub revents: ::short_t,
}
new!(pollfd)
pub const POLLIN: ::short_t = 0x001;
pub const POLLRDNORM: ::short_t = 0x040;
pub const POLLRDBAND: ::short_t = 0x080;
pub const POLLPRI: ::short_t = 0x002;
pub const POLLOUT: ::short_t = 0x004;
pub const POLLWRNORM: ::short_t = 0x100;
pub const POLLWRBAND: ::short_t = 0x200;
pub const POLLERR: ::short_t = 0x008;
pub const POLLHUP: ::short_t = 0x010;
pub const POLLNVAL: ::short_t = 0x020;
