pub use self::os::{idtype_t};
pub use self::os::{P_ALL};
pub use self::os::{P_PID};
pub use self::os::{P_PGID};
pub use self::os::{WCONTINUED};
pub use self::os::{WNOHANG};
pub use self::os::{WUNTRACED};
pub use self::os::{WEXITED};
pub use self::os::{WNOWAIT};
pub use self::os::{WSTOPPED};
pub use self::os::{WEXITSTATUS};
pub use self::os::{WIFCONTINUED};
pub use self::os::{WIFEXITED};
pub use self::os::{WIFSIGNALED};
pub use self::os::{WIFSTOPPED};
pub use self::os::{WSTOPSIG};
pub use self::os::{WTERMSIG};

use {int_t}; 
use sys::types::{pid_t, id_t};
use signal::{siginfo_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn wait(status: &mut int_t) -> pid_t {
    extern { fn wait(status: *mut int_t) -> pid_t; }
    unsafe { wait(status as *mut _) }
}

pub fn waitpid(pid: pid_t, stat_loc: &mut int_t, options: int_t) -> pid_t {
    extern { fn waitpid(pid: pid_t, stat_loc: *mut int_t,
                        options: int_t) -> pid_t; }
    unsafe { waitpid(pid, stat_loc as *mut _, options) }
}

pub fn waitid(idtype: idtype_t, id: id_t, infop: &mut siginfo_t,
              options: int_t) -> int_t {
    extern { fn waitid(idtype: idtype_t, id: id_t,
                       infop: *mut siginfo_t, options: int_t) -> int_t; }
    unsafe { waitid(idtype, id, infop as *mut _, options) }
}
