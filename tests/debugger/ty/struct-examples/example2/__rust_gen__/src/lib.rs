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
    pub(crate) y: i32,
    pub(crate) z: i32,
}

impl A {
    pub(crate) fn __call__(x: i32, y: i32) -> Self {
        let z = x - 2;
        Self { x, y, z }
    }
    pub(crate) fn w<'eval>(&'eval self, __ctx: &dyn __EvalContext<'eval>) -> &'eval i32 {
        let __uid = entity_uid!(__ctx, "example2::A::w");
        if let Some(__result) = __ctx.opt_cached_lazy_field(self as *const _ as *const (), __uid) {
            return __result
                .unwrap()
                .downcast_eval_ref(&__registration__::__I32_VTABLE);
        }
        return __ctx
            .cache_lazy_field(
                self as *const _ as *const (),
                __uid,
                Ok(__Register::new_box(
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
        "example2::A".into()
    }
}

impl<'eval> __Registrable<'eval> for A {
    unsafe fn __to_register(self) -> __Register<'eval> {
        todo!()
    }
}

pub(crate) fn f1() -> A {
    return {
        let __this_x: i32 = 1;
        let __this_y: i32 = __this_x + 1;
        A::__call__(__this_x, __this_y)
    };
}

pub(crate) fn f3() -> () {
    let a = {
        let __this_x: i32 = 2;
        let __this_y: i32 = __this_x + 1;
        A::__call__(__this_x, __this_y)
    };
    assert!(a.get_x() == 2);
}
pub(crate) fn g1() -> i32 {
    let a = {
        let __this_x: i32 = 2;
        let __this_y: i32 = __this_x + 1;
        A::__call__(__this_x, __this_y)
    };
    return a.x;
}
