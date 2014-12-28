#[repr(C)]
#[deriving(Copy)]
pub struct FTW {
    pub base: ::int_t,
    pub level: ::int_t,
}
pub const FTW_F:   ::int_t = 0;
pub const FTW_D:   ::int_t = 1;
pub const FTW_DNR: ::int_t = 2;
pub const FTW_DP:  ::int_t = 5;
pub const FTW_NS:  ::int_t = 3;
pub const FTW_SL:  ::int_t = 4;
pub const FTW_SLN: ::int_t = 6;
pub const FTW_PHYS:  ::int_t = 1;
pub const FTW_MOUNT: ::int_t = 2;
pub const FTW_DEPTH: ::int_t = 8;
pub const FTW_CHDIR: ::int_t = 4;
