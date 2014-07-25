#[repr(C)]
pub struct aiocb {
    pub aio_fildes: ::int_t,
    pub aio_lio_opcode: ::int_t,
    pub aio_reqprio: ::int_t,
    pub aio_buf: *mut ::void_t,
    pub aio_nbytes: ::size_t,
    pub aio_sigevent: ::signal::sigevent,
    __next_prio: [u32, ..1],
    __abs_prio: [u32, ..1],
    __policy: [u32, ..1],
    __error_code: [u32, ..1],
    __return_value: [u32, ..1],
    pub aio_offset: ::sys::types::off_t,
    __pad: [u8, ..4],
    __glibc_reserved: [u8, ..32],
}
new!(aiocb)
pub static AIO_CANCELED: ::uint_t = 0;
pub static AIO_NOTCANCELED: ::uint_t = 1;
pub static AIO_ALLDONE: ::uint_t = 2;
pub static LIO_READ: ::uint_t = 0;
pub static LIO_WRITE: ::uint_t = 1;
pub static LIO_NOP: ::uint_t = 2;
pub static LIO_WAIT: ::uint_t = 0;
pub static LIO_NOWAIT: ::uint_t = 1;
