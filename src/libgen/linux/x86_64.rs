pub unsafe fn basename(path: *mut ::char_t) -> *mut ::char_t {
    extern { fn __xpg_basename(path: *mut ::char_t) -> *mut ::char_t; }
    __xpg_basename(path)
}
