#[repr(C)]
#[derive(Copy)]
pub struct datum {
    pub dptr: *mut ::char_t,
    pub dsize: ::int_t,
}

pub type DBM = ::void_t;

pub const DBM_INSERT:  ::int_t = 0;
pub const DBM_REPLACE: ::int_t = 1;
