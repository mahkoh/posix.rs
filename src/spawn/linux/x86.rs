#[repr(C)]
#[deriving(Copy)]
pub struct posix_spawnattr_t {
    __flags: [u16, ..1],
    __pgrp: [u32, ..1],
    __sd: [u32, ..32],
    __ss: [u32, ..32],
    __sp: [u32, ..1],
    __policy: [u32, ..1],
    __pad: [u32, ..16],
}
new!(posix_spawnattr_t);
#[repr(C)]
#[deriving(Copy)]
pub struct posix_spawn_file_actions_t {
    __allocated: [u32, ..1],
    __used: [u32, ..1],
    __actions: [u32, ..1],
    __pad: [u32, ..16],
}
new!(posix_spawn_file_actions_t);
pub const POSIX_SPAWN_RESETIDS: ::short_t = 0x01;
pub const POSIX_SPAWN_SETPGROUP: ::short_t = 0x02;
pub const POSIX_SPAWN_SETSCHEDPARAM: ::short_t = 0x10;
pub const POSIX_SPAWN_SETSCHEDULER: ::short_t = 0x20;
pub const POSIX_SPAWN_SETSIGDEF: ::short_t = 0x04;
pub const POSIX_SPAWN_SETSIGMASK: ::short_t = 0x08;
