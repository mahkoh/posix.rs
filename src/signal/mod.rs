pub use self::os::{sig_atomic_t};
pub use self::os::{sigset_t};
pub use self::os::{sigevent};
pub use self::os::{sigval};
pub use self::os::{sigaction};
pub use self::os::{mcontext_t};
pub use self::os::{ucontext};
pub use self::os::{stack_t};
pub use self::os::{siginfo_t};
pub use self::os::{SIG_DFL};
pub use self::os::{SIG_ERR};
pub use self::os::{SIG_IGN};
pub use self::os::{SIGEV_NONE};
pub use self::os::{SIGEV_SIGNAL};
pub use self::os::{SIGEV_THREAD};
pub use self::os::{SIGABRT};
pub use self::os::{SIGALRM};
pub use self::os::{SIGBUS};
pub use self::os::{SIGCHLD};
pub use self::os::{SIGCONT};
pub use self::os::{SIGFPE};
pub use self::os::{SIGHUP};
pub use self::os::{SIGILL};
pub use self::os::{SIGINT};
pub use self::os::{SIGKILL};
pub use self::os::{SIGPIPE};
pub use self::os::{SIGQUIT};
pub use self::os::{SIGSEGV};
pub use self::os::{SIGSTOP};
pub use self::os::{SIGTERM};
pub use self::os::{SIGTSTP};
pub use self::os::{SIGTTIN};
pub use self::os::{SIGTTOU};
pub use self::os::{SIGUSR1};
pub use self::os::{SIGUSR2};
pub use self::os::{SIGPOLL};
pub use self::os::{SIGPROF};
pub use self::os::{SIGSYS};
pub use self::os::{SIGTRAP};
pub use self::os::{SIGURG};
pub use self::os::{SIGVTALRM};
pub use self::os::{SIGXCPU};
pub use self::os::{SIGXFSZ};
pub use self::os::{SIG_BLOCK};
pub use self::os::{SIG_UNBLOCK};
pub use self::os::{SIG_SETMASK};
pub use self::os::{SA_NOCLDSTOP};
pub use self::os::{SA_ONSTACK};
pub use self::os::{SA_RESETHAND};
pub use self::os::{SA_RESTART};
pub use self::os::{SA_SIGINFO};
pub use self::os::{SA_NOCLDWAIT};
pub use self::os::{SA_NODEFER};
pub use self::os::{SS_ONSTACK};
pub use self::os::{SS_DISABLE};
pub use self::os::{MINSIGSTKSZ};
pub use self::os::{SIGSTKSZ};
pub use self::os::{ILL_ILLOPC};
pub use self::os::{ILL_ILLOPN};
pub use self::os::{ILL_ILLADR};
pub use self::os::{ILL_ILLTRP};
pub use self::os::{ILL_PRVOPC};
pub use self::os::{ILL_PRVREG};
pub use self::os::{ILL_COPROC};
pub use self::os::{ILL_BADSTK};
pub use self::os::{FPE_INTDIV};
pub use self::os::{FPE_INTOVF};
pub use self::os::{FPE_FLTDIV};
pub use self::os::{FPE_FLTOVF};
pub use self::os::{FPE_FLTUND};
pub use self::os::{FPE_FLTRES};
pub use self::os::{FPE_FLTINV};
pub use self::os::{FPE_FLTSUB};
pub use self::os::{SEGV_MAPERR};
pub use self::os::{SEGV_ACCERR};
pub use self::os::{BUS_ADRALN};
pub use self::os::{BUS_ADRERR};
pub use self::os::{BUS_OBJERR};
pub use self::os::{TRAP_BRKPT};
pub use self::os::{TRAP_TRACE};
pub use self::os::{CLD_EXITED};
pub use self::os::{CLD_KILLED};
pub use self::os::{CLD_DUMPED};
pub use self::os::{CLD_TRAPPED};
pub use self::os::{CLD_STOPPED};
pub use self::os::{CLD_CONTINUED};
pub use self::os::{POLL_IN};
pub use self::os::{POLL_OUT};
pub use self::os::{POLL_MSG};
pub use self::os::{POLL_ERR};
pub use self::os::{POLL_PRI};
pub use self::os::{POLL_HUP};
pub use self::os::{SI_USER};
pub use self::os::{SI_QUEUE};
pub use self::os::{SI_TIMER};
pub use self::os::{SI_ASYNCIO};
pub use self::os::{SI_MESGQ};

use {NTStr, int_t, char_t};
use sys::types::{pid_t, pthread_t};
use time::{timespec};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn kill(pid: pid_t, sig: int_t) -> int_t {
    extern { fn kill(pid: pid_t, sig: int_t) -> int_t; }
    unsafe { kill(pid, sig) }
}

pub fn killpg(pid: pid_t, sig: int_t) -> int_t {
    extern { fn killpg(pid: pid_t, sig: int_t) -> int_t; }
    unsafe { killpg(pid, sig) }
}

pub fn psiginfo<T: NTStr>(pinfo: &siginfo_t, s: &T) {
    extern { fn psiginfo(pinfo: *const siginfo_t, s: *const char_t); }
    unsafe { psiginfo(pinfo as *const _, s.as_ptr()) }
}

pub fn psignal<T: NTStr>(sig: int_t, s: &T) {
    extern { fn psignal(sig: int_t, s: *const char_t); }
    unsafe { psignal(sig, s.as_ptr()) }
}

pub fn pthread_kill(threadid: pthread_t, signo: int_t) -> int_t {
    extern { fn pthread_kill(id: pthread_t, signo: int_t) -> int_t; }
    unsafe { pthread_kill(threadid, signo) }
}

