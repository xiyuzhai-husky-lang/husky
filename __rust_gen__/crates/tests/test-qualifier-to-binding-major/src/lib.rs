#![allow(warnings)]
pub mod __init__;
use __husky::*;

// ad hoc
fn __input<'a, 'eval: 'a>(
    __ctx: &'a __EvalContext<'eval>,
) -> &'a domains::ml::datasets::cv::mnist::BinaryImage28 {
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
}

impl __HasStaticTypeInfo for A {
    type __StaticSelf = A;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "test_qualifier_to_binding_major::A".into()
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
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> __EntityRoutePtr {
        __ty_route_from_static_binded::<Self>("test_qualifier_to_binding_major::A")
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
#[derive(Debug, Clone, PartialEq, __Serialize)]
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

impl __HasStaticTypeInfo for B {
    type __StaticSelf = B;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "test_qualifier_to_binding_major::B".into()
    }
}

impl<'eval> __AnyValue<'eval> for B {
    fn __print_short(&self) -> String {
        "{ ... }".to_owned()
    }

    fn __to_json_value(&self) -> __JsonValue {
        serde_json::value::to_value(self).unwrap()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> __EntityRoutePtr {
        __ty_route_from_static_binded::<Self>("test_qualifier_to_binding_major::B")
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
pub(crate) fn take_copyable_eval_ref<'eval>(x: &'eval i32) -> i32 {
    return 1;
}
