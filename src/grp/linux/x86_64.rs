#[repr(C)]
#[deriving(Copy)]
pub struct group {
    pub gr_name: *mut ::char_t,
    pub gr_passwd: *mut ::char_t,
    pub gr_gid: ::sys::types::gid_t,
    pub gr_mem: *mut *mut ::char_t,
}

new!(group);
