#[repr(C)]
pub struct sched_param {
    __sched_priority: [u32, ..1],
}
new!(sched_param)
pub static SCHED_FIFO: ::int_t = 1;
pub static SCHED_RR: ::int_t = 2;
pub static SCHED_OTHER: ::int_t = 0;
