use crate::*;
use check_utils::should_eq;
use print_utils::{msg_once, p, ps};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, PartialEq)]
#[serde(tag = "kind")]
pub enum VisualProps {
    BinaryImage28 {
        padded_rows: [u32; 30],
    },
    BinaryGrid28 {
        padded_rows: [u32; 31],
    },
    Primitive {
        value: CopyableValue,
    },
    Contour {
        points: Vec<Point2dProps>,
    },
    LineSegment {
        start: Point2dProps,
        end: Point2dProps,
    },
    Group(Vec<VisualProps>),
}

impl VisualProps {
    pub fn from_xml_value(xml_value: XmlValue) -> VisualProps {
        let mut data = xml_value.props.take_data();
        msg_once!("ad hoc");
        match xml_value.tag_kind.as_str() {
            "Contour" => {
                should_eq!(data.len(), 1);
                let (ident, value) = data.pop().unwrap();
                let points: Vec<Point2dProps> = serde_json::from_value(value).unwrap();
                VisualProps::Contour { points }
            }
            "LineSegment" => {
                should_eq!(data.len(), 2);
                // end
                let (ident, value) = data.pop().unwrap();
                should_eq!(ident.as_str(), "end");
                let end: Point2dProps = serde_json::from_value(value).unwrap();
                // start
                let (ident, value) = data.pop().unwrap();
                should_eq!(ident.as_str(), "start");
                let start: Point2dProps = serde_json::from_value(value).unwrap();
                VisualProps::LineSegment { start, end }
            }
            _ => todo!(),
        }
    }
}

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

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Point2dProps {
    pub x: f32,
    pub y: f32,
}

impl Point2dProps {
    pub fn from_ij28(i: usize, j: usize) -> Self {
        Point2dProps {
            x: j as f32 + 1.0,
            y: 29.0 - i as f32,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vector2dProps {
    x: f32,
    y: f32,
}
