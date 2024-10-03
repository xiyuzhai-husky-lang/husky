use crate::*;
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
pub static mut __nested__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn nested() {
    let t = {
        1
    };
}
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
pub static mut __closure_inline__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn closure_inline() {
    let t = |x: i32|x + 1;
}
#[rustfmt::skip]
#[allow(non_upper_case_globals)]
pub static mut __closure_nested__ITEM_PATH_ID_INTERFACE: Option<__ItemPathIdInterface> = None;

#[rustfmt::skip]
pub fn closure_nested() {
    let t = |x: i32|( {
        x + 1
    });
}