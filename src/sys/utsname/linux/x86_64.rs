#[repr(C)]
#[derive(Copy)]
pub struct utsname {
    pub sysname: [::char_t; 65usize],
    pub nodename: [::char_t; 65usize],
    pub release: [::char_t; 65usize],
    pub version: [::char_t; 65usize],
    pub machine: [::char_t; 65usize],
    __domainname: [::char_t; 65usize],
}

new!(utsname);
