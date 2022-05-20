use super::*;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum ImageProps {
    Colored { pixels: Vec<Vec<(u8, u8, u8)>> },
    Binary28 { rows: Box<[u32; 28]> },
}

impl ImageProps {
    pub fn binary_image_28(padded_rows: &[u32; 30]) -> Self {
        Self::Binary28 {
            rows: Box::new(padded_rows[1..29].try_into().unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Clone)]
pub struct Shape2dGroupProps {
    pub shapes: Vec<Shape2dProps>,
    pub line_width: f32,
    pub color: Color,
    pub kind: Shape2dKind,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Shape2dKind {
    Arrow2d,
    Point2d,
}

#[derive(Debug, Serialize, Clone)]
#[serde(untagged)]
pub enum Shape2dProps {
    Arrow2d { from: Point2d, to: Point2d },
    Point2d { point: Point2d },
}

impl Shape2dGroupProps {
    pub fn laser_grid28(padded_rows: &[u32; 31]) -> Self {
        let mut shapes = Vec::<Shape2dProps>::new();
        for i in 0..29 {
            for j in 0..29 {
                let value = (padded_rows[i + 1] >> (31 - (j + 1))) & 1;
                shapes.push(Shape2dProps::Point2d {
                    point: Point2d::from_ij28(i, j),
                })
            }
        }
        Self {
            shapes,
            line_width: 2.0,
            color: Color::Red,
            kind: Shape2dKind::Point2d,
        }
    }
}
