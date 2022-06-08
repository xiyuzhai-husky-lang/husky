use vm::MemberValue;

use crate::*;

#[test]
fn test_i32_as_usize() {
    let a = (-1i32) as usize; // this will not fail
    p!(a);
    // let b: usize = (-1i32).try_into().unwrap(); // this will fail
}
