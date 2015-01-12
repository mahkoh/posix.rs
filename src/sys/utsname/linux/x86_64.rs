#[repr(C)]
#[derive(Copy)]
pub struct utsname {
    pub sysname: [::char_t; 65us],
    pub nodename: [::char_t; 65us],
    pub release: [::char_t; 65us],
    pub version: [::char_t; 65us],
    pub machine: [::char_t; 65us],
    __domainname: [::char_t; 65us],
}

new!(utsname);
