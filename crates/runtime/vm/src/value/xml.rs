use crate::*;
use serde::Serialize;
use serde_json::value::Value;
use word::{CustomIdentifier, IdentPairDict};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlValue {
    pub name: String,
    pub props: IdentPairDict<Value>,
}

impl Serialize for XmlValue {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        todo!()
    }
}

impl<'eval> AnyValue<'eval> for XmlValue {
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
