#[repr(C)]
pub struct passwd {
    pub pw_name: *mut ::schar_t,
    pub pw_passwd: *mut ::schar_t,
    pub pw_uid: ::sys::types::uid_t,
    pub pw_gid: ::sys::types::gid_t,
    pub pw_gecos: *mut ::schar_t,
    pub pw_dir: *mut ::schar_t,
    pub pw_shell: *mut ::schar_t,
}
new!(passwd)
