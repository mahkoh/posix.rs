#[repr(C)]
#[derive(Copy)]
struct __exit_status {
    __e_termination: ::short_t,
    __e_exit: ::short_t,
}

#[repr(C)]
#[derive(Copy)]
pub struct utmpx {
    pub ut_type:          ::short_t,
    pub ut_pid:           ::sys::types::pid_t,
    pub ut_line:          [::char_t; 32u],
    pub ut_id:            [::char_t; 4u],
    pub ut_user:          [::char_t; 32u],
    pub ut_host:          [::char_t; 256u],
    ut_exit:          __exit_status,
    pub ut_session:       i32,
    pub ut_tv:            ::sys::time::timeval,
    pub ut_addr_v6:       [i32; 4u],
    __glibc_reserved: [::char_t; 20u],
}

new!(utmpx);

pub const EMPTY:         ::short_t = 0;
pub const BOOT_TIME:     ::short_t = 2;
pub const OLD_TIME:      ::short_t = 4;
pub const NEW_TIME:      ::short_t = 3;
pub const USER_PROCESS:  ::short_t = 7;
pub const INIT_PROCESS:  ::short_t = 5;
pub const LOGIN_PROCESS: ::short_t = 6;
pub const DEAD_PROCESS:  ::short_t = 8;
