pub type wint_t = ::uint_t;
pub type wctype_t = ::ulong_t;

#[repr(C)]
pub struct mbstate_t {
    pub _data: [u32, ..2],
}

pub static WEOF: wint_t = -1;
