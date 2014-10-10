#![allow(non_uppercase_statics)]

#[repr(C)]
pub struct FILE {
    _data: [u64, ..27],
}

#[repr(C)]
pub struct fpos_t {
    _data: [u64, ..2],
}

pub const BUFSIZ:       ::int_t = 8192;
pub const L_ctermid:    ::int_t = 9;
pub const L_tmpnam:     ::int_t = 20;
pub const _IOFBF:       ::int_t = 0;
pub const _IOLBF:       ::int_t = 1;
pub const _IONBF:       ::int_t = 2;
pub const SEEK_CUR:     ::int_t = 1;
pub const SEEK_END:     ::int_t = 2;
pub const SEEK_SET:     ::int_t = 0;
pub const FILENAME_MAX: ::int_t = 4096;
pub const FOPEN_MAX:    ::int_t = 16;
pub const TMP_MAX:      ::int_t = 238328;
pub const EOF:          ::int_t = -1;

pub fn stdin() -> *mut FILE {
    extern { static mut stdin: *mut FILE; }
    unsafe { stdin }
}

pub fn stdout() -> *mut FILE {
    extern { static mut stdout: *mut FILE; }
    unsafe { stdout }
}

pub fn stderr() -> *mut FILE {
    extern { static mut stderr: *mut FILE; }
    unsafe { stderr }
}
