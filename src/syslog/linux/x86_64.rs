pub const LOG_PID:     ::int_t = 1;
pub const LOG_CONS:    ::int_t = 2;
pub const LOG_NDELAY:  ::int_t = 8;
pub const LOG_ODELAY:  ::int_t = 4;
pub const LOG_NOWAIT:  ::int_t = 16;
pub const LOG_KERN:    ::int_t = 0;
pub const LOG_USER:    ::int_t = 8;
pub const LOG_MAIL:    ::int_t = 16;
pub const LOG_NEWS:    ::int_t = 56;
pub const LOG_UUCP:    ::int_t = 64;
pub const LOG_DAEMON:  ::int_t = 24;
pub const LOG_AUTH:    ::int_t = 32;
pub const LOG_CRON:    ::int_t = 72;
pub const LOG_LPR:     ::int_t = 48;
pub const LOG_LOCAL0:  ::int_t = 128;
pub const LOG_LOCAL1:  ::int_t = 136;
pub const LOG_LOCAL2:  ::int_t = 144;
pub const LOG_LOCAL3:  ::int_t = 152;
pub const LOG_LOCAL4:  ::int_t = 160;
pub const LOG_LOCAL5:  ::int_t = 168;
pub const LOG_LOCAL6:  ::int_t = 176;
pub const LOG_LOCAL7:  ::int_t = 184;
pub const LOG_EMERG:   ::int_t = 0;
pub const LOG_ALERT:   ::int_t = 1;
pub const LOG_CRIT:    ::int_t = 2;
pub const LOG_ERR:     ::int_t = 3;
pub const LOG_WARNING: ::int_t = 4;
pub const LOG_NOTICE:  ::int_t = 5;
pub const LOG_INFO:    ::int_t = 6;
pub const LOG_DEBUG:   ::int_t = 7;

pub fn LOG_MASK(pri: ::int_t) -> ::int_t {
    1 << pri as usize
}
