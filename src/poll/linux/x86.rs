pub type nfds_t = ::ulong_t;
#[repr(C)]
pub struct pollfd {
    pub fd: ::int_t,
    pub events: ::short_t,
    pub revents: ::short_t,
}
new!(pollfd)
pub static POLLIN: ::short_t = 0x001;
pub static POLLRDNORM: ::short_t = 0x040;
pub static POLLRDBAND: ::short_t = 0x080;
pub static POLLPRI: ::short_t = 0x002;
pub static POLLOUT: ::short_t = 0x004;
pub static POLLWRNORM: ::short_t = 0x100;
pub static POLLWRBAND: ::short_t = 0x200;
pub static POLLERR: ::short_t = 0x008;
pub static POLLHUP: ::short_t = 0x010;
pub static POLLNVAL: ::short_t = 0x020;
