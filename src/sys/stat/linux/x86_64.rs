#[repr(C)]
#[deriving(Copy)]
pub struct stat {
    pub st_dev:           ::sys::types::dev_t,
    pub st_ino:           ::sys::types::ino_t,
    pub st_nlink:         ::sys::types::nlink_t,
    pub st_mode:          ::sys::types::mode_t,
    pub st_uid:           ::sys::types::uid_t,
    pub st_gid:           ::sys::types::gid_t,
    __pad0:           ::int_t,
    pub st_rdev:          ::sys::types::dev_t,
    pub st_size:          ::sys::types::off_t,
    pub st_blksize:       ::sys::types::blksize_t,
    pub st_blocks:        ::sys::types::blkcnt_t,
    pub st_atim:          ::time::timespec,
    pub st_mtim:          ::time::timespec,
    pub st_ctim:          ::time::timespec,
    __glibc_reserved: [::long_t, ..3u],
}

new!(stat);

pub const S_IFMT:   ::sys::types::mode_t = 61440;
pub const S_IFBLK:  ::sys::types::mode_t = 24576;
pub const S_IFCHR:  ::sys::types::mode_t = 8192;
pub const S_IFIFO:  ::sys::types::mode_t = 4096;
pub const S_IFREG:  ::sys::types::mode_t = 32768;
pub const S_IFDIR:  ::sys::types::mode_t = 16384;
pub const S_IFLNK:  ::sys::types::mode_t = 40960;
pub const S_IFSOCK: ::sys::types::mode_t = 49152;

pub fn S_ISBLK(m: ::sys::types::mode_t) -> bool {
    m & S_IFMT == S_IFBLK
}

pub fn S_ISCHR(m: ::sys::types::mode_t) -> bool {
    m & S_IFMT == S_IFCHR
}

pub fn S_ISDIR(m: ::sys::types::mode_t) -> bool {
    m & S_IFMT == S_IFDIR
}

pub fn S_ISFIFO(m: ::sys::types::mode_t) -> bool {
    m & S_IFMT == S_IFIFO
}

pub fn S_ISREG(m: ::sys::types::mode_t) -> bool {
    m & S_IFMT == S_IFREG
}

pub fn S_ISLNK(m: ::sys::types::mode_t) -> bool {
    m & S_IFMT == S_IFLNK
}

pub fn S_ISSOCK(m: ::sys::types::mode_t) -> bool {
    m & S_IFMT == S_IFSOCK
}

pub fn S_TYPEISMQ(_: &stat) -> bool {
    false
}

pub fn S_TYPEISSEM(_: &stat) -> bool {
    false
}

pub fn S_TYPEISSHM(_: &stat) -> bool {
    false
}

pub const UTIME_NOW: ::int_t = (1 << 30) - 1;
pub const UTIME_OMIT: ::int_t = (1 << 30) - 2;
