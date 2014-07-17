#[repr(C)]
pub struct dirent {
    pub d_ino: ::sys::types::ino_t,
    pub d_off: ::sys::types::off_t,
    pub d_reclen: ::ushort_t,
    pub d_type: ::uchar_t,
    pub d_name: [::char_t, ..256u],
}

pub type DIR = ::void_t;
