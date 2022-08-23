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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(C)]

pub(crate) enum A {
    Haha,
}

impl From<i32> for A {
    fn from(__raw: i32) -> Self {
        match __raw {
            0 => A::Haha,
            _ => panic!(),
        }
    }
}
impl __StaticInfo for A {
    type __StaticSelf = A;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "test_enum::A".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}
