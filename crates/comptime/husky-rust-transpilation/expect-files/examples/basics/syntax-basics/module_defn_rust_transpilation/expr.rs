use crate::*;

#[rustfmt::skip]
pub fn nested() {
    let t = {
        1
    };
}

#[rustfmt::skip]
pub fn closure_inline() {
    let t = |x: i32|x + 1;
}

#[rustfmt::skip]
pub fn closure_nested() {
    let t = |x: i32|( {
        x + 1
    });
}