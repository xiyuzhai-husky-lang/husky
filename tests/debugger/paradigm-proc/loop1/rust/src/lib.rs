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

pub(crate) fn for_loop1() -> i32 {
    let mut b = 0;
    for i in (0 + 1)..5 {
        b += 1;
    }
    return b;
}

pub(crate) fn for_loop2() -> i32 {
    let mut b = 0;
    for i in 0..5 {
        b += 1;
    }
    return b;
}

pub(crate) fn for_loop3() -> i32 {
    let mut b = 0;
    for i in 0..5 {
        b += i;
    }
    return b;
}

pub(crate) fn for_loop4() -> i32 {
    let mut b = 0;
    for i in 0..5 {
        let x = 1;
        b += i;
    }
    return b;
}

pub(crate) fn forext_loop1() -> i32 {
    let mut b = 0;
    let mut i = 3;
    while i < 5 {
        b += 1;
        i += 1;
    }
    return b;
}

pub(crate) fn forext_loop2() -> i32 {
    let mut b = 0;
    let mut i = 3;
    while i > 0 {
        b += 1;
        i -= 1;
    }
    return b;
}

pub(crate) fn forext_loop3() -> i32 {
    let mut b = 0;
    let mut i = 3;
    while i > 0 {
        let x = 1;
        b += 1;
        i -= 1;
    }
    return b;
}

pub(crate) fn while_loop1() -> i32 {
    let mut b = 0;
    while b < 5 {
        b += 1;
    }
    return b;
}

pub(crate) fn while_loop2() -> i32 {
    let mut b = 5;
    while b != 0 {
        b -= 1;
    }
    return b;
}

pub(crate) fn while_loop3() -> i32 {
    let mut b = 5;
    while b < 3 {
        b -= 1;
    }
    return b;
}

pub(crate) fn while_loop4() -> i32 {
    let mut b = 5;
    while b < 3 {
        let x = 2;
        b -= 1;
    }
    return b;
}

pub(crate) fn do_while_loop1() -> i32 {
    let mut b = 0;
    loop {
        b += 1;
        if !(b < 5) {
            break;
        }
    }
    return b;
}

pub(crate) fn do_while_loop2() -> i32 {
    let mut b = 5;
    loop {
        b -= 1;
        if !(b != 0) {
            break;
        }
    }
    return b;
}

pub(crate) fn do_while_loop3() -> i32 {
    let mut b = 5;
    loop {
        b -= 1;
        if !(b < 3) {
            break;
        }
    }
    return b;
}

pub(crate) fn do_while_loop4() -> i32 {
    let mut b = 5;
    loop {
        let mut y = -1;
        b -= 1;
        if !(b < 3) {
            break;
        }
    }
    return b;
}
