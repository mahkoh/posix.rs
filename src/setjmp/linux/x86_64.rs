#[repr(C)]
pub struct jmp_buf {
    _data: [u64, ..25]
}

#[repr(C)]
pub struct sigjmp_buf {
    _data: [u64, ..25]
}

new!(jmp_buf)
new!(sigjmp_buf)

pub unsafe fn sigsetjmp(env: *mut jmp_buf, savemask: ::int_t) -> ::int_t {
    extern { fn __sigsetjmp(env: *mut jmp_buf, savemask: ::int_t) -> ::int_t; }
    __sigsetjmp(env, savemask)
}
