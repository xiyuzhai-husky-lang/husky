use cyclic_slice::CyclicSlice;

use super::*;

impl<'eval, 'a: 'eval, 'b: 'eval, T: AnyValue<'a>> AnyValue<'eval> for CyclicSlice<'b, T> {
    fn static_type_id() -> StaticTypeId {
        StaticTypeId::CyclicSlice(Box::new(T::static_type_id()))
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "CyclicSlice".into()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        serde_json::value::Value::Array(self.iter().map(|elem| elem.to_json_value()).collect())
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }
}
