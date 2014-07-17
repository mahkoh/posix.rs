pub use self::os::{dirent};
pub use self::os::{DIR};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn closedir(dirp: *mut DIR) -> ::int_t {
    extern { fn closedir(dirp: *mut DIR) -> ::int_t; }
    unsafe { closedir(dirp as *mut DIR) }
}

pub fn dirfd(dirp: *mut DIR) -> ::int_t {
    extern { fn dirfd(dirp: *mut DIR) -> ::int_t; }
    unsafe { dirfd(dirp as *mut DIR) }
}

pub fn fdopendir(fd: ::int_t) -> *mut DIR {
    extern { fn fdopendir(fd: ::int_t) -> *mut DIR; }
    unsafe { fdopendir(fd) }
}

pub fn opendir<T: ::NTStr>(name: &T) -> *mut DIR {
    extern { fn opendir(name: *const ::char_t) -> *mut DIR; }
    unsafe { opendir(name.as_ptr()) }
}

pub fn readdir_r(dirp: *mut DIR, entry: &mut dirent, result: *mut *mut dirent) -> ::int_t {
    extern { fn readdir_r(dirp: *mut DIR, entry: *mut dirent,
                          result: *mut *mut dirent) -> ::int_t; }
    unsafe { readdir_r(dirp, entry as *mut _, result) }
}

pub fn rewinddir(dirp: *mut DIR) {
    extern { fn rewinddir(dirp: *mut DIR); }
    unsafe { rewinddir(dirp) }
}

pub fn seekdir(dirp: *mut DIR, pos: ::long_t) {
    extern { fn seekdir(dirp: *mut DIR, pos: ::long_t); }
    unsafe { seekdir(dirp, pos) }
}

pub fn telldir(dirp: *mut DIR) -> ::long_t {
    extern { fn telldir(dirp: *mut DIR) -> ::long_t; }
    unsafe { telldir(dirp) }
}

extern "C" {
    pub fn alphasort(e1: *mut *const dirent, e2: *mut *const dirent) -> ::int_t;
    pub fn readdir(dirp: *mut DIR) -> *mut dirent;
    pub fn scandir(dir: *const ::char_t,
                   namelist: *mut *mut *mut dirent,
                   selector: ::std::option::Option<extern "C" fn (arg1: *const dirent) -> ::int_t>,
                   cmp: ::std::option::Option<extern "C" fn (arg1: *mut *const dirent, arg2: *mut *const dirent) -> ::int_t>) -> ::int_t;
}
