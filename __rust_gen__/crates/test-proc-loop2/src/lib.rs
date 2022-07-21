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

pub(crate) fn for_loop4() -> i32 {
    let mut b = 0;
    for i in 0..5 {
        let x = 1;
        b += i;
    }
    return b;
}
