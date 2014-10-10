pub type idtype_t = ::int_t;
pub const P_ALL:  idtype_t = 0;
pub const P_PID:  idtype_t = 1;
pub const P_PGID: idtype_t = 2;

pub const WCONTINUED: ::int_t = 8;
pub const WNOHANG:    ::int_t = 1;
pub const WUNTRACED:  ::int_t = 2;
pub const WEXITED:    ::int_t = 4;
pub const WNOWAIT:    ::int_t = 16777216;
pub const WSTOPPED:   ::int_t = 2;

pub fn WEXITSTATUS(status: ::int_t) -> ::int_t {
    (status & 0xff00) >> 8
}

pub fn WIFCONTINUED(status: ::int_t) -> bool {
    status == 0xffff
}

pub fn WIFEXITED(status: ::int_t) -> bool {
    WTERMSIG(status) == 0
}

pub fn WIFSIGNALED(status: ::int_t) -> bool {
    ((((status) & 0x7f) + 1) as i8 >> 1) > 0
}

pub fn WIFSTOPPED(status: ::int_t) -> bool {
    (status & 0xff) == 0x7f
}

pub fn WSTOPSIG(status: ::int_t) -> ::int_t {
    WEXITSTATUS(status)
}

pub fn WTERMSIG(status: ::int_t) -> ::int_t {
    status & 0x7f
}
