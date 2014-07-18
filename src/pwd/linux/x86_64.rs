#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::char_t,
    pub pw_passwd: *mut ::char_t,
    pub pw_uid: ::sys::types::uid_t,
    pub pw_gid: ::sys::types::gid_t,
    pub pw_gecos: *mut ::char_t,
    pub pw_dir: *mut ::char_t,
    pub pw_shell: *mut ::char_t,
}
