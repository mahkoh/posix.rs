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
pub static NCCS:      uint = 32;
pub static VEOF:      uint = 4;
pub static VEOL:      uint = 11;
pub static VERASE:    uint = 2;
pub static VINTR:     uint = 0;
pub static VKILL:     uint = 3;
pub static VMIN:      uint = 6;
pub static VQUIT:     uint = 1;
pub static VSTART:    uint = 8;
pub static VSTOP:     uint = 9;
pub static VSUSP:     uint = 10;
pub static VTIME:     uint = 5;
pub static BRKINT:    tcflag_t = 0o000002;
pub static ICRNL:     tcflag_t = 0o000400;
pub static IGNBRK:    tcflag_t = 0o000001;
pub static IGNCR:     tcflag_t = 0o000200;
pub static IGNPAR:    tcflag_t = 0o000004;
pub static INLCR:     tcflag_t = 0o000100;
pub static INPCK:     tcflag_t = 0o000020;
pub static ISTRIP:    tcflag_t = 0o000040;
pub static IXANY:     tcflag_t = 0o004000;
pub static IXOFF:     tcflag_t = 0o010000;
pub static IXON:      tcflag_t = 0o002000;
pub static PARMRK:    tcflag_t = 0o000010;
pub static OPOST:     tcflag_t = 0o000001;
pub static ONLCR:     tcflag_t = 0o000004;
pub static OCRNL:     tcflag_t = 0o000010;
pub static ONOCR:     tcflag_t = 0o000020;
pub static ONLRET:    tcflag_t = 0o000040;
pub static OFDEL:     tcflag_t = 0o000200;
pub static OFILL:     tcflag_t = 0o000100;
pub static NLDLY:     tcflag_t = 0o000400;
pub static NL0:       tcflag_t = 0o000000;
pub static NL1:       tcflag_t = 0o000400;
pub static CRDLY:     tcflag_t = 0o003000;
pub static CR0:       tcflag_t = 0o000000;
pub static CR1:       tcflag_t = 0o001000;
pub static CR2:       tcflag_t = 0o002000;
pub static CR3:       tcflag_t = 0o003000;
pub static TABDLY:    tcflag_t = 0o014000;
pub static TAB0:      tcflag_t = 0o000000;
pub static TAB1:      tcflag_t = 0o004000;
pub static TAB2:      tcflag_t = 0o010000;
pub static TAB3:      tcflag_t = 0o014000;
pub static BSDLY:     tcflag_t = 0o020000;
pub static BS0:       tcflag_t = 0o000000;
pub static BS1:       tcflag_t = 0o020000;
pub static VTDLY:     tcflag_t = 0o040000;
pub static VT0:       tcflag_t = 0o000000;
pub static VT1:       tcflag_t = 0o040000;
pub static FFDLY:     tcflag_t = 0o100000;
pub static FF0:       tcflag_t = 0o000000;
pub static FF1:       tcflag_t = 0o100000;
pub static B0:        tcflag_t = 0o000000;
pub static B50:       tcflag_t = 0o000001;
pub static B75:       tcflag_t = 0o000002;
pub static B110:      tcflag_t = 0o000003;
pub static B134:      tcflag_t = 0o000004;
pub static B150:      tcflag_t = 0o000005;
pub static B200:      tcflag_t = 0o000006;
pub static B300:      tcflag_t = 0o000007;
pub static B600:      tcflag_t = 0o000010;
pub static B1200:     tcflag_t = 0o000011;
pub static B1800:     tcflag_t = 0o000012;
pub static B2400:     tcflag_t = 0o000013;
pub static B4800:     tcflag_t = 0o000014;
pub static B9600:     tcflag_t = 0o000015;
pub static B19200:    tcflag_t = 0o000016;
pub static B38400:    tcflag_t = 0o000017;
pub static CSIZE:     tcflag_t = 0o000060;
pub static CS5:       tcflag_t = 0o000000;
pub static CS6:       tcflag_t = 0o000020;
pub static CS7:       tcflag_t = 0o000040;
pub static CS8:       tcflag_t = 0o000060;
pub static CSTOPB:    tcflag_t = 0o000100;
pub static CREAD:     tcflag_t = 0o000200;
pub static PARENB:    tcflag_t = 0o000400;
pub static PARODD:    tcflag_t = 0o001000;
pub static HUPCL:     tcflag_t = 0o002000;
pub static CLOCAL:    tcflag_t = 0o004000;
pub static ECHO:      tcflag_t = 0o000010;
pub static ECHOE:     tcflag_t = 0o000020;
pub static ECHOK:     tcflag_t = 0o000040;
pub static ECHONL:    tcflag_t = 0o000100;
pub static ICANON:    tcflag_t = 0o000002;
pub static IEXTEN:    tcflag_t = 0o100000;
pub static ISIG:      tcflag_t = 0o000001;
pub static NOFLSH:    tcflag_t = 0o000200;
pub static TOSTOP:    tcflag_t = 0o000400;
pub static TCSANOW:   ::int_t = 0;
pub static TCSADRAIN: ::int_t = 1;
pub static TCSAFLUSH: ::int_t = 2;
pub static TCIFLUSH:  ::int_t = 0;
pub static TCIOFLUSH: ::int_t = 2;
pub static TCOFLUSH:  ::int_t = 1;
pub static TCIOFF:    ::int_t = 2;
pub static TCION:     ::int_t = 3;
pub static TCOOFF:    ::int_t = 0;
pub static TCOON:     ::int_t = 1;
