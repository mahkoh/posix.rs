pub use self::os::{fexcept_t};
pub use self::os::{fenv_t};
pub use self::os::{FE_DIVBYZERO};
pub use self::os::{FE_INEXACT};
pub use self::os::{FE_INVALID};
pub use self::os::{FE_OVERFLOW};
pub use self::os::{FE_UNDERFLOW};
pub use self::os::{FE_ALL_EXCEPT};
pub use self::os::{FE_DOWNWARD};
pub use self::os::{FE_TONEAREST};
pub use self::os::{FE_TOWARDZERO};
pub use self::os::{FE_UPWARD};
pub use self::os::{FE_DFL_ENV};

use {int_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn feclearexcept(excepts: int_t) -> int_t {
    extern { fn feclearexcept(excepts: int_t) -> int_t; }
    unsafe { feclearexcept(excepts) }
}

pub fn fegetenv(envp: &mut fenv_t) -> int_t {
    extern { fn fegetenv(envp: *mut fenv_t) -> int_t; }
    unsafe { fegetenv(envp as *mut _) }
}

pub fn fegetexceptflag(flagp: &mut fexcept_t, excepts: int_t) -> int_t {
    extern { fn fegetexceptflag(flagp: *mut fexcept_t, excepts: int_t) -> int_t; }
    unsafe { fegetexceptflag(flagp as *mut _, excepts) }
}

pub fn fegetround() -> int_t {
    extern { fn fegetround() -> int_t; }
    unsafe { fegetround() }
}

pub fn feholdexcept(envp: &mut fenv_t) -> int_t {
    extern { fn feholdexcept(envp: *mut fenv_t) -> int_t; }
    unsafe { feholdexcept(envp as *mut _) }
}

pub fn feraiseexcept(excepts: int_t) -> int_t {
    extern { fn feraiseexcept(excepts: int_t) -> int_t; }
    unsafe { feraiseexcept(excepts) }
}

pub fn fesetenv(envp: &fenv_t) -> int_t {
    extern { fn fesetenv(envp: *const fenv_t) -> int_t; }
    unsafe { fesetenv(envp as *const _) }
}

pub fn fesetexceptflag(flagp: &fexcept_t, excepts: int_t) -> int_t {
    extern { fn fesetexceptflag(flagp: *const fexcept_t, excepts: int_t) -> int_t; }
    unsafe { fesetexceptflag(flagp as *const _, excepts) }
}

pub fn fesetround(rounding_direction: int_t) -> int_t {
    extern { fn fesetround(rounding_direction: int_t) -> int_t; }
    unsafe { fesetround(rounding_direction) }
}

pub fn fetestexcept(excepts: int_t) -> int_t {
    extern { fn fetestexcept(excepts: int_t) -> int_t; }
    unsafe { fetestexcept(excepts) }
}

pub fn feupdateenv(envp: &fenv_t) -> int_t {
    extern { fn feupdateenv(envp: *const fenv_t) -> int_t; }
    unsafe { feupdateenv(envp as *const _) }
}
