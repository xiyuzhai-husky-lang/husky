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

#[derive(Debug, Clone, PartialEq, __Serialize)]
pub(crate) struct A {
    pub(crate) x: i32,
}

impl A {
    pub(crate) fn __call__(x: i32) -> Self {
        Self { x }
    }
pub(crate) fn y<'eval>(&'eval self, __ctx: &__EvalContext<'eval>) -> &'eval i32 {
    let __uid = entity_uid!(__ctx, "test_struct_example1::A::y");
    if let Some(__result) = __opt_cached_lazy_field(__ctx, self, __uid) {
        return __result.unwrap();
    }
        return __cache_lazy_field(
        __ctx,
        self,
        __uid,
        Ok((self.x + 1).__into_eval_value())
    ).unwrap();

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

impl __HasStaticTypeInfo for A {
    type __StaticSelf = A;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "test_struct_example1::A".into()
    }
}

impl<'eval> __AnyValue<'eval> for A {
    fn __print_short(&self) -> String {
        "{ ... }".to_owned()
    }

    fn __to_json_value(&self) -> __JsonValue {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short {
        self
    }

    fn __static_ty() -> __EntityRoutePtr {
        __ty_route_from_static_binded::<Self>("test_struct_example1::A")
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        __EvalValue::Owned(__OwnedValue::new(self))
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        todo!()
    }
}

pub(crate) fn f1() -> A {
    return A::__call__(1)
}

pub(crate) fn f2() -> A {
    let mut a = A::__call__(2);
    a.x = 1;
    return a
}

pub(crate) fn f3() -> () {
    let a = A::__call__(2);
    assert!(a.get_x() == 2);
}
pub(crate) fn g1() -> i32 {
    let a = A::__call__(2);
    return a.x;

}
