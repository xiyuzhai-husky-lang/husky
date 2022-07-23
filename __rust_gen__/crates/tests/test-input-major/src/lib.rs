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

pub(crate) fn find_connected_components(
    binary_image: &domains::ml::datasets::cv::mnist::BinaryImage28,
) -> i32 {
    let mut b = binary_image.clone();
    b[(0) as usize] = 1u32;
    return 1;
}
