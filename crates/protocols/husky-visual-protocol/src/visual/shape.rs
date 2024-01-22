use ordered_float::OrderedFloat;

use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct ShapeVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { ShapeVisual }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeVisualData {
    LineSegment { start: Point, end: Point },
    Contour { points: Vec<Point> },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
}

impl Point {
    pub fn splat(v: f32) -> Point {
        Point {
            x: v.into(),
            y: v.into(),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct VisualRect {
    /// One of the corners of the rectangle, usually the left top one.
    pub min: Point,

    /// The other corner, opposing [`Self::min`]. Usually the right bottom one.
    pub max: Point,
}

impl VisualRect {
    pub fn mnist() -> Self {
        Self {
            min: Point::default(),
            max: Point::splat(28.0),
        }
    }
}

/// # egui support
#[cfg(feature = "egui")]
impl Point {
    pub fn to_screen(self, visual_rect: VisualRect, rect: egui::Rect) -> egui::Pos2 {
        let a = (self.x - visual_rect.min.x) / (visual_rect.max.x - visual_rect.min.x);
        let a = a.into_inner();
        let x = rect.min.x + (rect.max.x - rect.min.x) * a;
        let b = (self.y - visual_rect.min.y) / (visual_rect.max.y - visual_rect.min.y);
        let b = b.into_inner();
        let y = rect.max.y + (rect.min.y - rect.max.y) * b;
        egui::Pos2 { x, y }
    }
}

impl From<(f32, f32)> for Point {
    fn from(point: (f32, f32)) -> Self {
        Point {
            x: point.0.into(),
            y: point.1.into(),
        }
    }
}

impl ShapeVisual {
    pub fn new_line_segment(
        start: (f32, f32),
        end: (f32, f32),
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        Self(
            visual_synchrotron.alloc_visual(ShapeVisualData::LineSegment {
                start: start.into(),
                end: end.into(),
            }),
        )
    }

    pub fn new_contour(
        points: impl Iterator<Item = (f32, f32)>,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        Self(
            visual_synchrotron.alloc_visual(ShapeVisualData::Contour {
                points: points
                    .map(|(x, y)| Point {
                        x: x.into(),
                        y: y.into(),
                    })
                    .collect(),
            }),
        )
    }

    pub fn data<'a>(self, visual_synchrotron: &'a VisualSynchrotron) -> &'a ShapeVisualData {
        let VisualData::Shape(data) = self.0.data(visual_synchrotron) else {
            unreachable!()
        };
        data
    }
}

impl Visual {
    pub fn new_line_segment(
        start: (f32, f32),
        end: (f32, f32),
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        ShapeVisual::new_line_segment(start, end, visual_synchrotron).into()
    }

    pub fn new_contour(
        points: impl Iterator<Item = (f32, f32)>,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        ShapeVisual::new_contour(points, visual_synchrotron).into()
    }
}
