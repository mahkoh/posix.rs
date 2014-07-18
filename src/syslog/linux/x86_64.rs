pub static LOG_PID:     ::int_t = 1;
pub static LOG_CONS:    ::int_t = 2;
pub static LOG_NDELAY:  ::int_t = 8;
pub static LOG_ODELAY:  ::int_t = 4;
pub static LOG_NOWAIT:  ::int_t = 16;
pub static LOG_KERN:    ::int_t = 0;
pub static LOG_USER:    ::int_t = 8;
pub static LOG_MAIL:    ::int_t = 16;
pub static LOG_NEWS:    ::int_t = 56;
pub static LOG_UUCP:    ::int_t = 64;
pub static LOG_DAEMON:  ::int_t = 24;
pub static LOG_AUTH:    ::int_t = 32;
pub static LOG_CRON:    ::int_t = 72;
pub static LOG_LPR:     ::int_t = 48;
pub static LOG_LOCAL0:  ::int_t = 128;
pub static LOG_LOCAL1:  ::int_t = 136;
pub static LOG_LOCAL2:  ::int_t = 144;
pub static LOG_LOCAL3:  ::int_t = 152;
pub static LOG_LOCAL4:  ::int_t = 160;
pub static LOG_LOCAL5:  ::int_t = 168;
pub static LOG_LOCAL6:  ::int_t = 176;
pub static LOG_LOCAL7:  ::int_t = 184;
pub static LOG_EMERG:   ::int_t = 0;
pub static LOG_ALERT:   ::int_t = 1;
pub static LOG_CRIT:    ::int_t = 2;
pub static LOG_ERR:     ::int_t = 3;
pub static LOG_WARNING: ::int_t = 4;
pub static LOG_NOTICE:  ::int_t = 5;
pub static LOG_INFO:    ::int_t = 6;
pub static LOG_DEBUG:   ::int_t = 7;

pub fn LOG_MASK(pri: ::int_t) -> ::int_t {
    1 << pri as uint
}
