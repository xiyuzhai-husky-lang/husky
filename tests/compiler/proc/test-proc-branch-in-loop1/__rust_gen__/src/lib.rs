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

pub(crate) fn f1() -> i32 {
    let mut a = 0;
    for i in 0..5 {
        a += i;
        if i > 2 {
            break;
        }
    }
    return a;
}

pub(crate) fn f2() -> i32 {
    let mut a = 0;
    for i in 0..5 {
        a += i;
        if i > 2 {
            break;
        }
        a += 1;
    }
    return a;
}

pub(crate) fn f3() -> i32 {
    let mut a = 0;
    for i in 0..5 {
        a += i;
        if i > 3 {
            break;
        }
        if i > 2 {
            break;
        }
        a += 1;
    }
    return a;
}
