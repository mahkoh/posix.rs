#[repr(C)]
pub struct FILE {
    _data: [u64, ..27],
}

#[repr(C)]
pub struct fpos_t {
    _data: [u64, ..2],
}

pub static BUFSIZ:       ::int_t = 8192;
pub static L_ctermid:    ::int_t = 9;
pub static L_tmpnam:     ::int_t = 20;
pub static _IOFBF:       ::int_t = 0;
pub static _IOLBF:       ::int_t = 1;
pub static _IONBF:       ::int_t = 2;
pub static SEEK_CUR:     ::int_t = 1;
pub static SEEK_END:     ::int_t = 2;
pub static SEEK_SET:     ::int_t = 0;
pub static FILENAME_MAX: ::int_t = 4096;
pub static FOPEN_MAX:    ::int_t = 16;
pub static TMP_MAX:      ::int_t = 238328;
pub static EOF:          ::int_t = -1;

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
