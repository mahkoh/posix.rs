pub use self::os::{if_nameindex};
pub use self::os::{IF_NAMESIZE};

use {NTStr, uint_t, char_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn if_nametoindex<T: NTStr>(ifname: &T) -> uint_t {
    extern { fn if_nametoindex(ifname: *const char_t) -> uint_t; }
    unsafe { if_nametoindex(ifname.as_ptr()) }
}

pub fn if_indextoname(ifindex: uint_t, dst: &mut [u8]) -> *mut char_t {
    extern { fn if_indextoname(ifindex: uint_t, ifname: *mut char_t) -> *mut char_t; }
    if dst.len() < IF_NAMESIZE as uint {
        0 as *mut _
    } else {
        unsafe { if_indextoname(ifindex, dst.as_mut_ptr() as *mut _) }
    }
}

pub fn if_nameindex() -> *mut if_nameindex {
    extern { fn if_nameindex() -> *mut if_nameindex; }
    unsafe { if_nameindex() }
}

pub fn if_freenameindex(ptr: *mut if_nameindex) {
    extern { fn if_freenameindex(ptr: *mut if_nameindex); }
    unsafe { if_freenameindex(ptr) }
}
