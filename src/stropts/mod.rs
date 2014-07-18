pub use self::os::{t_scalar_t};
pub use self::os::{t_uscalar_t};
pub use self::os::{bandinfo};
pub use self::os::{strbuf};
pub use self::os::{strpeek};
pub use self::os::{strfdinsert};
pub use self::os::{strioctl};
pub use self::os::{strrecvfd};
pub use self::os::{str_mlist};
pub use self::os::{str_list};
pub use self::os::{I_ATMARK};
pub use self::os::{I_CANPUT};
pub use self::os::{I_CKBAND};
pub use self::os::{I_FDINSERT};
pub use self::os::{I_FIND};
pub use self::os::{I_FLUSH};
pub use self::os::{I_FLUSHBAND};
pub use self::os::{I_GETBAND};
pub use self::os::{I_GETCLTIME};
pub use self::os::{I_GETSIG};
pub use self::os::{I_GRDOPT};
pub use self::os::{I_GWROPT};
pub use self::os::{I_LINK};
pub use self::os::{I_LIST};
pub use self::os::{I_LOOK};
pub use self::os::{I_NREAD};
pub use self::os::{I_PEEK};
pub use self::os::{I_PLINK};
pub use self::os::{I_POP};
pub use self::os::{I_PUNLINK};
pub use self::os::{I_PUSH};
pub use self::os::{I_RECVFD};
pub use self::os::{I_SENDFD};
pub use self::os::{I_SETCLTIME};
pub use self::os::{I_SETSIG};
pub use self::os::{I_SRDOPT};
pub use self::os::{I_STR};
pub use self::os::{I_SWROPT};
pub use self::os::{I_UNLINK};
pub use self::os::{FMNAMESZ};
pub use self::os::{FLUSHR};
pub use self::os::{FLUSHRW};
pub use self::os::{FLUSHW};
pub use self::os::{S_BANDURG};
pub use self::os::{S_ERROR};
pub use self::os::{S_HANGUP};
pub use self::os::{S_HIPRI};
pub use self::os::{S_INPUT};
pub use self::os::{S_MSG};
pub use self::os::{S_OUTPUT};
pub use self::os::{S_RDBAND};
pub use self::os::{S_RDNORM};
pub use self::os::{S_WRBAND};
pub use self::os::{S_WRNORM};
pub use self::os::{RS_HIPRI};
pub use self::os::{RMSGD};
pub use self::os::{RMSGN};
pub use self::os::{RNORM};
pub use self::os::{RPROTDAT};
pub use self::os::{RPROTDIS};
pub use self::os::{RPROTNORM};
pub use self::os::{SNDZERO};
pub use self::os::{ANYMARK};
pub use self::os::{LASTMARK};
pub use self::os::{MORECTL};
pub use self::os::{MOREDATA};
pub use self::os::{MSG_ANY};
pub use self::os::{MSG_BAND};
pub use self::os::{MSG_HIPRI};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn fattach<T: ::NTStr>(filedes: ::int_t, path: &T) -> ::int_t {
    extern { fn fattach(fildes: ::int_t, path: *const ::char_t) -> ::int_t; }
    unsafe { fattach(filedes, path.as_ptr()) }
}

pub fn fdetach<T: ::NTStr>(path: &T) -> ::int_t {
    extern { fn fdetach(path: *const ::char_t) -> ::int_t; }
    unsafe { fdetach(path.as_ptr()) }
}

pub fn getmsg(fildes: ::int_t, ctlptr: &mut strbuf, dataptr: &mut strbuf,
              flagsp: &mut ::int_t) -> ::int_t {
    extern { fn getmsg(fildes: ::int_t, ctlptr: *mut strbuf,
                       dataptr: *mut strbuf, flagsp: *mut ::int_t) -> ::int_t; }
    unsafe { getmsg(fildes, ctlptr as *mut _, dataptr as *mut _, flagsp as *mut _) }
}

pub fn getpmsg(fildes: ::int_t, ctlptr: &mut strbuf, dataptr: &mut strbuf,
               bandp: &mut ::int_t, flagsp: &mut ::int_t) -> ::int_t {
    extern { fn getpmsg(fildes: ::int_t, ctlptr: *mut strbuf,
                        dataptr: *mut strbuf, bandp: *mut ::int_t,
                        flagsp: *mut ::int_t) -> ::int_t; }
    unsafe { getpmsg(fildes, ctlptr as *mut _, dataptr as *mut _, bandp as *mut _,
                     flagsp as *mut _) }
}

pub fn isastream(fildes: ::int_t) -> ::int_t {
    extern { fn isastream(fildes: ::int_t) -> ::int_t; }
    unsafe { isastream(fildes) }
}

pub fn putmsg(fildes: ::int_t, ctlptr: &strbuf, dataptr: &strbuf,
              flags: ::int_t) -> ::int_t {
    extern { fn putmsg(fildes: ::int_t, ctlptr: *const strbuf,
                       dataptr: *const strbuf, flags: ::int_t) -> ::int_t; }
    unsafe { putmsg(fildes, ctlptr as *const _, dataptr as *const _, flags) }
}

pub fn putpmsg(fildes: ::int_t, ctlptr: &strbuf, dataptr: &strbuf, band: ::int_t,
               flags: ::int_t) -> ::int_t {
    extern { fn putpmsg(fildes: ::int_t, ctlptr: *const strbuf,
                        dataptr: *const strbuf, band: ::int_t,
                        flags: ::int_t) -> ::int_t; }
    unsafe { putpmsg(fildes, ctlptr as *const _, dataptr as *const _, band, flags) }
}

extern "C" {
    pub fn ioctl(fd: ::int_t, request: ::ulong_t, ...) -> ::int_t;
}
