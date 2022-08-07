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
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct A {
    pub(crate) x: i32,
}

impl A {
    pub(crate) fn __call__(x: i32) -> Self {
        Self { x }
    }
}

impl __StaticInfo for A {
    type __StaticSelf = A;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "major::A".into()
    }
}

impl<'eval> __Registrable<'eval> for A {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
#[derive(Debug, Clone, PartialEq)]
pub(crate) struct B {
    pub(crate) x: i32,
    pub(crate) a: A,
    pub(crate) y: i32,
}

impl B {
    pub(crate) fn __call__(x: i32, a: A) -> Self {
        let y = x + 1;
        Self { x, a, y }
    }
    pub(crate) fn clone_a(&self) -> A {
        return self.a.clone();
    }
}

impl __StaticInfo for B {
    type __StaticSelf = B;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "major::B".into()
    }
}

impl<'eval> __Registrable<'eval> for B {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}
pub(crate) fn take_copyable_eval_ref<'eval>(x: &'eval i32) -> i32 {
    return 1;
}
