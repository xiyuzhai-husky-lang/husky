// use std::ffi::{c_char, CString};

// use crate::*;

// #[derive(Debug)]
// struct A {}

// impl __StaticInfo for A {
//     type __StaticSelf = A;

//     fn __static_typename() -> std::borrow::Cow<'static, str> {
//         "A".into()
//     }
// }

// impl __Registrable for A {
//     unsafe fn __to_register__<'eval>(self) -> __Register<'eval> {
//         __Register::new_box(self, &A_VTABLE)
//     }

//     fn __copy__(&self) -> Self {
//         panic!()
//     }
// }

use crate::__Registrable;

#[test]
fn downcast_works_for_bool() {
    let mut ra = {
        let a = true;
        unsafe { a.__to_register__() }
    };
    let b: bool = ra.downcast_bool();
    assert!(b)
}

#[test]
fn downcast_works_for_i32() {
    let mut ra = {
        let a = 1i32;
        unsafe { a.__to_register__() }
    };
    let b: i32 = ra.downcast_i32();
}

// #[test]
// fn downcast_works2() {
//     let a = A {};
//     let mut ra = __Register::new_box(a, &A_VTABLE);
//     let b: A = ra.downcast();
// }

// #[test]
// #[cfg(feature = "extra")]
// fn downcast_works_after_into_eval() {
//     let mut ra = {
//         let a = A {};
//         unsafe { a.__to_register__() }
//     }
//     .into_eval();
//     let b: A = ra.downcast();
// }
