#[repr(C)]
#[derive(Copy)]
pub struct utsname {
    pub sysname: [::char_t; 65u],
    pub nodename: [::char_t; 65u],
    pub release: [::char_t; 65u],
    pub version: [::char_t; 65u],
    pub machine: [::char_t; 65u],
    __domainname: [::char_t; 65u],
}

new!(utsname);
