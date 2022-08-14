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

pub(crate) fn try_vec_opt() -> Vec<Option<i32>> {
    let mut v = Vec::<Option<i32>>::__call__(vec![]);
    v.push(Some(1));
    assert!(v[(0) as usize] == Some(1));
    return v;
}
