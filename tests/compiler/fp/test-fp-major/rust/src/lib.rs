#![allow(warnings)]
pub mod __init__;
pub mod __registration__;
use __husky::root::*;

fn __input<'a, 'eval: 'a>(__ctx: &'a dyn __EvalContext<'eval>) -> &'a f32 {
    unsafe {
        __ctx
            .target_input()
            .downcast_temp_ref(&__registration__::__F32_VTABLE)
    }
}
pub(crate) fn try_fp<'eval>(__ctx: &dyn __EvalContext<'eval>) -> i32 {
    let f1 = ThickFp::__new_base(i32::sgn as fn(i32) -> i32);
    return f1.call1(1, __ctx);
}
