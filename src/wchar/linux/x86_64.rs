pub type wint_t = ::uint_t;
pub type wctype_t = ::ulong_t;

#[repr(C)]
#[deriving(Copy)]
pub struct mbstate_t {
    _data: [u32, ..2],
}

new!(mbstate_t);

pub const WEOF: wint_t = -1;
