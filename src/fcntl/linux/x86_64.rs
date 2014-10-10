#[repr(C)]
pub struct flock {
    pub l_type: ::short_t,
    pub l_whence: ::short_t,
    pub l_start: ::sys::types::off_t,
    pub l_len: ::sys::types::off_t,
    pub l_pid: ::sys::types::pid_t,
}

pub const F_DUPFD:               ::int_t = 0;
pub const F_DUPFD_CLOEXEC:       ::int_t = 1030;
pub const F_GETFD:               ::int_t = 1;
pub const F_SETFD:               ::int_t = 2;
pub const F_GETFL:               ::int_t = 3;
pub const F_SETFL:               ::int_t = 4;
pub const F_GETLK:               ::int_t = 5;
pub const F_SETLK:               ::int_t = 6;
pub const F_SETLKW:              ::int_t = 7;
pub const F_GETOWN:              ::int_t = 9;
pub const F_SETOWN:              ::int_t = 8;
pub const FD_CLOEXEC:            ::int_t = 1;
pub const F_RDLCK:               ::int_t = 0;
pub const F_UNLCK:               ::int_t = 2;
pub const F_WRLCK:               ::int_t = 1;
pub const O_CLOEXEC:             ::int_t = 524288;
pub const O_CREAT:               ::int_t = 64;
pub const O_DIRECTORY:           ::int_t = 65536;
pub const O_EXCL:                ::int_t = 128;
pub const O_NOCTTY:              ::int_t = 256;
pub const O_NOFOLLOW:            ::int_t = 131072;
pub const O_TRUNC:               ::int_t = 512;
pub const O_APPEND:              ::int_t = 1024;
pub const O_DSYNC:               ::int_t = 4096;
pub const O_NONBLOCK:            ::int_t = 2048;
pub const O_RSYNC:               ::int_t = 1052672;
pub const O_SYNC:                ::int_t = 1052672;
pub const O_ACCMODE:             ::int_t = 3;
pub const O_RDONLY:              ::int_t = 0;
pub const O_RDWR:                ::int_t = 2;
pub const O_WRONLY:              ::int_t = 1;
pub const AT_FDCWD:              ::int_t = -100;
pub const AT_EACCESS:            ::int_t = 512;
pub const AT_SYMLINK_NOFOLLOW:   ::int_t = 256;
pub const AT_SYMLINK_FOLLOW:     ::int_t = 1024;
pub const AT_REMOVEDIR:          ::int_t = 512;
pub const POSIX_FADV_DONTNEED:   ::int_t = 4;
pub const POSIX_FADV_NOREUSE:    ::int_t = 5;
pub const POSIX_FADV_NORMAL:     ::int_t = 0;
pub const POSIX_FADV_RANDOM:     ::int_t = 1;
pub const POSIX_FADV_SEQUENTIAL: ::int_t = 2;
pub const POSIX_FADV_WILLNEED:   ::int_t = 3;
