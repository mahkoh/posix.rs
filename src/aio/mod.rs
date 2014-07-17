pub use self::os::{aiocb};
pub use self::os::{AIO_ALLDONE};
pub use self::os::{AIO_CANCELED};
pub use self::os::{AIO_NOTCANCELED};
pub use self::os::{LIO_NOP};
pub use self::os::{LIO_NOWAIT};
pub use self::os::{LIO_READ};
pub use self::os::{LIO_WAIT};
pub use self::os::{LIO_WRITE};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn aio_read(aiocbp: &mut aiocb) -> ::int_t {
    extern { fn aio_read(aiocbp: *mut aiocb) -> ::int_t; }
    unsafe { aio_read(aiocbp as *mut _) }
}

pub fn aio_write(aiocbp: &mut aiocb) -> ::int_t {
    extern { fn aio_write(aiocbp: *mut aiocb) -> ::int_t; }
    unsafe { aio_write(aiocbp as *mut _) }
}

pub fn aio_error(aiocbp: &aiocb) -> ::int_t {
    extern { fn aio_error(aiocbp: *const aiocb) -> ::int_t; }
    unsafe { aio_error(aiocbp as *const _) }
}

pub fn lio_listio(mode: ::int_t, list: &[&mut aiocb],
                  sevp: &mut ::signal::sigevent) -> ::int_t {
    extern {
        fn lio_listio(mode: ::int_t, list: *const *mut aiocb,
                      nent: ::int_t, sig: *mut ::signal::sigevent) -> ::int_t;
    }
    unsafe {
        lio_listio(mode, list.as_ptr() as *const _, list.len() as ::int_t, sevp as *mut _)
    }
}

pub fn aio_return(aiocbp: &mut aiocb) -> ::ssize_t {
    extern { fn aio_return(aiocbp: *mut aiocb) -> ::ssize_t; }
    unsafe { aio_return(aiocbp as *mut _) }
}

pub fn aio_cancel(fd: ::int_t, aiocb: &mut aiocb) -> ::int_t {
    extern { fn aio_cancel(fd: ::int_t, aiocbp: *mut aiocb) -> ::int_t; }
    unsafe { aio_cancel(fd, aiocb as *mut _) }
}

pub fn aio_suspend(list: &[&aiocb], timeout: &::time::timespec) -> ::int_t {
    extern {
        fn aio_suspend(list: *const *const aiocb, nent: ::int_t,
                       timeout: *const ::time::timespec) -> ::int_t;
    }
    unsafe { aio_suspend(list.as_ptr() as *const _, list.len() as ::int_t,
                         timeout as *const _) }
}

pub fn aio_fsync(op: ::int_t, aiocb: &mut aiocb) -> ::int_t {
    extern { fn aio_fsync(op: ::int_t, aiocb: *mut aiocb) -> ::int_t; }
    unsafe { aio_fsync(op, aiocb as *mut _) }
}
