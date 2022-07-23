#![allow(warnings)]
pub mod __init__;
use __husky::*;

// ad hoc
fn __input<'a, 'eval: 'a>(
    __ctx: &'a __EvalContext<'eval>,
) -> &'a domains::ml::datasets::cv::mnist::BinaryImage28 {
    unsafe { __evaluator(__ctx) }
        .eval_input
        .any_ref()
        .__downcast_ref()
}

pub(crate) fn add(x: i32, y: i32) -> i32 {
    return x + y;
}

pub(crate) fn bitor_assign() -> bool {
    let mut b = 0u32;
    b |= 1u32;
    return b == 1u32;
}

pub(crate) fn bitand_assign() -> bool {
    let mut b = 0u32;
    b &= 1u32;
    return b == 0u32;
}
