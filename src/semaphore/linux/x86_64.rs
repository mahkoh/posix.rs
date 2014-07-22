#[repr(C)]
pub struct sem_t {
    _data: [u64, ..4u],
}

new!(sem_t)

pub static SEM_FAILED: *mut sem_t = 0 as *mut _;
