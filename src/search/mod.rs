pub use self::os::{entry};

use {void_t, int_t, size_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub type ENTRY = entry;

#[repr(C)]
#[deriving(Copy)]
pub enum ACTION {
    FIND,
    ENTER,
}

#[repr(C)]
#[deriving(Copy)]
pub enum VISIT {
    preorder,
    postorder,
    endorder,
    leaf,
}

pub type compar_fn_t = extern fn (*const void_t, *const void_t) -> int_t;
pub type action_fn_t = extern fn (*const void_t, VISIT, int_t);

extern {
    pub fn hcreate(nel: size_t) -> int_t;
    pub fn hdestroy();
    pub fn hsearch(item: ENTRY, action: ACTION) -> *mut ENTRY;
    pub fn insque(elem: *mut void_t, prev: *mut void_t);
    pub fn lfind(key: *const void_t, base: *const void_t, nmemb: *mut size_t,
                 size: size_t, compar: compar_fn_t) -> *mut void_t;
    pub fn lsearch(key: *const void_t, base: *mut void_t, nmemb: *mut size_t,
                   size: size_t, compar: compar_fn_t) -> *mut void_t;
    pub fn remque(elem: *mut void_t);
    pub fn tdelete(key: *const void_t, rootp: *mut *mut void_t,
                   compar: compar_fn_t) -> *mut void_t;
    pub fn tfind(key: *const void_t, rootp: *const *mut void_t,
                 compar: compar_fn_t) -> *mut void_t;
    pub fn tsearch(key: *const void_t, rootp: *mut *mut void_t,
                   compar: compar_fn_t) -> *mut void_t;
    pub fn twalk(root: *const void_t, action: action_fn_t);
}
