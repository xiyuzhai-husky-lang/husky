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

pub(crate) fn find_connected_components(binary_image: &cv::datasets::mnist::BinaryImage28) -> i32 {
    let mut b = binary_image.clone();
    b[(0) as usize] = 1u32;
    return 1;
}
