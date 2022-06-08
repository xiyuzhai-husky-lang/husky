use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum VisualProps {
    BinaryImage28 {
        padded_rows: [u32; 30],
    },
    BinaryGrid28 {
        padded_rows: [u32; 31],
    },
    Primitive {
        value: PrimitiveValueProps,
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
