#[repr(C)]
pub struct sched_param {
    __sched_priority: [u32, ..1],
}
new!(sched_param);
pub const SCHED_FIFO: ::int_t = 1;
pub const SCHED_RR: ::int_t = 2;
pub const SCHED_OTHER: ::int_t = 0;
