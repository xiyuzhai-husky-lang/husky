use std::ops::Deref;

fn f(x: &mut i32) {}
fn ff(x: <&mut &mut i32 as std::ops::Deref>::Target) {}

#[test]
fn test_ff() {
    let mut x = 1;
    f(&mut x);
    ff(&mut x);
}
