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
pub struct Shape2dGroup {
    pub shapes: Vec<Shape2d>,
    pub line_width: f32,
    pub color: Color,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum Shape2d {
    Arrow { from: Point2d, to: Point2d },
}

impl Shape2dGroup {
    pub fn laser_grid28(padded_rows: &[u32; 31]) -> Shape2dGroup {
        todo!()
    }
}
