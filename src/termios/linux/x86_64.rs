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
    pub c_cc: [cc_t, ..32u],
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
pub static BRKINT:    tcflag_t = 2;
pub static ICRNL:     tcflag_t = 256;
pub static IGNBRK:    tcflag_t = 1;
pub static IGNCR:     tcflag_t = 128;
pub static IGNPAR:    tcflag_t = 4;
pub static INLCR:     tcflag_t = 64;
pub static INPCK:     tcflag_t = 16;
pub static ISTRIP:    tcflag_t = 32;
pub static IXANY:     tcflag_t = 2048;
pub static IXOFF:     tcflag_t = 4096;
pub static IXON:      tcflag_t = 1024;
pub static PARMRK:    tcflag_t = 8;
pub static OPOST:     tcflag_t = 1;
pub static ONLCR:     tcflag_t = 4;
pub static OCRNL:     tcflag_t = 8;
pub static ONOCR:     tcflag_t = 16;
pub static ONLRET:    tcflag_t = 32;
pub static OFDEL:     tcflag_t = 128;
pub static OFILL:     tcflag_t = 64;
pub static NLDLY:     tcflag_t = 256;
pub static NL0:       tcflag_t = 0;
pub static NL1:       tcflag_t = 256;
pub static CRDLY:     tcflag_t = 1536;
pub static CR0:       tcflag_t = 0;
pub static CR1:       tcflag_t = 512;
pub static CR2:       tcflag_t = 1024;
pub static CR3:       tcflag_t = 1536;
pub static TABDLY:    tcflag_t = 6144;
pub static TAB0:      tcflag_t = 0;
pub static TAB1:      tcflag_t = 2048;
pub static TAB2:      tcflag_t = 4096;
pub static TAB3:      tcflag_t = 6144;
pub static BSDLY:     tcflag_t = 8192;
pub static BS0:       tcflag_t = 0;
pub static BS1:       tcflag_t = 8192;
pub static VTDLY:     tcflag_t = 16384;
pub static VT0:       tcflag_t = 0;
pub static VT1:       tcflag_t = 16384;
pub static FFDLY:     tcflag_t = 32768;
pub static FF0:       tcflag_t = 0;
pub static FF1:       tcflag_t = 32768;
pub static B0:        tcflag_t = 0;
pub static B50:       tcflag_t = 1;
pub static B75:       tcflag_t = 2;
pub static B110:      tcflag_t = 3;
pub static B134:      tcflag_t = 4;
pub static B150:      tcflag_t = 5;
pub static B200:      tcflag_t = 6;
pub static B300:      tcflag_t = 7;
pub static B600:      tcflag_t = 8;
pub static B1200:     tcflag_t = 9;
pub static B1800:     tcflag_t = 10;
pub static B2400:     tcflag_t = 11;
pub static B4800:     tcflag_t = 12;
pub static B9600:     tcflag_t = 13;
pub static B19200:    tcflag_t = 14;
pub static B38400:    tcflag_t = 15;
pub static CSIZE:     tcflag_t = 48;
pub static CS5:       tcflag_t = 0;
pub static CS6:       tcflag_t = 16;
pub static CS7:       tcflag_t = 32;
pub static CS8:       tcflag_t = 48;
pub static CSTOPB:    tcflag_t = 64;
pub static CREAD:     tcflag_t = 128;
pub static PARENB:    tcflag_t = 256;
pub static PARODD:    tcflag_t = 512;
pub static HUPCL:     tcflag_t = 1024;
pub static CLOCAL:    tcflag_t = 2048;
pub static ECHO:      tcflag_t = 8;
pub static ECHOE:     tcflag_t = 16;
pub static ECHOK:     tcflag_t = 32;
pub static ECHONL:    tcflag_t = 64;
pub static ICANON:    tcflag_t = 2;
pub static IEXTEN:    tcflag_t = 32768;
pub static ISIG:      tcflag_t = 1;
pub static NOFLSH:    tcflag_t = 128;
pub static TOSTOP:    tcflag_t = 256;
pub static TCSANOW:   tcflag_t = 0;
pub static TCSADRAIN: tcflag_t = 1;
pub static TCSAFLUSH: tcflag_t = 2;
pub static TCIFLUSH:  tcflag_t = 0;
pub static TCIOFLUSH: tcflag_t = 2;
pub static TCOFLUSH:  tcflag_t = 1;
pub static TCIOFF:    tcflag_t = 2;
pub static TCION:     tcflag_t = 3;
pub static TCOOFF:    tcflag_t = 0;
pub static TCOON:     tcflag_t = 1;
