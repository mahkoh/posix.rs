pub type jmp_buf    = [u64, ..25];
pub type sigjmp_buf = [u64, ..25];

pub unsafe fn sigsetjmp(env: *mut jmp_buf, savemask: ::int_t) -> ::int_t {
    extern { fn __sigsetjmp(env: *mut jmp_buf, savemask: ::int_t) -> ::int_t; }
    __sigsetjmp(env, savemask)
}
