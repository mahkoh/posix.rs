use {char_t, int_t, long_t};

pub use self::os::{dirent};
pub use self::os::{DIR};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn closedir(dirp: *mut DIR) -> int_t {
    extern { fn closedir(dirp: *mut DIR) -> int_t; }
    unsafe { closedir(dirp as *mut DIR) }
}

pub fn dirfd(dirp: *mut DIR) -> int_t {
    extern { fn dirfd(dirp: *mut DIR) -> int_t; }
    unsafe { dirfd(dirp as *mut DIR) }
}

pub fn fdopendir(fd: int_t) -> *mut DIR {
    extern { fn fdopendir(fd: int_t) -> *mut DIR; }
    unsafe { fdopendir(fd) }
}

pub fn opendir<T: ::NTStr>(name: &T) -> *mut DIR {
    extern { fn opendir(name: *const char_t) -> *mut DIR; }
    unsafe { opendir(name.as_ptr()) }
}

pub fn readdir_r(dirp: *mut DIR, entry: &mut dirent, result: &mut usize) -> int_t {
    extern { fn readdir_r(dirp: *mut DIR, entry: *mut dirent,
                          result: *mut *mut dirent) -> int_t; }
    unsafe { readdir_r(dirp, entry as *mut _, result as *mut _ as *mut _) }
}

pub fn rewinddir(dirp: *mut DIR) {
    extern { fn rewinddir(dirp: *mut DIR); }
    unsafe { rewinddir(dirp) }
}

pub fn seekdir(dirp: *mut DIR, pos: long_t) {
    extern { fn seekdir(dirp: *mut DIR, pos: long_t); }
    unsafe { seekdir(dirp, pos) }
}

pub fn telldir(dirp: *mut DIR) -> long_t {
    extern { fn telldir(dirp: *mut DIR) -> long_t; }
    unsafe { telldir(dirp) }
}

extern {
    pub fn alphasort(e1: *mut *const dirent, e2: *mut *const dirent) -> int_t;
    pub fn readdir(dirp: *mut DIR) -> *mut dirent;
    pub fn scandir(dir: *const char_t,
                   namelist: *mut *mut *mut dirent,
                   selector: Option<extern "C" fn (arg1: *const dirent) -> int_t>,
                   cmp: Option<extern "C" fn (arg1: *mut *const dirent, arg2: *mut *const dirent) -> int_t>) -> int_t;
}

#[cfg(test)]
mod tests {
    use {ToNTStr};
    use std::io::{TempDir};

    #[test]
    fn test() {
        let tmp = TempDir::new("").unwrap();
        let mut tmp_path = tmp.path().clone();
        tmp_path.push("xxx");
        for i in [0u, 1, 2].iter() {
            tmp_path.set_filename(format!("{}", i));
            ::std::io::fs::File::create(&tmp_path).unwrap();
        }
        let path = tmp.path().to_nt_str();
        let dirp = super::opendir(&path);
        let mut ent = super::dirent::new();
        let mut res = 1;
        for _ in range(0u, 2) {
            for (i, _) in [".", "..", "0", "1", "2", "3"].iter().enumerate() {
                assert!(super::readdir_r(dirp, &mut ent, &mut res) != -1);
                if res == 0 {
                    assert_eq!(i, 5);
                    break;
                }
                assert_eq!(ent.d_off, i as ::sys::types::off_t + 1);
                assert_eq!(super::telldir(dirp), i as ::long_t + 1);
            }
            assert_eq!(res, 0);
            super::rewinddir(dirp);
        }
        super::closedir(dirp);
    }
}
