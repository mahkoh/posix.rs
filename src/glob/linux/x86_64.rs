#[repr(C)]
#[deriving(Copy)]
pub struct glob_t {
    pub gl_pathc: ::size_t,
    pub gl_pathv: *mut *mut ::char_t,
    pub gl_offs: ::size_t,
    pub gl_flags: ::int_t,
    pub gl_closedir: ::std::option::Option<extern fn
                                               (arg1: *mut ::void_t)>,
    pub gl_readdir: ::std::option::Option<extern fn(arg1: *mut ::void_t)
                                              -> *mut ::void_t>,
    pub gl_opendir: ::std::option::Option<extern fn(arg1: *const ::char_t)
                                              -> *mut ::void_t>,
    pub gl_lstat: ::std::option::Option<extern fn
                                            (arg1: *const ::char_t,
                                             arg2: *mut ::void_t) -> ::int_t>,
    pub gl_stat: ::std::option::Option<extern fn
                                           (arg1: *const ::char_t,
                                            arg2: *mut ::void_t) -> ::int_t>,
}

pub const GLOB_APPEND:   ::int_t = 32;
pub const GLOB_DOOFFS:   ::int_t = 8;
pub const GLOB_ERR:      ::int_t = 1;
pub const GLOB_MARK:     ::int_t = 2;
pub const GLOB_NOCHECK:  ::int_t = 16;
pub const GLOB_NOESCAPE: ::int_t = 64;
pub const GLOB_NOSORT:   ::int_t = 4;
pub const GLOB_ABORTED:  ::int_t = 2;
pub const GLOB_NOMATCH:  ::int_t = 3;
pub const GLOB_NOSPACE:  ::int_t = 1;
