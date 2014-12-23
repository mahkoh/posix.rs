pub type fexcept_t = ::ushort_t;

#[repr(C)]
pub struct fenv_t {
    __control_word: ::ushort_t,
    __glibc_reserved1: ::ushort_t,
    __status_word: ::ushort_t,
    __glibc_reserved2: ::ushort_t,
    __tags: ::ushort_t,
    __glibc_reserved3: ::ushort_t,
    __eip: ::uint_t,
    __cs_selector: ::ushort_t,
    __opcode: ::uint_t,
    __glibc_reserved4: ::uint_t,
    __data_offset: ::uint_t,
    __data_selector: ::ushort_t,
    __glibc_reserved5: ::ushort_t,
    __mxcsr: ::uint_t,
}

new!(fenv_t);

pub const FE_DIVBYZERO:  ::int_t = 4;
pub const FE_INEXACT:    ::int_t = 32;
pub const FE_INVALID:    ::int_t = 1;
pub const FE_OVERFLOW:   ::int_t = 8;
pub const FE_UNDERFLOW:  ::int_t = 16;
pub const FE_ALL_EXCEPT: ::int_t = 61;
pub const FE_DOWNWARD:   ::int_t = 1024;
pub const FE_TONEAREST:  ::int_t = 0;
pub const FE_TOWARDZERO: ::int_t = 3072;
pub const FE_UPWARD:     ::int_t = 2048;

pub fn FE_DFL_ENV() -> *const fenv_t {
    unsafe { ::std::mem::transmute(-1i64) }
}
