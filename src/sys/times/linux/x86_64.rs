#[repr(C)]
pub struct tms {
    pub tms_utime:  ::sys::types::clock_t,
    pub tms_stime:  ::sys::types::clock_t,
    pub tms_cutime: ::sys::types::clock_t,
    pub tms_cstime: ::sys::types::clock_t,
}

impl tms {
    pub fn new() -> tms {
        unsafe { ::std::mem::zeroed() }
    }
}
