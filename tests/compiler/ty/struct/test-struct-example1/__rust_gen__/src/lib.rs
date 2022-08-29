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
    pub(crate) fn y<'eval>(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval i32 {
        let __uid = entity_uid!(__ctx, "test_struct_example1::A::y");
        if let Some(__result) =
            __ctx.opt_cached_lazy_field(self as *const _ as *const std::ffi::c_void, __uid)
        {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__I32_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const std::ffi::c_void,
                __uid,
                Ok(__Register::new_box::<i32>(
                    self.x + 1,
                    &__registration__::__I32_VTABLE,
                )),
            )
            .unwrap()
            .downcast_eval_ref(&__registration__::__I32_VTABLE);
    }
    pub(crate) fn get_x(&self) -> i32 {
        return self.x;
    }

    pub(crate) fn get_x_plus_constant(&self) -> i32 {
        let c = 2;
        return self.x + c;
    }

    pub(crate) fn get_x_squared(&self) -> i32 {
        return self.x * self.x;
    }
}

impl __StaticInfo for A {
    type __StaticSelf = A;

    fn __static_typename() -> std::borrow::Cow<'static, str> {
        "test_struct_example1::A".into()
    }

    unsafe fn __transmute_static(self) -> Self::__StaticSelf {
        std::mem::transmute(self)
    }
}

pub(crate) fn f1() -> A {
    return A::__call__(1);
}

pub(crate) fn f2() -> A {
    let mut a = A::__call__(2);
    a.x = 1;
    return a;
}

pub(crate) fn f3() -> () {
    let a = A::__call__(2);
    assert!(a.get_x() == 2);
}
pub(crate) fn g1() -> i32 {
    let a = A::__call__(2);
    return a.x;
}
