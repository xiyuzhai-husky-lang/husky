use crate::*;

impl<'eval> AnyValue<'eval> for VisualProps {
    fn static_type_id() -> StaticTypeId {
        std::any::TypeId::of::<Self>().into()
    }

    fn static_type_name() -> std::borrow::Cow<'static, str> {
        "XmlValue".into()
    }

    fn to_json_value(&self) -> serde_json::value::Value {
        todo!()
    }
}
