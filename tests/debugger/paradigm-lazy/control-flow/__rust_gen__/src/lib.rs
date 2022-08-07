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
