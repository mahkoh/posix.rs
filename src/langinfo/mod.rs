pub use self::os::{CODESET};
pub use self::os::{D_T_FMT};
pub use self::os::{D_FMT};
pub use self::os::{T_FMT};
pub use self::os::{T_FMT_AMPM};
pub use self::os::{AM_STR};
pub use self::os::{PM_STR};
pub use self::os::{DAY_1};
pub use self::os::{DAY_2};
pub use self::os::{DAY_3};
pub use self::os::{DAY_4};
pub use self::os::{DAY_5};
pub use self::os::{DAY_6};
pub use self::os::{DAY_7};
pub use self::os::{ABDAY_1};
pub use self::os::{ABDAY_2};
pub use self::os::{ABDAY_3};
pub use self::os::{ABDAY_4};
pub use self::os::{ABDAY_5};
pub use self::os::{ABDAY_6};
pub use self::os::{ABDAY_7};
pub use self::os::{MON_1};
pub use self::os::{MON_2};
pub use self::os::{MON_3};
pub use self::os::{MON_4};
pub use self::os::{MON_5};
pub use self::os::{MON_6};
pub use self::os::{MON_7};
pub use self::os::{MON_8};
pub use self::os::{MON_9};
pub use self::os::{MON_10};
pub use self::os::{MON_11};
pub use self::os::{MON_12};
pub use self::os::{ABMON_1};
pub use self::os::{ABMON_2};
pub use self::os::{ABMON_3};
pub use self::os::{ABMON_4};
pub use self::os::{ABMON_5};
pub use self::os::{ABMON_6};
pub use self::os::{ABMON_7};
pub use self::os::{ABMON_8};
pub use self::os::{ABMON_9};
pub use self::os::{ABMON_10};
pub use self::os::{ABMON_11};
pub use self::os::{ABMON_12};
pub use self::os::{ERA};
pub use self::os::{ERA_D_FMT};
pub use self::os::{ERA_D_T_FMT};
pub use self::os::{ERA_T_FMT};
pub use self::os::{ALT_DIGITS};
pub use self::os::{RADIXCHAR};
pub use self::os::{THOUSEP};
pub use self::os::{YESEXPR};
pub use self::os::{NOEXPR};
pub use self::os::{CRNCYSTR};

#[cfg(target_os = "linux")]
#[path = "linux/mod.rs"]
mod os;

extern "C" {
    pub fn nl_langinfo(item: ::nl_types::nl_item) -> *mut ::char_t;
    pub fn nl_langinfo_l(item: ::nl_types::nl_item,
                         l: ::locale::locale_t) -> *mut ::char_t;
}
