#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: ::sys::socket::sa_family_t,
    pub sun_path: [::char_t, ..108u],
}
