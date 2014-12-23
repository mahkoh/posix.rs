#[repr(C)]
pub struct wordexp_t {
    pub we_wordc: ::size_t,
    pub we_wordv: *mut *mut ::char_t,
    pub we_offs: ::size_t,
}

new!(wordexp_t);

pub const WRDE_APPEND:  ::int_t = 2;
pub const WRDE_DOOFFS:  ::int_t = 1;
pub const WRDE_NOCMD:   ::int_t = 4;
pub const WRDE_REUSE:   ::int_t = 8;
pub const WRDE_SHOWERR: ::int_t = 16;
pub const WRDE_UNDEF:   ::int_t = 32;
pub const WRDE_BADCHAR: ::int_t = 2;
pub const WRDE_BADVAL:  ::int_t = 3;
pub const WRDE_CMDSUB:  ::int_t = 4;
pub const WRDE_NOSPACE: ::int_t = 1;
pub const WRDE_SYNTAX:  ::int_t = 5;
