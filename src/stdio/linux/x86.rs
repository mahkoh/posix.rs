#[repr(C)]
#[deriving(Copy)]
pub struct FILE {
    _flags: [u32, ..1],
    _IO_read_ptr: [u32, ..1],
    _IO_read_end: [u32, ..1],
    _IO_read_base: [u32, ..1],
    _IO_write_base: [u32, ..1],
    _IO_write_ptr: [u32, ..1],
    _IO_write_end: [u32, ..1],
    _IO_buf_base: [u32, ..1],
    _IO_buf_end: [u32, ..1],
    _IO_save_base: [u32, ..1],
    _IO_backup_base: [u32, ..1],
    _IO_save_end: [u32, ..1],
    _markers: [u32, ..1],
    _chain: [u32, ..1],
    _fileno: [u32, ..1],
    _flags2: [u32, ..1],
    _old_offset: [u32, ..1],
    _cur_column: [u16, ..1],
    _vtable_offset: [u8, ..1],
    _shortbuf: [u8, ..1],
    _lock: [u32, ..1],
    _offset: [u32, ..2],
    __pad1: [u32, ..1],
    __pad2: [u32, ..1],
    __pad3: [u32, ..1],
    __pad4: [u32, ..1],
    __pad5: [u32, ..1],
    _mode: [u32, ..1],
    _unused2: [u8, ..40],
}
new!(FILE);
pub type fpos_t = [u32, ..3];
pub const BUFSIZ: ::int_t = 8192;
pub const L_ctermid: ::int_t = 9;
pub const L_tmpnam: ::int_t = 20;
pub const _IOFBF: ::int_t = 0;
pub const _IOLBF: ::int_t = 1;
pub const _IONBF: ::int_t = 2;
pub const SEEK_CUR: ::int_t = 1;
pub const SEEK_END: ::int_t = 2;
pub const SEEK_SET: ::int_t = 0;
pub const FILENAME_MAX: ::int_t = 4096;
pub const FOPEN_MAX: ::int_t = 16;
pub const TMP_MAX: ::int_t = 238328;
pub const EOF: ::int_t = (-1);

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
