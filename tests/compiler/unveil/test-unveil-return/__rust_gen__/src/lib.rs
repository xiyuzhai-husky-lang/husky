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
pub(crate) fn some_i32() -> Option<i32> {
    return Some(1);
}
pub(crate) fn try_unveil() -> Option<i32> {
    if let Some(result) = some_i32() {
        return Some(result);
    }
    return Some(1);
}
