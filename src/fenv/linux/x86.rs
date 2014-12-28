pub type fexcept_t = ::ushort_t;
#[repr(C)]
#[deriving(Copy)]
pub struct fenv_t {
    __control_word: [u16, ..1],
    __glibc_reserved1: [u16, ..1],
    __status_word: [u16, ..1],
    __glibc_reserved2: [u16, ..1],
    __tags: [u16, ..1],
    __glibc_reserved3: [u16, ..1],
    __eip: [u32, ..1],
    __cs_selector: [u16, ..1],
    __opcode: [u32, ..1],
    __glibc_reserved4: [u32, ..1],
    __data_offset: [u32, ..1],
    __data_selector: [u16, ..1],
    __glibc_reserved5: [u16, ..1],
}
new!(fenv_t);
pub const FE_DIVBYZERO: ::int_t = 0x04;
pub const FE_INEXACT: ::int_t = 0x20;
pub const FE_INVALID: ::int_t = 0x01;
pub const FE_OVERFLOW: ::int_t = 0x08;
pub const FE_UNDERFLOW: ::int_t = 0x10;
pub const FE_ALL_EXCEPT: ::int_t = (0x20 | 0x04 | 0x10 | 0x08 | 0x01);
pub const FE_DOWNWARD: ::int_t = 0x400;
pub const FE_TONEAREST: ::int_t = 0;
pub const FE_TOWARDZERO: ::int_t = 0xc00;
pub const FE_UPWARD: ::int_t = 0x800;

pub fn FE_DFL_ENV() -> *const fenv_t {
    unsafe { ::std::mem::transmute(-1i32) }
}
