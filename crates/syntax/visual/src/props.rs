use check_utils::should_eq;
use print_utils::{p, ps};
use serde::{Deserialize, Serialize};
use vm::{CopyableValue, XmlValue};

#[derive(Debug, Clone, Serialize)]
#[serde(tag = "kind")]
pub enum VisualProps {
    BinaryImage28 { padded_rows: [u32; 30] },
    BinaryGrid28 { padded_rows: [u32; 31] },
    Primitive { value: CopyableValue },
    Contour { points: Vec<Point2dProps> },
}

impl VisualProps {
    pub fn from_xml_value(xml_value: XmlValue) -> VisualProps {
        let mut data = xml_value.props.take_data();
        should_eq!(data.len(), 1);
        let (ident, value) = data.pop().unwrap();
        match xml_value.name.as_str() {
            "Contour" => {
                let points: Vec<Point2dProps> = serde_json::from_value(value).unwrap();
                VisualProps::Contour { points }
            }
            _ => todo!(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
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
