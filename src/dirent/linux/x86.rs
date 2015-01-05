#[repr(C)]
#[derive(Copy)]
pub struct dirent {
    pub d_ino: ::sys::types::ino_t,
    pub d_off: ::sys::types::off_t,
    pub d_reclen: ::ushort_t,
    pub d_type: ::uchar_t,
    pub d_name: [::schar_t; 256],
}
new!(dirent);
#[repr(C)]
#[derive(Copy)]
pub struct DIR {
    _dummy: (),
}
new!(DIR);
