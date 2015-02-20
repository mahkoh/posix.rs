pub type mqd_t = ::int_t;

#[repr(C)]
#[derive(Copy)]
pub struct mq_attr {
    pub mq_flags: ::long_t,
    pub mq_maxmsg: ::long_t,
    pub mq_msgsize: ::long_t,
    pub mq_curmsgs: ::long_t,
    __pad: [::long_t; 4usize],
}
