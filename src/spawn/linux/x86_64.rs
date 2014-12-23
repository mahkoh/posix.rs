#[repr(C)]
pub struct posix_spawnattr_t {
    _data: [u64, ..42],
}

new!(posix_spawnattr_t);

#[repr(C)]
pub struct posix_spawn_file_actions_t {
    _data: [u64, ..10],
}

new!(posix_spawn_file_actions_t);

pub const POSIX_SPAWN_RESETIDS:      ::short_t = 1;
pub const POSIX_SPAWN_SETPGROUP:     ::short_t = 2;
pub const POSIX_SPAWN_SETSCHEDPARAM: ::short_t = 16;
pub const POSIX_SPAWN_SETSCHEDULER:  ::short_t = 32;
pub const POSIX_SPAWN_SETSIGDEF:     ::short_t = 4;
pub const POSIX_SPAWN_SETSIGMASK:    ::short_t = 8;
