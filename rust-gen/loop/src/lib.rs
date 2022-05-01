
pub(crate) fn for_loop1() -> i32 {
    let mut b = 0i32;
    for i in (0i32 + 1)..5i32 {
        b += 1i32;
    }
    b
}

pub(crate) fn for_loop2() -> i32 {
    let mut b = 0i32;
    for i in 0..5i32 {
        b += 1i32;
    }
    b
}

pub(crate) fn for_loop3() -> i32 {
    let mut b = 0i32;
    for i in 0..5i32 {
        b += i;
    }
    b
}

pub(crate) fn forext_loop1() -> i32 {
    let mut b = 0i32;
    let mut i = 3i32;
    while i < 5i32 {
        b += 1i32;
        i += 1;
    }
    b
}

pub(crate) fn forext_loop2() -> i32 {
    let mut b = 0i32;
    let mut i = 3i32;
    while i > 0i32 {
        b += 1i32;
        i -= 1;
    }
    b
}

pub(crate) fn while_loop1() -> i32 {
    let mut b = 0i32;
    while b < 5i32 {
        b += 1i32;
    }
    b
}

pub(crate) fn while_loop2() -> i32 {
    let mut b = 5i32;
    while b != 0 {
        b -= 1i32;
    }
    b
}

pub(crate) fn while_loop3() -> i32 {
    let mut b = 5i32;
    while b < 3i32 {
        b -= 1i32;
    }
    b
}

pub(crate) fn do_while_loop1() -> i32 {
    let mut b = 0i32;
    loop {
        b += 1i32;
        if !(b < 5i32) {
            break;
        }
    }
    b
}

pub(crate) fn do_while_loop2() -> i32 {
    let mut b = 5i32;
    loop {
        b -= 1i32;
        if !(b != 0) {
            break;
        }
    }
    b
}

pub(crate) fn do_while_loop3() -> i32 {
    let mut b = 5i32;
    loop {
        b -= 1i32;
        if !(b < 3i32) {
            break;
        }
    }
    b
}

pub mod __init__ {
    pub fn link_entity_with_compiled(compile_time: &mut compile_time_db::HuskyLangCompileTime) {
        todo!()
    }
}
