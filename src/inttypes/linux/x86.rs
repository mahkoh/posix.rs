#[repr(C)]
#[derive(Copy)]
pub struct imaxdiv_t {
    pub quot: ::longlong_t,
    pub rem: ::longlong_t,
}
new!(imaxdiv_t);
