pub use self::os::{cc_t};
pub use self::os::{speed_t};
pub use self::os::{tcflag_t};
pub use self::os::{termios};
pub use self::os::{NCCS};
pub use self::os::{VEOF};
pub use self::os::{VEOL};
pub use self::os::{VERASE};
pub use self::os::{VINTR};
pub use self::os::{VKILL};
pub use self::os::{VMIN};
pub use self::os::{VQUIT};
pub use self::os::{VSTART};
pub use self::os::{VSTOP};
pub use self::os::{VSUSP};
pub use self::os::{VTIME};
pub use self::os::{BRKINT};
pub use self::os::{ICRNL};
pub use self::os::{IGNBRK};
pub use self::os::{IGNCR};
pub use self::os::{IGNPAR};
pub use self::os::{INLCR};
pub use self::os::{INPCK};
pub use self::os::{ISTRIP};
pub use self::os::{IXANY};
pub use self::os::{IXOFF};
pub use self::os::{IXON};
pub use self::os::{PARMRK};
pub use self::os::{OPOST};
pub use self::os::{ONLCR};
pub use self::os::{OCRNL};
pub use self::os::{ONOCR};
pub use self::os::{ONLRET};
pub use self::os::{OFDEL};
pub use self::os::{OFILL};
pub use self::os::{NLDLY};
pub use self::os::{NL0};
pub use self::os::{NL1};
pub use self::os::{CRDLY};
pub use self::os::{CR0};
pub use self::os::{CR1};
pub use self::os::{CR2};
pub use self::os::{CR3};
pub use self::os::{TABDLY};
pub use self::os::{TAB0};
pub use self::os::{TAB1};
pub use self::os::{TAB2};
pub use self::os::{TAB3};
pub use self::os::{BSDLY};
pub use self::os::{BS0};
pub use self::os::{BS1};
pub use self::os::{VTDLY};
pub use self::os::{VT0};
pub use self::os::{VT1};
pub use self::os::{FFDLY};
pub use self::os::{FF0};
pub use self::os::{FF1};
pub use self::os::{B0};
pub use self::os::{B50};
pub use self::os::{B75};
pub use self::os::{B110};
pub use self::os::{B134};
pub use self::os::{B150};
pub use self::os::{B200};
pub use self::os::{B300};
pub use self::os::{B600};
pub use self::os::{B1200};
pub use self::os::{B1800};
pub use self::os::{B2400};
pub use self::os::{B4800};
pub use self::os::{B9600};
pub use self::os::{B19200};
pub use self::os::{B38400};
pub use self::os::{CSIZE};
pub use self::os::{CS5};
pub use self::os::{CS6};
pub use self::os::{CS7};
pub use self::os::{CS8};
pub use self::os::{CSTOPB};
pub use self::os::{CREAD};
pub use self::os::{PARENB};
pub use self::os::{PARODD};
pub use self::os::{HUPCL};
pub use self::os::{CLOCAL};
pub use self::os::{ECHO};
pub use self::os::{ECHOE};
pub use self::os::{ECHOK};
pub use self::os::{ECHONL};
pub use self::os::{ICANON};
pub use self::os::{IEXTEN};
pub use self::os::{ISIG};
pub use self::os::{NOFLSH};
pub use self::os::{TOSTOP};
pub use self::os::{TCSANOW};
pub use self::os::{TCSADRAIN};
pub use self::os::{TCSAFLUSH};
pub use self::os::{TCIFLUSH};
pub use self::os::{TCIOFLUSH};
pub use self::os::{TCOFLUSH};
pub use self::os::{TCIOFF};
pub use self::os::{TCION};
pub use self::os::{TCOOFF};
pub use self::os::{TCOON};

use {int_t};
use sys::types::{pid_t};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

pub fn cfgetispeed(termios_p: &termios) -> speed_t {
    extern { fn cfgetispeed(termios_p: *const termios) -> speed_t; }
    unsafe { cfgetispeed(termios_p as *const _) }
}

pub fn cfgetospeed(termios_p: &termios) -> speed_t {
    extern { fn cfgetospeed(termios_p: *const termios) -> speed_t; }
    unsafe { cfgetospeed(termios_p as *const _) }
}

pub fn cfsetispeed(termios_p: &mut termios, speed: speed_t) -> int_t {
    extern { fn cfsetispeed(termios_p: *mut termios, speed: speed_t) -> int_t; }
    unsafe { cfsetispeed(termios_p as *mut _, speed) }
}

pub fn cfsetospeed(termios_p: &mut termios, speed: speed_t) -> int_t {
    extern { fn cfsetospeed(termios_p: *mut termios, speed: speed_t) -> int_t; }
    unsafe { cfsetospeed(termios_p as *mut _, speed) }
}

pub fn tcdrain(fd: int_t) -> int_t {
    extern { fn tcdrain(fd: int_t) -> int_t; }
    unsafe { tcdrain(fd) }
}

pub fn tcflow(fd: int_t, action: int_t) -> int_t {
    extern { fn tcflow(fd: int_t, action: int_t) -> int_t; }
    unsafe { tcflow(fd, action) }
}

pub fn tcflush(fd: int_t, queue_selector: int_t) -> int_t {
    extern { fn tcflush(fd: int_t, queue_selector: int_t) -> int_t; }
    unsafe { tcflush(fd, queue_selector) }
}

pub fn tcgetattr(fd: int_t, termios_p: &mut termios) -> int_t {
    extern { fn tcgetattr(fd: int_t, termios_p: *mut termios) -> int_t; }
    unsafe { tcgetattr(fd, termios_p as *mut _) }
}

pub fn tcgetsid(fd: int_t) -> pid_t {
    extern { fn tcgetsid(fd: int_t) -> pid_t; }
    unsafe { tcgetsid(fd) }
}

pub fn tcsendbreak(fd: int_t, duration: int_t) -> int_t {
    extern { fn tcsendbreak(fd: int_t, duration: int_t) -> int_t; }
    unsafe { tcsendbreak(fd, duration) }
}

pub fn tcsetattr(fd: int_t, optional_actions: int_t, termios_p: &termios) -> int_t {
    extern { fn tcsetattr(fd: int_t, optional_actions: int_t,
                          termios_p: *const termios) -> int_t; }
    unsafe { tcsetattr(fd, optional_actions, termios_p as *const _) }
}
