use crate::*;
use std::borrow::Cow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct __CallFormValue {
    pub opt_linkage: Option<LinkageDeprecated>,
}

impl __HasStaticTypeInfo for __CallFormValue {
    type __StaticSelf = Self;

    fn __static_type_name() -> Cow<'static, str> {
        todo!()
    }
}

impl<'eval> __AnyValue<'eval> for __CallFormValue {
    fn __into_temp_value<'temp>(self) -> __TempValue<'temp, 'eval>
    where
        'eval: 'temp,
    {
        todo!()
    }

    fn __into_eval_value(self) -> __EvalValue<'eval>
    where
        Self: 'eval,
    {
        todo!()
    }

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
        todo!()
    }

    fn __static_ty() -> EntityRoutePtr {
        todo!()
    }
}
