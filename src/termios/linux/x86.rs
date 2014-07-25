pub type cc_t = ::uchar_t;
pub type speed_t = ::uint_t;
pub type tcflag_t = ::uint_t;
#[repr(C)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t, ..32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
new!(termios)
pub static NCCS: ::int_t = 32;
pub static VEOF: ::int_t = 4;
pub static VEOL: ::int_t = 11;
pub static VERASE: ::int_t = 2;
pub static VINTR: ::int_t = 0;
pub static VKILL: ::int_t = 3;
pub static VMIN: ::int_t = 6;
pub static VQUIT: ::int_t = 1;
pub static VSTART: ::int_t = 8;
pub static VSTOP: ::int_t = 9;
pub static VSUSP: ::int_t = 10;
pub static VTIME: ::int_t = 5;
pub static BRKINT: ::int_t = 0o000002;
pub static ICRNL: ::int_t = 0o000400;
pub static IGNBRK: ::int_t = 0o000001;
pub static IGNCR: ::int_t = 0o000200;
pub static IGNPAR: ::int_t = 0o000004;
pub static INLCR: ::int_t = 0o000100;
pub static INPCK: ::int_t = 0o000020;
pub static ISTRIP: ::int_t = 0o000040;
pub static IXANY: ::int_t = 0o004000;
pub static IXOFF: ::int_t = 0o010000;
pub static IXON: ::int_t = 0o002000;
pub static PARMRK: ::int_t = 0o000010;
pub static OPOST: ::int_t = 0o000001;
pub static ONLCR: ::int_t = 0o000004;
pub static OCRNL: ::int_t = 0o000010;
pub static ONOCR: ::int_t = 0o000020;
pub static ONLRET: ::int_t = 0o000040;
pub static OFDEL: ::int_t = 0o000200;
pub static OFILL: ::int_t = 0o000100;
pub static NLDLY: ::int_t = 0o000400;
pub static NL0: ::int_t = 0o000000;
pub static NL1: ::int_t = 0o000400;
pub static CRDLY: ::int_t = 0o003000;
pub static CR0: ::int_t = 0o000000;
pub static CR1: ::int_t = 0o001000;
pub static CR2: ::int_t = 0o002000;
pub static CR3: ::int_t = 0o003000;
pub static TABDLY: ::int_t = 0o014000;
pub static TAB0: ::int_t = 0o000000;
pub static TAB1: ::int_t = 0o004000;
pub static TAB2: ::int_t = 0o010000;
pub static TAB3: ::int_t = 0o014000;
pub static BSDLY: ::int_t = 0o020000;
pub static BS0: ::int_t = 0o000000;
pub static BS1: ::int_t = 0o020000;
pub static VTDLY: ::int_t = 0o040000;
pub static VT0: ::int_t = 0o000000;
pub static VT1: ::int_t = 0o040000;
pub static FFDLY: ::int_t = 0o100000;
pub static FF0: ::int_t = 0o000000;
pub static FF1: ::int_t = 0o100000;
pub static B0: ::int_t = 0o000000;
pub static B50: ::int_t = 0o000001;
pub static B75: ::int_t = 0o000002;
pub static B110: ::int_t = 0o000003;
pub static B134: ::int_t = 0o000004;
pub static B150: ::int_t = 0o000005;
pub static B200: ::int_t = 0o000006;
pub static B300: ::int_t = 0o000007;
pub static B600: ::int_t = 0o000010;
pub static B1200: ::int_t = 0o000011;
pub static B1800: ::int_t = 0o000012;
pub static B2400: ::int_t = 0o000013;
pub static B4800: ::int_t = 0o000014;
pub static B9600: ::int_t = 0o000015;
pub static B19200: ::int_t = 0o000016;
pub static B38400: ::int_t = 0o000017;
pub static CSIZE: ::int_t = 0o000060;
pub static CS5: ::int_t = 0o000000;
pub static CS6: ::int_t = 0o000020;
pub static CS7: ::int_t = 0o000040;
pub static CS8: ::int_t = 0o000060;
pub static CSTOPB: ::int_t = 0o000100;
pub static CREAD: ::int_t = 0o000200;
pub static PARENB: ::int_t = 0o000400;
pub static PARODD: ::int_t = 0o001000;
pub static HUPCL: ::int_t = 0o002000;
pub static CLOCAL: ::int_t = 0o004000;
pub static ECHO: ::int_t = 0o000010;
pub static ECHOE: ::int_t = 0o000020;
pub static ECHOK: ::int_t = 0o000040;
pub static ECHONL: ::int_t = 0o000100;
pub static ICANON: ::int_t = 0o000002;
pub static IEXTEN: ::int_t = 0o100000;
pub static ISIG: ::int_t = 0o000001;
pub static NOFLSH: ::int_t = 0o000200;
pub static TOSTOP: ::int_t = 0o000400;
pub static TCSANOW: ::int_t = 0;
pub static TCSADRAIN: ::int_t = 1;
pub static TCSAFLUSH: ::int_t = 2;
pub static TCIFLUSH: ::int_t = 0;
pub static TCIOFLUSH: ::int_t = 2;
pub static TCOFLUSH: ::int_t = 1;
pub static TCIOFF: ::int_t = 2;
pub static TCION: ::int_t = 3;
pub static TCOOFF: ::int_t = 0;
pub static TCOON: ::int_t = 1;
