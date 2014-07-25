#[repr(C)]
pub struct sem_t {
    __size: [u8, ..16],
    __align: [u32, ..1],
}
new!(sem_t)

pub static SEM_FAILED: *mut sem_t = 0 as *mut _;
