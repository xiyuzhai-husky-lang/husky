use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum ImageLayerData {
    Colored { pixels: Vec<Vec<(u8, u8, u8)>> },
    Binary28 { rows: Box<[u32; 28]> },
}

impl ImageLayerData {
    pub fn binary_image28(padded_rows: &[u32; 30]) -> Self {
        Self::Binary28 {
            rows: Box::new(padded_rows[1..29].try_into().unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum Shape2dData {
    Arrow2d {
        from: Point2dData,
        to: Point2dData,
    },
    Point2d {
        point: Point2dData,
    },
    Contour {
        points: Vec<Point2dData>,
    },
    LineSegment {
        start: Point2dData,
        end: Point2dData,
    },
    Group {
        shapes: Vec<Shape2dData>,
        line_width: f32,
    },
}

impl Shape2dData {
    pub fn laser_grid28(padded_rows: &[u32; 31]) -> Self {
        let mut shapes = Vec::<Shape2dData>::new();
        for i in 0..29 {
            for j in 0..29 {
                let value = (padded_rows[i + 1] >> (31 - (j + 1))) & 1;
                if value != 0 {
                    shapes.push(Shape2dData::Point2d {
                        point: Point2dData::from_ij28(i, j),
                    })
                }
            }
        }
        Shape2dData::Group {
            shapes,
            line_width: 2.0,
        }
    }
}
