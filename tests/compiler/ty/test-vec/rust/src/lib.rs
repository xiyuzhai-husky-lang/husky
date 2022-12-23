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

pub(crate) fn test_pop_with<'eval>(__ctx: &dyn __EvalContext<'eval>) -> i32 {
    let mut v = vec![0, 1, 2, 4, 3];
    let b = v.pop_with_largest_opt_f32_copyable(
        ThickFp::__new_base(score as fn(i32) -> Option<f32>),
        __ctx,
    );
    assert!(b == Some(4));
    assert!(v == vec![0, 1, 2, 3]);
    return v.ilen();
}
pub(crate) fn score(a: i32) -> Option<f32> {
    return Some(a as f32);
}
