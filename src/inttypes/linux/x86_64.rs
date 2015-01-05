#[repr(C)]
#[derive(Copy)]
pub struct imaxdiv_t {
    pub quot: ::long_t,
    pub rem: ::long_t,
}
new!(imaxdiv_t);
