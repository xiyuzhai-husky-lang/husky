use crate::*;
use serde::Serialize;
use serde_json::value::Value;
use word::{CustomIdentifier, IdentPairDict};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct XmlValue {
    pub tag_kind: XmlTagKind,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum XmlTagKind {
    Point2d,
    Contour,
    Arrow2d,
    LineSegment,
}

impl XmlTagKind {
    pub fn as_str(self) -> &'static str {
        match self {
            XmlTagKind::Point2d => "Point2d",
            XmlTagKind::Arrow2d => "Arrow2d",
            XmlTagKind::Contour => "Contour",
            XmlTagKind::LineSegment => "LineSegment",
        }
    }

    pub fn from_ident(ident: CustomIdentifier) -> Self {
        match ident.as_str() {
            "Point2d" => XmlTagKind::Point2d,
            "Contour" => XmlTagKind::Contour,
            "Arrow2d" => XmlTagKind::Arrow2d,
            "LineSegment" => XmlTagKind::LineSegment,
            _ => todo!(),
        }
    }
}
