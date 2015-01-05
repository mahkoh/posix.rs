#[repr(C)]
#[derive(Copy)]
pub struct tms {
    pub tms_utime: ::sys::types::clock_t,
    pub tms_stime: ::sys::types::clock_t,
    pub tms_cutime: ::sys::types::clock_t,
    pub tms_cstime: ::sys::types::clock_t,
}
new!(tms);
