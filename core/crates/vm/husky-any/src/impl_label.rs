use crate::*;
use husky_trace_protocol::Label;

impl __HasStaticTypeInfo for Label {
    type __StaticSelf = Self;

    fn __static_type_name() -> std::borrow::Cow<'static, str> {
        "Label".into()
    }
}

impl<'eval> __AnyValue<'eval> for Label {
    fn __print_short(&self) -> String {
        todo!()
    }

    fn __to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn __short<'short>(&self) -> &dyn __AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }

    fn __static_ty() -> EntityRoutePtr {
        todo!()
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
