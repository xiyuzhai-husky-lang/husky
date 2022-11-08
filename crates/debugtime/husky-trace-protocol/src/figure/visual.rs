use super::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(tag = "kind")]
pub enum VisualData {
    BinaryImage28 {
        padded_rows: [u32; 30],
    },
    BinaryGrid28 {
        padded_rows: [u32; 31],
    },
    Primitive {
        value: PrimitiveValueData,
    },
    Contour {
        points: Vec<Point2dData>,
    },
    LineSegment {
        start: Point2dData,
        end: Point2dData,
    },
    Group(Vec<VisualData>),
}

pub enum VisualWorld {
    Primitive,
    Graphics2d,
    Graphics3d,
}

impl VisualData {
    pub fn void() -> VisualData {
        VisualData::Primitive { value: ().into() }
    }

    pub fn world(&self) -> VisualWorld {
        match self {
            VisualData::LineSegment { .. }
            | VisualData::Contour { .. }
            | VisualData::BinaryImage28 { .. }
            | VisualData::BinaryGrid28 { .. } => VisualWorld::Graphics2d,
            VisualData::Primitive { .. } => VisualWorld::Primitive,
            VisualData::Group(group) => group[0].world(),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub struct Point2dData {
    pub x: f32,
    pub y: f32,
}

impl Point2dData {
    pub fn from_ij28(i: usize, j: usize) -> Self {
        Point2dData {
            x: j as f32 + 1.0,
            y: 29.0 - i as f32,
        }
    }

    pub fn to(self, other: Point2dData) -> Point2dData {
        Point2dData {
            x: other.x - self.x,
            y: other.y - self.y,
        }
    }

    pub fn angle(self) -> f32 {
        self.y.atan2(self.x)
    }

    pub fn norm(self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Vector2dProps {
    x: f32,
    y: f32,
}
