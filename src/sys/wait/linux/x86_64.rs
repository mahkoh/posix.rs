pub type idtype_t = ::int_t;
pub static P_ALL:  idtype_t = 0;
pub static P_PID:  idtype_t = 1;
pub static P_PGID: idtype_t = 2;

pub static WCONTINUED: ::int_t = 8;
pub static WNOHANG:    ::int_t = 1;
pub static WUNTRACED:  ::int_t = 2;
pub static WEXITED:    ::int_t = 4;
pub static WNOWAIT:    ::int_t = 16777216;
pub static WSTOPPED:   ::int_t = 2;

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
    (((((status) & 0x7f) + 1) >> 1) as i8) > 0
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
