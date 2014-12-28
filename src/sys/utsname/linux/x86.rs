#[repr(C)]
#[deriving(Copy)]
pub struct utsname {
    pub sysname: [::schar_t, ..65],
    pub nodename: [::schar_t, ..65],
    pub release: [::schar_t, ..65],
    pub version: [::schar_t, ..65],
    pub machine: [::schar_t, ..65],
    __domainname: [u8, ..65],
}
new!(utsname);
