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

pub(crate) fn f() -> i32 {
    let mut v = Vec::<i32>::__call__(vec![]);
    let u = v.clone();
    v.push(2);
    assert!(v[(0) as usize] == 2);
    return v.ilen();
}

pub(crate) fn change_element() -> i32 {
    let mut v = Vec::<i32>::__call__(vec![]);
    v.push(2);
    v[(0) as usize] = 3;
    return v[(0) as usize];
}
