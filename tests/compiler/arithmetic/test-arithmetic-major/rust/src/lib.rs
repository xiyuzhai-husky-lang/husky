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
