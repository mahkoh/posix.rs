#[repr(C)]
pub struct sockaddr_un {
    pub sun_family: ::sys::socket::sa_family_t,
    pub sun_path: [::schar_t, ..108],
}
new!(sockaddr_un)
