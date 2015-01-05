pub type jmp_buf = [[u32; 39]; 1];
pub type sigjmp_buf = [[u32; 39]; 1];

pub unsafe fn sigsetjmp(env: *mut jmp_buf, savemask: ::int_t) -> ::int_t {
    extern { fn __sigsetjmp(env: *mut jmp_buf, savemask: ::int_t) -> ::int_t; }
    __sigsetjmp(env, savemask)
}