pub fn pthread_sigmask(how: int_t, new: &sigset_t, old: Option<&mut sigset_t>) -> int_t {
    extern { fn pthread_sigmask(how: int_t, new: *const sigset_t, old: *mut sigset_t) -> int_t; }
    match old {
        Some(p) => unsafe { pthread_sigmask(how, new as *const _, p as *mut _) },
        None => unsafe { pthread_sigmask(how, new as *const _, 0 as *mut _) },
    }
}

pub fn raise(sig: int_t) -> int_t {
    extern { fn raise(sig: int_t) -> int_t; }
    unsafe { raise(sig) }
}

pub fn sigaction(sig: int_t, act: &sigaction, oact: Option<&mut sigaction>) -> int_t {
    extern {
        fn sigaction(sig: int_t, act: *const sigaction, old: *mut sigaction) -> int_t;
    }
    match oact {
        Some(p) => unsafe { sigaction(sig, act as *const _, p as *mut _) },
        None => unsafe { sigaction(sig, act as *const _, 0 as *mut _) },
    }
}

pub fn sigaddset(set: &mut sigset_t, signo: int_t) -> int_t {
    extern { fn sigaddset(set: *mut sigset_t, signo: int_t) -> int_t; }
    unsafe { sigaddset(set as *mut _, signo) }
}

pub fn sigaltstack(ss: &stack_t, oss: Option<&mut stack_t>) -> int_t {
    extern { fn sigaltstack(ss: *const stack_t, oss: *mut stack_t) -> int_t; }
    match oss {
        Some(p) => unsafe { sigaltstack(ss as *const _, p as *mut _) },
        None => unsafe { sigaltstack(ss as *const _, 0 as *mut _) },
    }
}

pub fn sigdelset(set: &mut sigset_t, signo: int_t) -> int_t {
    extern { fn sigdelset(set: *mut sigset_t, signo: int_t) -> int_t; }
    unsafe { sigdelset(set as *mut _, signo) }
}

pub fn sigemptyset(set: &mut sigset_t) -> int_t {
    extern { fn sigemptyset(set: *mut sigset_t) -> int_t; }
    unsafe { sigemptyset(set as *mut _) }
}

pub fn sigfillset(set: &mut sigset_t) -> int_t {
    extern { fn sigfillset(set: *mut sigset_t) -> int_t; }
    unsafe { sigfillset(set as *mut _) }
}

pub fn siginterrupt(sig: int_t, interrupt: int_t) -> int_t {
    extern { fn siginterrupt(sig: int_t, interrupt: int_t) -> int_t; }
    unsafe { siginterrupt(sig, interrupt) }
}

pub fn sigismember(set: &sigset_t, signo: int_t) -> int_t {
    extern { fn sigismember(set: *const sigset_t, signo: int_t) -> int_t; }
    unsafe { sigismember(set as *const _, signo) }
}

pub fn signal(sig: int_t, handler: extern fn(int_t)) -> extern fn(int_t) {
    extern { fn signal(sig: int_t, handler: extern fn(int_t)) -> extern fn(int_t); }
    unsafe { signal(sig, handler) }
}

pub fn sigpending(set: &mut sigset_t) -> int_t {
    extern { fn sigpending(set: *mut sigset_t) -> int_t; }
    unsafe { sigpending(set as *mut _) }
}

pub fn sigprocmask(how: int_t, set: &sigset_t, oset: Option<&mut sigset_t>) -> int_t {
    extern { fn sigprocmask(how: int_t, set: *const sigset_t, oset: *mut sigset_t) -> int_t; }
    match oset {
        Some(p) => unsafe { sigprocmask(how, set as *const _, p as *mut _) },
        None => unsafe { sigprocmask(how, set as *const _, 0 as *mut _) },
    }
}

pub fn sigqueue(pid: pid_t, sig: int_t, val: sigval) -> int_t {
    extern { fn sigqueue(pid: pid_t, sig: int_t, val: sigval) -> int_t; }
    unsafe { sigqueue(pid, sig, val) }
}

pub fn sigsuspend(set: &sigset_t) -> int_t {
    extern { fn sigsuspend(set: *const sigset_t) -> int_t; }
    unsafe { sigsuspend(set as *const _) }
}

pub fn sigtimedwait(set: &sigset_t, info: Option<&mut siginfo_t>,
                    out: &timespec) -> int_t {
    extern { fn sigtimedwait(set: *const sigset_t, info: *mut siginfo_t,
                             timeout: *const timespec) -> int_t; }
    match info {
        Some(p) => unsafe { sigtimedwait(set as *const _, p as *mut _, out as *const _) },
        _ => unsafe { sigtimedwait(set as *const _, 0 as *mut _, out as *const _) },
    }
}

pub fn sigwait(set: &sigset_t, sig: &mut int_t) -> int_t {
    extern { fn sigwait(set: *const sigset_t, sig: *mut int_t) -> int_t; }
    unsafe { sigwait(set as *const _, sig as *mut _) }
}

pub fn sigwaitinfo(set: &sigset_t, info: Option<&mut siginfo_t>) -> int_t {
    extern {  fn sigwaitinfo(set: *const sigset_t, info: *mut siginfo_t) -> int_t; }
    match info {
        Some(p) => unsafe { sigwaitinfo(set as *const _, p as *mut _) },
        _ => unsafe { sigwaitinfo(set as *const _, 0 as *mut _) },
    }
}
