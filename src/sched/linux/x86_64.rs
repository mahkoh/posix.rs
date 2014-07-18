#[repr(C)]
pub struct sched_param {
    pub sched_priority: ::int_t,
}

pub static SCHED_FIFO:  ::int_t = 1;
pub static SCHED_RR:    ::int_t = 2;
pub static SCHED_OTHER: ::int_t = 0;
