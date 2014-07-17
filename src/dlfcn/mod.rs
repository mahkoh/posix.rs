pub use self::os::{RTLD_LAZY};
pub use self::os::{RTLD_NOW};
pub use self::os::{RTLD_GLOBAL};
pub use self::os::{RTLD_LOCAL};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn dlopen<T: ::NTStr>(file: &T, mode: ::int_t) -> *mut ::void_t {
    extern { fn dlopen(file: *const ::char_t, mode: ::int_t) -> *mut ::void_t; }
    unsafe { dlopen(file.as_ptr(), mode) }
}

pub fn dlclose(handle: *mut ::void_t) -> ::int_t {
    extern { fn dlclose(handle: *mut ::void_t) -> ::int_t; }
    unsafe { dlclose(handle) }
}

pub fn dlsym<T: ::NTStr>(handle: *mut ::void_t, name: &T) -> *mut ::void_t {
    extern { fn dlsym(handle: *mut ::void_t, name: *const ::char_t) -> *mut ::void_t; }
    unsafe { dlsym(handle, name.as_ptr()) }
}

pub fn dlerror() -> *mut ::char_t {
    extern { fn dlerror() -> *mut ::char_t; }
    unsafe { dlerror() }
}
