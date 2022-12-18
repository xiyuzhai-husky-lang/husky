#![allow(warnings)]
pub mod __init__;
pub mod __registration__;
use __husky::root::*;

fn __input<'a, 'eval: 'a>(
    __ctx: &'a dyn __EvalContext<'eval>,
) -> &'a domains::ml::datasets::cv::mnist::BinaryImage28 {
    unsafe {
        __ctx
            .target_input()
            .downcast_temp_ref(&__registration__::__BINARY_IMAGE_28_VTABLE)
    }
}
pub(crate) mod connected_component;
pub(crate) mod eight;
pub(crate) mod fermi;
pub(crate) mod five;
pub(crate) mod four;
pub(crate) mod geom2d;
pub(crate) mod line_segment_sketch;
pub(crate) mod major;
pub(crate) mod nine;
pub(crate) mod one;
pub(crate) mod raw_contour;
pub(crate) mod seven;
pub(crate) mod six;
pub(crate) mod three;
pub(crate) mod two;
pub(crate) mod zero;
