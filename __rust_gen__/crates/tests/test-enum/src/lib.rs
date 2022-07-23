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

#[derive(Debug, Clone, Copy, PartialEq, Eq, __Serialize)]
pub(crate) enum A {
    Haha,
}

impl __HasStaticTypeInfo for A {
    type __StaticSelf = A;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "test_enum::A".into()
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
        __ty_route_from_static_binded::<Self>("test_enum::A")
    }

    fn __into_eval_value(self) -> __EvalValue<'eval> {
        todo!()
    }

    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        todo!()
    }
}
