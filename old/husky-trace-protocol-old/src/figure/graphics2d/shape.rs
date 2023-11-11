use super::*;

pub trait ContainsShapes<'a> {
    fn shapes(&self) -> Vec<&'a Shape2dData>;
}

impl<'a, T> ContainsShapes<'a> for std::rc::Rc<T>
where
    T: ContainsShapes<'a>,
{
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        <T as ContainsShapes>::shapes(self)
    }
}

impl<'a, T> ContainsShapes<'a> for std::sync::Arc<T>
where
    T: ContainsShapes<'a>,
{
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        <T as ContainsShapes>::shapes(self)
    }
}

impl<'a, T, S> ContainsShapes<'a> for (T, S)
where
    T: ContainsShapes<'a>,
    S: ContainsShapes<'a>,
{
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        let mut shapes = self.0.shapes();
        shapes.extend(self.1.shapes().into_iter());
        shapes
    }
}

impl<'a, T> ContainsShapes<'a> for [T]
where
    T: ContainsShapes<'a>,
{
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        let mut shapes = vec![];

        for item in self.iter() {
            shapes.extend(item.shapes().into_iter());
        }
        shapes
    }
}

impl<'a, T> ContainsShapes<'a> for Vec<T>
where
    T: ContainsShapes<'a>,
{
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        let mut shapes = vec![];

        for item in self.iter() {
            shapes.extend(item.shapes().into_iter());
        }
        shapes
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
        Shape2dData::Group { shapes }
    }
}

impl From<VisualData> for Shape2dData {
    fn from(visual_data: VisualData) -> Self {
        match visual_data {
            VisualData::BinaryImage28 { .. } => panic!(),
            VisualData::Primitive { .. } => todo!(),
            VisualData::BinaryGrid28 { ref padded_rows } => Shape2dData::laser_grid28(padded_rows),
            VisualData::Contour { points } => Shape2dData::Contour { points },
            VisualData::Group(group) => Shape2dData::Group {
                shapes: group
                    .into_iter()
                    .map(|visual_data| visual_data.into())
                    .collect(),
            },
            VisualData::LineSegment { start, end } => Shape2dData::LineSegment { start, end },
        }
    }
}
