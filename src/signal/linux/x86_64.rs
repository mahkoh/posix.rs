pub type sig_atomic_t = ::int_t;

#[repr(C)]
pub struct sigset_t {
    _data: [::ulong_t, ..16u],
}

new!(sigset_t)

#[repr(C)]
pub struct sigevent {
    pub sigev_value: sigval,
    pub sigev_signo: ::int_t,
    pub sigev_notify: ::int_t,
    pub sigev_notify_function: ::std::option::Option<extern fn(arg1: sigval)>,
    pub sigev_notify_attribute: *mut ::sys::types::pthread_attr_t,
    _pad: [u64, ..4u],
}

new!(sigevent)

#[repr(C)]
pub struct sigval {
    _data: [u64, ..1u],
}

new!(sigval)

impl sigval {
    pub fn sival_int(&self) -> &::int_t {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn sival_ptr(&self) -> &*mut ::void_t {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn sival_int_mut(&mut self) -> &mut ::int_t {
        unsafe { ::std::mem::transmute(self) }
    }

    pub fn sival_ptr_mut(&mut self) -> &mut *mut ::void_t {
        unsafe { ::std::mem::transmute(self) }
    }
}

#[repr(C)]
pub struct sigaction {
    hlr: [u64, ..1u],
    pub sa_mask: sigset_t,
    pub sa_flags: ::int_t,
    pub sa_restorer: ::std::option::Option<extern fn()>,
}

new!(sigaction)

impl sigaction {
    pub fn sa_handler(&self) -> &::std::option::Option<extern fn(arg1: ::int_t)> {
        unsafe { ::std::mem::transmute(&self.hlr) }
    }

    pub fn sa_sigaction(&self) ->
            &::std::option::Option<extern fn(arg1: ::int_t, arg2: *mut siginfo_t, arg3: *mut ::void_t)> {
        unsafe { ::std::mem::transmute(&self.hlr) }
    }

    pub fn sa_handler_mut(&mut self) ->
            &mut ::std::option::Option<extern fn(arg1: ::int_t)> {
        unsafe { ::std::mem::transmute(&self.hlr) }
    }

    pub fn sa_sigaction_mut(&mut self) ->
            &mut ::std::option::Option<extern fn (arg1: ::int_t, arg2: *mut siginfo_t, arg3: *mut ::void_t)> {
        unsafe { ::std::mem::transmute(&self.hlr) }
    }
}

#[repr(C)]
pub struct mcontext_t {
    _data: [u64, ..32],
}

new!(mcontext_t)

#[repr(C)]
pub struct ucontext {
    pub uc_flags: ::ulong_t,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    _data: [u64, ..64],
}

new!(ucontext)

#[repr(C)]
pub struct stack_t {
    pub ss_sp: *mut ::void_t,
    pub ss_flags: ::int_t,
    pub ss_size: ::size_t,
}

new!(stack_t)

#[repr(C)]
pub struct siginfo_t {
    pub si_signo: ::int_t,
    pub si_errno: ::int_t,
    pub si_code: ::int_t,
    _data: [u64, ..14u],
}

new!(siginfo_t)

impl siginfo_t {
    pub fn si_pid(&self) -> &::sys::types::pid_t {
        let tmp: &_rt = unsafe { ::std::mem::transmute(&self._data) };
        &tmp.si_pid
    }

    pub fn si_pid_mut(&mut self) -> &mut ::sys::types::pid_t {
        let tmp: &mut _rt = unsafe { ::std::mem::transmute(&self._data) };
        &mut tmp.si_pid
    }

    pub fn si_uid(&self) -> &::sys::types::uid_t {
        let tmp: &_rt = unsafe { ::std::mem::transmute(&self._data) };
        &tmp.si_uid
    }

    pub fn si_uid_mut(&mut self) -> &mut ::sys::types::uid_t {
        let tmp: &mut _rt = unsafe { ::std::mem::transmute(&self._data) };
        &mut tmp.si_uid
    }

    pub fn si_addr(&self) -> &*mut ::void_t {
        let tmp: &_sigfault = unsafe { ::std::mem::transmute(&self._data) };
        &tmp.si_addr
    }

    pub fn si_addr_mut(&mut self) -> &mut *mut ::void_t {
        let tmp: &mut _sigfault = unsafe { ::std::mem::transmute(&self._data) };
        &mut tmp.si_addr
    }

    pub fn si_status(&self) -> &::int_t {
        let tmp: &_sigchld = unsafe { ::std::mem::transmute(&self._data) };
        &tmp.si_status
    }

    pub fn si_status_mut(&mut self) -> &mut ::int_t {
        let tmp: &mut _sigchld = unsafe { ::std::mem::transmute(&self._data) };
        &mut tmp.si_status
    }

    pub fn si_band(&self) -> &::long_t {
        let tmp: &_sigpoll = unsafe { ::std::mem::transmute(&self._data) };
        &tmp.si_band
    }

    pub fn si_band_mut(&mut self) -> &mut ::long_t {
        let tmp: &mut _sigpoll = unsafe { ::std::mem::transmute(&self._data) };
        &mut tmp.si_band
    }

    pub fn si_value(&self) -> &sigval {
        let tmp: &_rt = unsafe { ::std::mem::transmute(&self._data) };
        &tmp.si_sigval
    }

    pub fn si_value_mut(&mut self) -> &mut sigval {
        let tmp: &mut _rt = unsafe { ::std::mem::transmute(&self._data) };
        &mut tmp.si_sigval
    }
}

#[repr(C)]
struct _rt {
    si_pid: ::sys::types::pid_t,
    si_uid: ::sys::types::uid_t,
    si_sigval: sigval,
}

#[repr(C)]
struct _sigchld {
    si_pid: ::sys::types::pid_t,
    si_uid: ::sys::types::uid_t,
    si_status: ::int_t,
}

#[repr(C)]
struct _sigfault {
    si_addr: *mut ::void_t,
    si_addr_lsb: ::short_t,
}

#[repr(C)]
struct _sigpoll {
    si_band: ::long_t,
}

pub fn SIG_DFL() -> extern fn(::int_t) {
    unsafe { ::std::mem::transmute::<uint,_>(0) }
}

pub fn SIG_ERR() -> extern fn(::int_t) {
    unsafe { ::std::mem::transmute::<uint,_>(-1) }
}

pub fn SIG_IGN() -> extern fn(::int_t) {
    unsafe { ::std::mem::transmute::<uint,_>(1) }
}

pub static SIGEV_NONE:    ::int_t = 1;
pub static SIGEV_SIGNAL:  ::int_t = 0;
pub static SIGEV_THREAD:  ::int_t = 2;
pub static SIGABRT:       ::int_t = 6;
pub static SIGALRM:       ::int_t = 14;
pub static SIGBUS:        ::int_t = 7;
pub static SIGCHLD:       ::int_t = 17;
pub static SIGCONT:       ::int_t = 18;
pub static SIGFPE:        ::int_t = 8;
pub static SIGHUP:        ::int_t = 1;
pub static SIGILL:        ::int_t = 4;
pub static SIGINT:        ::int_t = 2;
pub static SIGKILL:       ::int_t = 9;
pub static SIGPIPE:       ::int_t = 13;
pub static SIGQUIT:       ::int_t = 3;
pub static SIGSEGV:       ::int_t = 11;
pub static SIGSTOP:       ::int_t = 19;
pub static SIGTERM:       ::int_t = 15;
pub static SIGTSTP:       ::int_t = 20;
pub static SIGTTIN:       ::int_t = 21;
pub static SIGTTOU:       ::int_t = 22;
pub static SIGUSR1:       ::int_t = 10;
pub static SIGUSR2:       ::int_t = 12;
pub static SIGPOLL:       ::int_t = 29;
pub static SIGPROF:       ::int_t = 27;
pub static SIGSYS:        ::int_t = 31;
pub static SIGTRAP:       ::int_t = 5;
pub static SIGURG:        ::int_t = 23;
pub static SIGVTALRM:     ::int_t = 26;
pub static SIGXCPU:       ::int_t = 24;
pub static SIGXFSZ:       ::int_t = 25;
pub static SIG_BLOCK:     ::int_t = 0;
pub static SIG_UNBLOCK:   ::int_t = 1;
pub static SIG_SETMASK:   ::int_t = 2;
pub static SA_NOCLDSTOP:  ::int_t = 1;
pub static SA_ONSTACK:    ::int_t = 134217728;
pub static SA_RESETHAND:  ::int_t = -2147483648;
pub static SA_RESTART:    ::int_t = 268435456;
pub static SA_SIGINFO:    ::int_t = 4;
pub static SA_NOCLDWAIT:  ::int_t = 2;
pub static SA_NODEFER:    ::int_t = 1073741824;
pub static SS_ONSTACK:    ::int_t = 1;
pub static SS_DISABLE:    ::int_t = 2;
pub static MINSIGSTKSZ:   ::int_t = 2048;
pub static SIGSTKSZ:      ::int_t = 8192;
pub static ILL_ILLOPC:    ::int_t = 1;
pub static ILL_ILLOPN:    ::int_t = 2;
pub static ILL_ILLADR:    ::int_t = 3;
pub static ILL_ILLTRP:    ::int_t = 4;
pub static ILL_PRVOPC:    ::int_t = 5;
pub static ILL_PRVREG:    ::int_t = 6;
pub static ILL_COPROC:    ::int_t = 7;
pub static ILL_BADSTK:    ::int_t = 8;
pub static FPE_INTDIV:    ::int_t = 1;
pub static FPE_INTOVF:    ::int_t = 2;
pub static FPE_FLTDIV:    ::int_t = 3;
pub static FPE_FLTOVF:    ::int_t = 4;
pub static FPE_FLTUND:    ::int_t = 5;
pub static FPE_FLTRES:    ::int_t = 6;
pub static FPE_FLTINV:    ::int_t = 7;
pub static FPE_FLTSUB:    ::int_t = 8;
pub static SEGV_MAPERR:   ::int_t = 1;
pub static SEGV_ACCERR:   ::int_t = 2;
pub static BUS_ADRALN:    ::int_t = 1;
pub static BUS_ADRERR:    ::int_t = 2;
pub static BUS_OBJERR:    ::int_t = 3;
pub static TRAP_BRKPT:    ::int_t = 1;
pub static TRAP_TRACE:    ::int_t = 2;
pub static CLD_EXITED:    ::int_t = 1;
pub static CLD_KILLED:    ::int_t = 2;
pub static CLD_DUMPED:    ::int_t = 3;
pub static CLD_TRAPPED:   ::int_t = 4;
pub static CLD_STOPPED:   ::int_t = 5;
pub static CLD_CONTINUED: ::int_t = 6;
pub static POLL_IN:       ::int_t = 1;
pub static POLL_OUT:      ::int_t = 2;
pub static POLL_MSG:      ::int_t = 3;
pub static POLL_ERR:      ::int_t = 4;
pub static POLL_PRI:      ::int_t = 5;
pub static POLL_HUP:      ::int_t = 6;
pub static SI_USER:       ::int_t = 0;
pub static SI_QUEUE:      ::int_t = -1;
pub static SI_TIMER:      ::int_t = -2;
pub static SI_ASYNCIO:    ::int_t = -4;
pub static SI_MESGQ:      ::int_t = -3;
