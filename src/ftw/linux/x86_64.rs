#[repr(C)]
pub struct FTW {
    pub base: ::int_t,
    pub level: ::int_t,
}

pub static FTW_F:   ::int_t = 0;
pub static FTW_D:   ::int_t = 1;
pub static FTW_DNR: ::int_t = 2;
pub static FTW_DP:  ::int_t = 5;
pub static FTW_NS:  ::int_t = 3;
pub static FTW_SL:  ::int_t = 4;
pub static FTW_SLN: ::int_t = 6;

pub static FTW_PHYS:  ::int_t = 1;
pub static FTW_MOUNT: ::int_t = 2;
pub static FTW_DEPTH: ::int_t = 8;
pub static FTW_CHDIR: ::int_t = 4;
