use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum ImageLayerProps {
    Colored { pixels: Vec<Vec<(u8, u8, u8)>> },
    Binary28 { rows: Box<[u32; 28]> },
}

impl ImageLayerProps {
    pub fn binary_image28(padded_rows: &[u32; 30]) -> Self {
        Self::Binary28 {
            rows: Box::new(padded_rows[1..29].try_into().unwrap()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum Shape2dProps {
    Arrow2d {
        from: Point2dProps,
        to: Point2dProps,
    },
    Point2d {
        point: Point2dProps,
    },
    Contour {
        points: Vec<Point2dProps>,
    },
    LineSegment {
        start: Point2dProps,
        end: Point2dProps,
    },
    Group {
        shapes: Vec<Shape2dProps>,
        line_width: f32,
    },
}

impl Shape2dProps {
    pub fn laser_grid28(padded_rows: &[u32; 31]) -> Self {
        let mut shapes = Vec::<Shape2dProps>::new();
        for i in 0..29 {
            for j in 0..29 {
                let value = (padded_rows[i + 1] >> (31 - (j + 1))) & 1;
                if value != 0 {
                    shapes.push(Shape2dProps::Point2d {
                        point: Point2dProps::from_ij28(i, j),
                    })
                }
            }
        }
        Shape2dProps::Group {
            shapes,
            line_width: 2.0,
        }
    }
}
