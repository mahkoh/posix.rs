#[repr(C)]
pub struct datum {
    pub dptr: *mut ::char_t,
    pub dsize: ::int_t,
}

pub type DBM = ::void_t;

pub static DBM_INSERT:  ::int_t = 0;
pub static DBM_REPLACE: ::int_t = 1;
