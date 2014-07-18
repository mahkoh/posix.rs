#[repr(C)]
pub struct posix_spawnattr_t {
    data: [u64, ..42],
}

#[repr(C)]
pub struct posix_spawn_file_actions_t {
    data: [u64, ..10],
}

pub static POSIX_SPAWN_RESETIDS:      ::short_t = 1;
pub static POSIX_SPAWN_SETPGROUP:     ::short_t = 2;
pub static POSIX_SPAWN_SETSCHEDPARAM: ::short_t = 16;
pub static POSIX_SPAWN_SETSCHEDULER:  ::short_t = 32;
pub static POSIX_SPAWN_SETSIGDEF:     ::short_t = 4;
pub static POSIX_SPAWN_SETSIGMASK:    ::short_t = 8;
