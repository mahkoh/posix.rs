pub use self::os::{datum};
pub use self::os::DBM;
pub use self::os::{DBM_INSERT};
pub use self::os::{DBM_REPLACE};

use {NTStr, int_t, char_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn dbm_open<T: NTStr>(file: &T, flags: int_t, mode: int_t) -> *mut DBM {
    extern { fn dbm_open(file: *const char_t, flags: int_t, mode: int_t) -> *mut DBM; }
    unsafe { dbm_open(file.as_ptr(), flags, mode) }
}

pub fn dbm_close(dbf: *mut DBM) {
    extern { fn dbm_close(dbf: *mut DBM); }
    unsafe { dbm_close(dbf) }
}

pub fn dbm_fetch(dbf: *mut DBM, key: datum) -> datum {
    extern { fn dbm_fetch(dbf: *mut DBM, key: datum) -> datum; }
    unsafe { dbm_fetch(dbf, key) }
}

pub fn dbm_store(dbf: *mut DBM, key: datum, content: datum, flags: int_t) -> int_t {
    extern { fn dbm_store(dbf: *mut DBM, key: datum, content: datum,
                          flags: int_t) -> int_t; }
    unsafe { dbm_store(dbf, key, content, flags) }
}

pub fn dbm_delete(dbf: *mut DBM, key: datum) -> int_t {
    extern { fn dbm_delete(dbf: *mut DBM, key: datum) -> int_t; }
    unsafe { dbm_delete(dbf, key) }
}

pub fn dbm_firstkey(dbf: *mut DBM) -> datum {
    extern { fn dbm_firstkey(dbf: *mut DBM) -> datum; }
    unsafe { dbm_firstkey(dbf) }
}

pub fn dbm_nextkey(dbf: *mut DBM) -> datum {
    extern { fn dbm_nextkey(dbf: *mut DBM) -> datum; }
    unsafe { dbm_nextkey(dbf) }
}

pub fn dbm_error(dbf: *mut DBM) -> int_t {
    extern { fn dbm_error(dbf: *mut DBM) -> int_t; }
    unsafe { dbm_error(dbf) }
}

pub fn dbm_clearerr(dbf: *mut DBM) {
    extern { fn dbm_clearerr(dbf: *mut DBM); }
    unsafe { dbm_clearerr(dbf) }
}
