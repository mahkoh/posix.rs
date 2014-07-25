#[repr(C)]
pub struct group {
    pub gr_name: *mut ::schar_t,
    pub gr_passwd: *mut ::schar_t,
    pub gr_gid: ::sys::types::gid_t,
    pub gr_mem: *mut *mut ::schar_t,
}
new!(group)
