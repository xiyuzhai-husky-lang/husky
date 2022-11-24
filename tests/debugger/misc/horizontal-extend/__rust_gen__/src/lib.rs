#![allow(warnings)]
pub mod __init__;
pub mod __registration__;
use __husky::root::*;

fn __input<'a, 'eval: 'a>(
    __ctx: &'a dyn __EvalContext<'eval>,
) -> &'a cv::datasets::mnist::BinaryImage28 {
    unsafe {
        __ctx
            .target_input()
            .downcast_temp_ref(&__registration__::__BINARY_IMAGE_28_VTABLE)
    }
}

pub(crate) fn horizontal_extend(a: u32, x: u32) -> u32 {
    let mut y = a & (x | (x << 1) | (x >> 1));
    let mut z = a & (y | (y << 1) | (y >> 1));
    while z != y {
        y = z;
        z = a & (y | (y << 1) | (y >> 1));
    }
    return y;
}
