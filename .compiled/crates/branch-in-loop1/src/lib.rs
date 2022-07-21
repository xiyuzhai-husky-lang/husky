#![allow(warnings)]
pub mod __init__;
use __husky_root::*;


// ad hoc
fn __input<'a, 'eval:'a>(__ctx: &'a __EvalContext<'eval>) -> &'a domains::ml::datasets::cv::mnist::BinaryImage28 {
    unsafe { __evaluator(__ctx) }
        .eval_input
        .any_ref()
        .__downcast_ref()
}


pub(crate) fn f1() -> i32 {
    let mut a = 0;
    for i in 0..5 {
        a += i;
        if i > 2 {
            break;
        }
    }
    return a
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
    return a
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
    return a
}
