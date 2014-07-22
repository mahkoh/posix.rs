pub use self::os::{FILE};
pub use self::os::{fpos_t};
pub use self::os::{BUFSIZ};
pub use self::os::{L_ctermid};
pub use self::os::{L_tmpnam};
pub use self::os::{_IOFBF};
pub use self::os::{_IOLBF};
pub use self::os::{_IONBF};
pub use self::os::{SEEK_CUR};
pub use self::os::{SEEK_END};
pub use self::os::{SEEK_SET};
pub use self::os::{FILENAME_MAX};
pub use self::os::{FOPEN_MAX};
pub use self::os::{TMP_MAX};
pub use self::os::{EOF};
pub use self::os::{stdin};
pub use self::os::{stdout};
pub use self::os::{stderr};

use {NTStr, char_t, size_t, int_t, ssize_t, void_t, long_t};
use sys::types::{off_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn clearerr(stream: *mut FILE) {
    extern { fn clearerr(stream: *mut FILE); }
    unsafe { clearerr(stream); }
}

extern {
    pub fn ctermid(s: *mut char_t) -> *mut char_t;
}

extern {
    pub fn dprintf(fd: int_t, fmt: *const char_t, ...) -> int_t;
}

pub fn fclose(stream: *mut FILE) -> int_t {
    extern { fn fclose(stream: *mut FILE) -> int_t; }
    unsafe { fclose(stream) }
}

pub fn fdopen<T: NTStr>(fd: int_t, modes: &T) -> *mut FILE {
    extern { fn fdopen(fd: int_t, modes: *const char_t) -> *mut FILE; }
    unsafe { fdopen(fd, modes.as_ptr()) }
}

pub fn feof(stream: *mut FILE) -> int_t {
    extern { fn feof(stream: *mut FILE) -> int_t; }
    unsafe { feof(stream) }
}

pub fn ferror(stream: *mut FILE) -> int_t {
    extern { fn ferror(stream: *mut FILE) -> int_t; }
    unsafe { ferror(stream) }
}

pub fn fflush(stream: *mut FILE) -> int_t {
    extern { fn fflush(stream: *mut FILE) -> int_t; }
    unsafe { fflush(stream) }
}

pub fn fgetc(stream: *mut FILE) -> int_t {
    extern { fn fgetc(stream: *mut FILE) -> int_t; }
    unsafe { fgetc(stream) }
}

pub fn fgetpos(stream: *mut FILE, pos: &mut fpos_t) -> int_t {
    extern { fn fgetpos(stream: *mut FILE, pos: *mut fpos_t) -> int_t; }
    unsafe { fgetpos(stream, pos as *mut _) }
}

pub fn fgets(s: &mut [u8], stream: *mut FILE) -> *mut char_t {
    extern { fn fgets(s: *mut char_t, n: int_t, stream: *mut FILE) -> *mut char_t; }
    unsafe { fgets(s.as_mut_ptr() as *mut _, s.len() as int_t, stream) }
}

pub fn fileno(stream: *mut FILE) -> int_t {
    extern { fn fileno(stream: *mut FILE) -> int_t; }
    unsafe { fileno(stream) }
}

pub fn flockfile(stream: *mut FILE) {
    extern { fn flockfile(stream: *mut FILE); }
    unsafe { flockfile(stream); }
}

pub fn fmemopen(s: &mut [u8], modes: *const char_t) -> *mut FILE {
    extern { fn fmemopen(s: *mut void_t, len: size_t, modes: *const char_t) -> *mut FILE; }
    unsafe { fmemopen(s.as_ptr() as *mut _, s.len() as size_t, modes) }
}

pub fn fopen<T: NTStr, U: NTStr>(filename: &T, modes: &U) -> *mut FILE {
    extern { fn fopen(filename: *const char_t, modes: *const char_t) -> *mut FILE; }
    unsafe { fopen(filename.as_ptr(), modes.as_ptr()) }
}

extern {
    pub fn fprintf(stream: *mut FILE, format: *const char_t, ...) -> int_t;
}

pub fn fputc(c: int_t, stream: *mut FILE) -> int_t {
    extern { fn fputc(c: int_t, stream: *mut FILE) -> int_t; }
    unsafe { fputc(c, stream) }
}

pub fn fputs<T: NTStr>(s: &T, stream: *mut FILE) -> int_t {
    extern { fn fputs(s: *const char_t, stream: *mut FILE) -> int_t; }
    unsafe { fputs(s.as_ptr(), stream) }
}

extern {
    pub fn fread(ptr: *mut void_t, size: size_t, n: size_t, stream: *mut FILE) -> size_t;
    pub fn fread_unlocked(ptr: *mut void_t, size: size_t, n: size_t,
                          stream: *mut FILE) -> size_t;
}

pub fn freopen<T: NTStr, U: NTStr>(filename: &T, modes: &U,
                                   stream: *mut FILE) -> *mut FILE {
    extern { fn freopen(filename: *const char_t, modes: *const char_t,
                        stream: *mut FILE) -> *mut FILE; }
    unsafe { freopen(filename.as_ptr(), modes.as_ptr(), stream) }
}

extern {
    pub fn fscanf(stream: *mut FILE, format: *const char_t, ...) -> int_t;
}

pub fn fseeko(stream: *mut FILE, off: off_t, whence: int_t) -> int_t {
    extern { fn fseeko(stream: *mut FILE, off: off_t, whence: int_t) -> int_t; }
    unsafe { fseeko(stream, off, whence) }
}

pub fn fseek(stream: *mut FILE, off: long_t, whence: int_t) -> int_t {
    extern { fn fseek(stream: *mut FILE, off: long_t, whence: int_t) -> int_t; }
    unsafe { fseek(stream, off, whence) }
}

pub fn fsetpos(stream: *mut FILE, pos: &fpos_t) -> int_t {
    extern { fn fsetpos(stream: *mut FILE, pos: *const fpos_t) -> int_t; }
    unsafe { fsetpos(stream, pos as *const _) }
}

pub fn ftello(stream: *mut FILE) -> off_t {
    extern { fn ftello(stream: *mut FILE) -> off_t; }
    unsafe { ftello(stream) }
}

pub fn ftell(stream: *mut FILE) -> long_t {
    extern { fn ftell(stream: *mut FILE) -> long_t; }
    unsafe { ftell(stream) }
}

pub fn ftrylockfile(stream: *mut FILE) -> int_t {
    extern { fn ftrylockfile(stream: *mut FILE) -> int_t; }
    unsafe { ftrylockfile(stream) }
}

pub fn funlockfile(stream: *mut FILE) {
    extern { fn funlockfile(stream: *mut FILE); }
    unsafe { funlockfile(stream); }
}

extern {
    pub fn fwrite(ptr: *const void_t, size: size_t, n: size_t, s: *mut FILE) -> size_t;
}

pub fn getchar() -> int_t {
    extern { fn getchar() -> int_t; }
    unsafe { getchar() }
}

pub fn getchar_unlocked() -> int_t {
    extern { fn getchar_unlocked() -> int_t; }
    unsafe { getchar_unlocked() }
}

pub fn getc(stream: *mut FILE) -> int_t {
    extern { fn getc(stream: *mut FILE) -> int_t; }
    unsafe { getc(stream) }
}

pub fn getc_unlocked(stream: *mut FILE) -> int_t {
    extern { fn getc_unlocked(stream: *mut FILE) -> int_t; }
    unsafe { getc_unlocked(stream) }
}

extern {
    pub fn getdelim(lineptr: *mut *mut char_t, n: *mut size_t, delimiter: int_t,
                    stream: *mut FILE) -> ssize_t;
    pub fn getline(lineptr: *mut *mut char_t, n: *mut size_t, stream: *mut FILE) -> ssize_t;
    pub fn gets(s: *mut char_t) -> *mut char_t;
}

pub fn getw(stream: *mut FILE) -> int_t {
    extern { fn getw(stream: *mut FILE) -> int_t; }
    unsafe { getw(stream) }
}

extern {
    pub fn open_memstream(bufloc: *mut *mut char_t, sizeloc: *mut size_t) -> *mut FILE;
}

pub fn pclose(stream: *mut FILE) -> int_t {
    extern { fn pclose(stream: *mut FILE) -> int_t; }
    unsafe { pclose(stream) }
}

pub fn perror<T: NTStr>(s: &T) {
    extern { fn perror(s: *const char_t); }
    unsafe { perror(s.as_ptr()); }
}

pub fn popen<T: NTStr, U: NTStr>(command: &T, modes: &U) -> *mut FILE {
    extern { fn popen(command: *const char_t, modes: *const char_t) -> *mut FILE; }
    unsafe { popen(command.as_ptr(), modes.as_ptr()) }
}

extern {
    pub fn printf(format: *const char_t, ...) -> int_t;
}

pub fn putc(c: int_t, stream: *mut FILE) -> int_t {
    extern { fn putc(c: int_t, stream: *mut FILE) -> int_t; }
    unsafe { putc(c, stream) }
}

pub fn putchar(c: int_t) -> int_t {
    extern { fn putchar(c: int_t) -> int_t; }
    unsafe { putchar(c) }
}

pub fn putchar_unlocked(c: int_t) -> int_t {
    extern { fn putchar_unlocked(c: int_t) -> int_t; }
    unsafe { putchar_unlocked(c) }
}

pub fn putc_unlocked(c: int_t, stream: *mut FILE) -> int_t {
    extern { fn putc_unlocked(c: int_t, stream: *mut FILE) -> int_t; }
    unsafe { putc_unlocked(c, stream) }
}

pub fn puts<T: NTStr>(s: &T) -> int_t {
    extern { fn puts(s: *const char_t) -> int_t; }
    unsafe { puts(s.as_ptr()) }
}

pub fn putw(w: int_t, stream: *mut FILE) -> int_t {
    extern { fn putw(w: int_t, stream: *mut FILE) -> int_t; }
    unsafe { putw(w, stream) }
}

pub fn remove<T: NTStr>(filename: &T) -> int_t {
    extern { fn remove(filename: *const char_t) -> int_t; }
    unsafe { remove(filename.as_ptr()) }
}

pub fn renameat<T: NTStr, U: NTStr>(oldfd: int_t, old: &T, newfd: int_t,
                                    new: &T) -> int_t {
    extern { fn renameat(oldfd: int_t, old: *const char_t, newfd: int_t,
                         new: *const char_t) -> int_t; }
    unsafe { renameat(oldfd, old.as_ptr(), newfd, new.as_ptr()) }
}

pub fn rename<T: NTStr, U: NTStr>(old: &T, new: &U) -> int_t {
    extern { fn rename(old: *const char_t, new: *const char_t) -> int_t; }
    unsafe { rename(old.as_ptr(), new.as_ptr()) }
}

pub fn rewind(stream: *mut FILE) {
    extern { fn rewind(stream: *mut FILE); }
    unsafe { rewind(stream); }
}

extern {
    pub fn scanf(format: *const char_t, ...) -> int_t;
}

extern {
    pub fn setbuf(stream: *mut FILE, buf: *mut char_t);
    pub fn setvbuf(stream: *mut FILE, buf: *mut char_t, modes: int_t, n: size_t) -> int_t;
    pub fn snprintf(s: *mut char_t, maxlen: size_t, format: *const char_t, ...) -> int_t;
    pub fn sprintf(s: *mut char_t, format: *const char_t, ...) -> int_t;
    pub fn sscanf(s: *const char_t, format: *const char_t, ...) -> int_t;
    pub fn tempnam(dir: *const char_t, pfx: *const char_t) -> *mut char_t;
}

pub fn tmpfile() -> *mut FILE {
    extern { fn tmpfile() -> *mut FILE; }
    unsafe { tmpfile() }
}

extern {
    pub fn tmpnam(s: *mut char_t) -> *mut char_t;
}

pub fn ungetc(c: int_t, stream: *mut FILE) -> int_t {
    extern { fn ungetc(c: int_t, stream: *mut FILE) -> int_t; }
    unsafe { ungetc(c, stream) }
}
