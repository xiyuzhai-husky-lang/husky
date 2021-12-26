use crate::*;

#[test]
fn test_add_i32() {
    let a = 1;
    let b = 2;
    let c = call!(pure add_i32(&a, &b): i32);
    assert_eq!(c, 3);
}

#[test]
fn test_sub_i32() {
    let a = 1;
    let b = 2;
    let c = call!(pure sub_i32(&a, &b): i32);
    assert_eq!(c, -1);
}
