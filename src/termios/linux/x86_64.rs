pub type cc_t = ::uchar_t;
pub type speed_t = ::uint_t;
pub type tcflag_t = ::uint_t;

#[repr(C)]
#[deriving(Copy)]
pub struct termios {
    pub c_iflag: tcflag_t,
    pub c_oflag: tcflag_t,
    pub c_cflag: tcflag_t,
    pub c_lflag: tcflag_t,
    pub c_line: cc_t,
    pub c_cc: [cc_t, ..32u],
    pub c_ispeed: speed_t,
    pub c_ospeed: speed_t,
}

new!(termios);

pub const NCCS:      uint = 32;
pub const VEOF:      uint = 4;
pub const VEOL:      uint = 11;
pub const VERASE:    uint = 2;
pub const VINTR:     uint = 0;
pub const VKILL:     uint = 3;
pub const VMIN:      uint = 6;
pub const VQUIT:     uint = 1;
pub const VSTART:    uint = 8;
pub const VSTOP:     uint = 9;
pub const VSUSP:     uint = 10;
pub const VTIME:     uint = 5;
pub const BRKINT:    tcflag_t = 2;
pub const ICRNL:     tcflag_t = 256;
pub const IGNBRK:    tcflag_t = 1;
pub const IGNCR:     tcflag_t = 128;
pub const IGNPAR:    tcflag_t = 4;
pub const INLCR:     tcflag_t = 64;
pub const INPCK:     tcflag_t = 16;
pub const ISTRIP:    tcflag_t = 32;
pub const IXANY:     tcflag_t = 2048;
pub const IXOFF:     tcflag_t = 4096;
pub const IXON:      tcflag_t = 1024;
pub const PARMRK:    tcflag_t = 8;
pub const OPOST:     tcflag_t = 1;
pub const ONLCR:     tcflag_t = 4;
pub const OCRNL:     tcflag_t = 8;
pub const ONOCR:     tcflag_t = 16;
pub const ONLRET:    tcflag_t = 32;
pub const OFDEL:     tcflag_t = 128;
pub const OFILL:     tcflag_t = 64;
pub const NLDLY:     tcflag_t = 256;
pub const NL0:       tcflag_t = 0;
pub const NL1:       tcflag_t = 256;
pub const CRDLY:     tcflag_t = 1536;
pub const CR0:       tcflag_t = 0;
pub const CR1:       tcflag_t = 512;
pub const CR2:       tcflag_t = 1024;
pub const CR3:       tcflag_t = 1536;
pub const TABDLY:    tcflag_t = 6144;
pub const TAB0:      tcflag_t = 0;
pub const TAB1:      tcflag_t = 2048;
pub const TAB2:      tcflag_t = 4096;
pub const TAB3:      tcflag_t = 6144;
pub const BSDLY:     tcflag_t = 8192;
pub const BS0:       tcflag_t = 0;
pub const BS1:       tcflag_t = 8192;
pub const VTDLY:     tcflag_t = 16384;
pub const VT0:       tcflag_t = 0;
pub const VT1:       tcflag_t = 16384;
pub const FFDLY:     tcflag_t = 32768;
pub const FF0:       tcflag_t = 0;
pub const FF1:       tcflag_t = 32768;
pub const B0:        tcflag_t = 0;
pub const B50:       tcflag_t = 1;
pub const B75:       tcflag_t = 2;
pub const B110:      tcflag_t = 3;
pub const B134:      tcflag_t = 4;
pub const B150:      tcflag_t = 5;
pub const B200:      tcflag_t = 6;
pub const B300:      tcflag_t = 7;
pub const B600:      tcflag_t = 8;
pub const B1200:     tcflag_t = 9;
pub const B1800:     tcflag_t = 10;
pub const B2400:     tcflag_t = 11;
pub const B4800:     tcflag_t = 12;
pub const B9600:     tcflag_t = 13;
pub const B19200:    tcflag_t = 14;
pub const B38400:    tcflag_t = 15;
pub const CSIZE:     tcflag_t = 48;
pub const CS5:       tcflag_t = 0;
pub const CS6:       tcflag_t = 16;
pub const CS7:       tcflag_t = 32;
pub const CS8:       tcflag_t = 48;
pub const CSTOPB:    tcflag_t = 64;
pub const CREAD:     tcflag_t = 128;
pub const PARENB:    tcflag_t = 256;
pub const PARODD:    tcflag_t = 512;
pub const HUPCL:     tcflag_t = 1024;
pub const CLOCAL:    tcflag_t = 2048;
pub const ECHO:      tcflag_t = 8;
pub const ECHOE:     tcflag_t = 16;
pub const ECHOK:     tcflag_t = 32;
pub const ECHONL:    tcflag_t = 64;
pub const ICANON:    tcflag_t = 2;
pub const IEXTEN:    tcflag_t = 32768;
pub const ISIG:      tcflag_t = 1;
pub const NOFLSH:    tcflag_t = 128;
pub const TOSTOP:    tcflag_t = 256;
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
