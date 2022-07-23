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

pub(crate) fn try_vec() -> i32 {
    let a = vec![1, 2, 3, 4, 5, 6, 7];
    return a.ilen();
}
