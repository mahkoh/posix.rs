pub type cc_t = ::uchar_t;
pub type speed_t = ::uint_t;
pub type tcflag_t = ::uint_t;
#[repr(C)]
#[derive(Copy)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t; 32],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}
new!(termios);
pub const NCCS:      usize = 32;
pub const VEOF:      usize = 4;
pub const VEOL:      usize = 11;
pub const VERASE:    usize = 2;
pub const VINTR:     usize = 0;
pub const VKILL:     usize = 3;
pub const VMIN:      usize = 6;
pub const VQUIT:     usize = 1;
pub const VSTART:    usize = 8;
pub const VSTOP:     usize = 9;
pub const VSUSP:     usize = 10;
pub const VTIME:     usize = 5;
pub const BRKINT:    tcflag_t = 0o000002;
pub const ICRNL:     tcflag_t = 0o000400;
pub const IGNBRK:    tcflag_t = 0o000001;
pub const IGNCR:     tcflag_t = 0o000200;
pub const IGNPAR:    tcflag_t = 0o000004;
pub const INLCR:     tcflag_t = 0o000100;
pub const INPCK:     tcflag_t = 0o000020;
pub const ISTRIP:    tcflag_t = 0o000040;
pub const IXANY:     tcflag_t = 0o004000;
pub const IXOFF:     tcflag_t = 0o010000;
pub const IXON:      tcflag_t = 0o002000;
pub const PARMRK:    tcflag_t = 0o000010;
pub const OPOST:     tcflag_t = 0o000001;
pub const ONLCR:     tcflag_t = 0o000004;
pub const OCRNL:     tcflag_t = 0o000010;
pub const ONOCR:     tcflag_t = 0o000020;
pub const ONLRET:    tcflag_t = 0o000040;
pub const OFDEL:     tcflag_t = 0o000200;
pub const OFILL:     tcflag_t = 0o000100;
pub const NLDLY:     tcflag_t = 0o000400;
pub const NL0:       tcflag_t = 0o000000;
pub const NL1:       tcflag_t = 0o000400;
pub const CRDLY:     tcflag_t = 0o003000;
pub const CR0:       tcflag_t = 0o000000;
pub const CR1:       tcflag_t = 0o001000;
pub const CR2:       tcflag_t = 0o002000;
pub const CR3:       tcflag_t = 0o003000;
pub const TABDLY:    tcflag_t = 0o014000;
pub const TAB0:      tcflag_t = 0o000000;
pub const TAB1:      tcflag_t = 0o004000;
pub const TAB2:      tcflag_t = 0o010000;
pub const TAB3:      tcflag_t = 0o014000;
pub const BSDLY:     tcflag_t = 0o020000;
pub const BS0:       tcflag_t = 0o000000;
pub const BS1:       tcflag_t = 0o020000;
pub const VTDLY:     tcflag_t = 0o040000;
pub const VT0:       tcflag_t = 0o000000;
pub const VT1:       tcflag_t = 0o040000;
pub const FFDLY:     tcflag_t = 0o100000;
pub const FF0:       tcflag_t = 0o000000;
pub const FF1:       tcflag_t = 0o100000;
pub const B0:        tcflag_t = 0o000000;
pub const B50:       tcflag_t = 0o000001;
pub const B75:       tcflag_t = 0o000002;
pub const B110:      tcflag_t = 0o000003;
pub const B134:      tcflag_t = 0o000004;
pub const B150:      tcflag_t = 0o000005;
pub const B200:      tcflag_t = 0o000006;
pub const B300:      tcflag_t = 0o000007;
pub const B600:      tcflag_t = 0o000010;
pub const B1200:     tcflag_t = 0o000011;
pub const B1800:     tcflag_t = 0o000012;
pub const B2400:     tcflag_t = 0o000013;
pub const B4800:     tcflag_t = 0o000014;
pub const B9600:     tcflag_t = 0o000015;
pub const B19200:    tcflag_t = 0o000016;
pub const B38400:    tcflag_t = 0o000017;
pub const CSIZE:     tcflag_t = 0o000060;
pub const CS5:       tcflag_t = 0o000000;
pub const CS6:       tcflag_t = 0o000020;
pub const CS7:       tcflag_t = 0o000040;
pub const CS8:       tcflag_t = 0o000060;
pub const CSTOPB:    tcflag_t = 0o000100;
pub const CREAD:     tcflag_t = 0o000200;
pub const PARENB:    tcflag_t = 0o000400;
pub const PARODD:    tcflag_t = 0o001000;
pub const HUPCL:     tcflag_t = 0o002000;
pub const CLOCAL:    tcflag_t = 0o004000;
pub const ECHO:      tcflag_t = 0o000010;
pub const ECHOE:     tcflag_t = 0o000020;
pub const ECHOK:     tcflag_t = 0o000040;
pub const ECHONL:    tcflag_t = 0o000100;
pub const ICANON:    tcflag_t = 0o000002;
pub const IEXTEN:    tcflag_t = 0o100000;
pub const ISIG:      tcflag_t = 0o000001;
pub const NOFLSH:    tcflag_t = 0o000200;
pub const TOSTOP:    tcflag_t = 0o000400;
pub const TCSANOW:   ::int_t = 0;
pub const TCSADRAIN: ::int_t = 1;
pub const TCSAFLUSH: ::int_t = 2;
pub const TCIFLUSH:  ::int_t = 0;
pub const TCIOFLUSH: ::int_t = 2;
pub const TCOFLUSH:  ::int_t = 1;
pub const TCIOFF:    ::int_t = 2;
pub const TCION:     ::int_t = 3;
pub const TCOOFF:    ::int_t = 0;
pub const TCOON:     ::int_t = 1;
