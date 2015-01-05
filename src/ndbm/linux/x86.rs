#[repr(C)]
#[derive(Copy)]
pub struct datum {
    pub dptr: *mut ::schar_t,
    pub dsize: ::int_t,
}
new!(datum);
#[repr(C)]
#[derive(Copy)]
pub struct DBM {
    pub file: [u8; 0],
    pub dirfd: ::int_t,
    _dbm_memory: [u32; 2],
    _dbm_fetch_val: [u32; 1],
    _dbm_errno: [u32; 1],
}
new!(DBM);
pub const DBM_INSERT: ::int_t = 0;
pub const DBM_REPLACE: ::int_t = 1;
