#[repr(C)]
pub struct utmpx {
    pub ut_type: ::short_t,
    pub ut_pid: ::sys::types::pid_t,
    pub ut_line: [::schar_t, ..32],
    pub ut_id: [::schar_t, ..4],
    pub ut_user: [::schar_t, ..32],
    pub ut_host: [::schar_t, ..256],
    pub ut_exit: [u16, ..2],
    pub ut_session: ::long_t,
    pub ut_tv: ::sys::time::timeval,
    pub ut_addr_v6: [::stdint::int32_t, ..4],
    __glibc_reserved: [u8, ..20],
}
new!(utmpx)
pub static EMPTY: ::short_t = 0;
pub static BOOT_TIME: ::short_t = 2;
pub static OLD_TIME: ::short_t = 4;
pub static NEW_TIME: ::short_t = 3;
pub static USER_PROCESS: ::short_t = 7;
pub static INIT_PROCESS: ::short_t = 5;
pub static LOGIN_PROCESS: ::short_t = 6;
pub static DEAD_PROCESS: ::short_t = 8;
