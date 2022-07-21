#![allow(warnings)]
pub mod __init__;
use __husky_root::*;

// ad hoc
fn __input<'a, 'eval: 'a>(
    __ctx: &'a __EvalContext<'eval>,
) -> &'a domains::ml::datasets::cv::mnist::BinaryImage28 {
    unsafe { __evaluator(__ctx) }
        .eval_input
        .any_ref()
        .__downcast_ref()
}

pub(crate) fn f() -> i32 {
    let mut v = Vec::<i32>::__call__(vec![]);
    let u = v.clone();
    v.push(2);
    assert!(v[(0) as usize] == 2);
    return v.ilen();
}

pub(crate) fn change_element() -> i32 {
    let mut v = Vec::<i32>::__call__(vec![]);
    v.push(2);
    v[(0) as usize] = 3;
    return v[(0) as usize];
}
