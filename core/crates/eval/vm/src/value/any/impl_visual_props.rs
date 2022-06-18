use crate::*;

impl<'eval> AnyValue<'eval> for VisualData {
    fn static_type_id() -> StaticTypeId {
        std::any::TypeId::of::<Self>().into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "XmlValue".into()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }

    fn short<'short>(&self) -> &dyn AnyValueDyn<'short>
    where
        'eval: 'short,
    {
        self
    }
}
