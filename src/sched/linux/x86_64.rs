#[repr(C)]
#[deriving(Copy)]
pub struct sched_param {
    pub sched_priority: ::int_t,
}

pub const SCHED_FIFO:  ::int_t = 1;
pub const SCHED_RR:    ::int_t = 2;
pub const SCHED_OTHER: ::int_t = 0;
