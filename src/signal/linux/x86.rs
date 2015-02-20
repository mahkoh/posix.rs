pub type sig_atomic_t = ::int_t;
pub type sigset_t = [u32; 32];
#[repr(C)]
#[derive(Copy)]
pub struct siginfo_t {
    pub si_signo: ::int_t,
    pub si_errno: ::int_t,
    pub si_code: ::int_t,
    _sifields: [u32; 29],
}
new!(siginfo_t);
#[repr(C)]
#[derive(Copy)]
pub struct sigval {
    _data: [u32; 1usize],
}
new!(sigval);
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
#[derive(Copy)]
pub struct sigevent {
    pub sigev_value: sigval,
    pub sigev_signo: ::int_t,
    pub sigev_notify: ::int_t,
    pub sigev_notify_function: ::std::option::Option<extern fn(arg1: sigval)>,
    pub sigev_notify_attribute: *mut ::sys::types::pthread_attr_t,
    _pad: [u32; 11],
}
new!(sigevent);
#[repr(C)]
#[derive(Copy)]
pub struct sigaction {
    __sigaction_handler: [u32; 1],
    pub sa_mask: sigset_t,
    pub sa_flags: ::int_t,
    pub sa_restorer: fn(),
}
new!(sigaction);
#[repr(C)]
#[derive(Copy)]
pub struct stack_t {
    pub ss_sp: *mut ::void_t,
    pub ss_flags: ::int_t,
    pub ss_size: ::size_t,
}
new!(stack_t);
#[repr(C)]
#[derive(Copy)]
pub struct mcontext_t {
    pub gregs: [::int_t; 19],
    pub fpregs: [u32; 28],
    pub oldmask: ::ulong_t,
    pub cr2: ::ulong_t,
}
new!(mcontext_t);
#[repr(C)]
#[derive(Copy)]
pub struct ucontext {
    pub uc_flags: ::ulong_t,
    pub uc_link: *mut ucontext,
    pub uc_stack: stack_t,
    pub uc_mcontext: mcontext_t,
    pub uc_sigmask: sigset_t,
    __fpregs_mem: [u32; 28],
}
new!(ucontext);
pub fn SIG_DFL() -> extern fn(::int_t) {
    unsafe { ::std::mem::transmute::<uint,_>(0) }
}

pub fn SIG_ERR() -> extern fn(::int_t) {
    unsafe { ::std::mem::transmute::<uint,_>(-1) }
}

pub fn SIG_IGN() -> extern fn(::int_t) {
    unsafe { ::std::mem::transmute::<uint,_>(1) }
}
pub const SIGEV_NONE: ::int_t = 1;
pub const SIGEV_SIGNAL: ::int_t = 0;
pub const SIGEV_THREAD: ::int_t = 2;
pub const SIGABRT: ::int_t = 6;
pub const SIGALRM: ::int_t = 14;
pub const SIGBUS: ::int_t = 7;
pub const SIGCHLD: ::int_t = 17;
pub const SIGCONT: ::int_t = 18;
pub const SIGFPE: ::int_t = 8;
pub const SIGHUP: ::int_t = 1;
pub const SIGILL: ::int_t = 4;
pub const SIGINT: ::int_t = 2;
pub const SIGKILL: ::int_t = 9;
pub const SIGPIPE: ::int_t = 13;
pub const SIGQUIT: ::int_t = 3;
pub const SIGSEGV: ::int_t = 11;
pub const SIGSTOP: ::int_t = 19;
pub const SIGTERM: ::int_t = 15;
pub const SIGTSTP: ::int_t = 20;
pub const SIGTTIN: ::int_t = 21;
pub const SIGTTOU: ::int_t = 22;
pub const SIGUSR1: ::int_t = 10;
pub const SIGUSR2: ::int_t = 12;
pub const SIGPOLL: ::int_t = 29;
pub const SIGPROF: ::int_t = 27;
pub const SIGSYS: ::int_t = 31;
pub const SIGTRAP: ::int_t = 5;
pub const SIGURG: ::int_t = 23;
pub const SIGVTALRM: ::int_t = 26;
pub const SIGXCPU: ::int_t = 24;
pub const SIGXFSZ: ::int_t = 25;
pub const SIG_BLOCK: ::int_t = 0;
pub const SIG_UNBLOCK: ::int_t = 1;
pub const SIG_SETMASK: ::int_t = 2;
pub const SA_NOCLDSTOP: ::int_t = 1;
pub const SA_ONSTACK: ::int_t = 0x08000000;
pub const SA_RESETHAND: ::int_t = -2147483648;
pub const SA_RESTART: ::int_t = 0x10000000;
pub const SA_SIGINFO: ::int_t = 4;
pub const SA_NOCLDWAIT: ::int_t = 2;
pub const SA_NODEFER: ::int_t = 0x40000000;
pub const SS_ONSTACK: ::int_t = 1;
pub const SS_DISABLE: ::int_t = 2;
pub const MINSIGSTKSZ: ::int_t = 2048;
pub const SIGSTKSZ: ::int_t = 8192;
pub const ILL_ILLOPC: ::int_t = 1;
pub const ILL_ILLOPN: ::int_t = 2;
pub const ILL_ILLADR: ::int_t = 3;
pub const ILL_ILLTRP: ::int_t = 4;
pub const ILL_PRVOPC: ::int_t = 5;
pub const ILL_PRVREG: ::int_t = 6;
pub const ILL_COPROC: ::int_t = 7;
pub const ILL_BADSTK: ::int_t = 8;
pub const FPE_INTDIV: ::int_t = 1;
pub const FPE_INTOVF: ::int_t = 2;
pub const FPE_FLTDIV: ::int_t = 3;
pub const FPE_FLTOVF: ::int_t = 4;
pub const FPE_FLTUND: ::int_t = 5;
pub const FPE_FLTRES: ::int_t = 6;
pub const FPE_FLTINV: ::int_t = 7;
pub const FPE_FLTSUB: ::int_t = 8;
pub const SEGV_MAPERR: ::int_t = 1;
pub const SEGV_ACCERR: ::int_t = 2;
pub const BUS_ADRALN: ::int_t = 1;
pub const BUS_ADRERR: ::int_t = 2;
pub const BUS_OBJERR: ::int_t = 3;
pub const TRAP_BRKPT: ::int_t = 1;
pub const TRAP_TRACE: ::int_t = 2;
pub const CLD_EXITED: ::int_t = 1;
pub const CLD_KILLED: ::int_t = 2;
pub const CLD_DUMPED: ::int_t = 3;
pub const CLD_TRAPPED: ::int_t = 4;
pub const CLD_STOPPED: ::int_t = 5;
pub const CLD_CONTINUED: ::int_t = 6;
pub const POLL_IN: ::int_t = 1;
pub const POLL_OUT: ::int_t = 2;
pub const POLL_MSG: ::int_t = 3;
pub const POLL_ERR: ::int_t = 4;
pub const POLL_PRI: ::int_t = 5;
pub const POLL_HUP: ::int_t = 6;
pub const SI_USER: ::int_t = 0;
pub const SI_QUEUE: ::int_t = -1;
pub const SI_TIMER: ::int_t = -2;
pub const SI_ASYNCIO: ::int_t = -4;
pub const SI_MESGQ: ::int_t = -3;
