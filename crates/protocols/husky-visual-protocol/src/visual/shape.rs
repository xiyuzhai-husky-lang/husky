use ordered_float::OrderedFloat;

use super::*;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy, Serialize, Deserialize, Hash)]
pub struct ShapeVisual(VisualId);

impl_visual_serde_id_from_to_for_sub_visual_id! { ShapeVisual }

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ShapeVisualData {
    LineSegment {
        start: Point,
        end: Point,
        stroke: Stroke,
    },
    Contour {
        points: Vec<Point>,
    },
}

#[derive(Default, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Point {
    pub x: OrderedFloat<f32>,
    pub y: OrderedFloat<f32>,
}

impl Into<(f32, f32)> for Point {
    fn into(self) -> (f32, f32) {
        (self.x.into(), self.y.into())
    }
}

impl std::fmt::Debug for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!(
            "Point {{ x: {}, y: {} }}",
            self.x.into_inner(),
            self.y.into_inner(),
        ))
    }
}

impl Point {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x: x.into(),
            y: y.into(),
        }
    }

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
        let a = *(((self.x + 1.0) - visual_rect.min.x) / (visual_rect.max.x - visual_rect.min.x));
        let x = rect.min.x + (rect.max.x - rect.min.x) * a;
        let b = *(((self.y + 1.0) - visual_rect.min.y) / (visual_rect.max.y - visual_rect.min.y));
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
        stroke: impl Into<Stroke>,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        Self(
            visual_synchrotron.alloc_visual(ShapeVisualData::LineSegment {
                start: start.into(),
                end: end.into(),
                stroke: stroke.into(),
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Stroke {
    pub width: OrderedFloat<f32>,
    pub color_class: Color,
}

impl Stroke {
    pub fn width(&self) -> f32 {
        self.width.into_inner()
    }
}

impl From<(f32, Color)> for Stroke {
    fn from(value: (f32, Color)) -> Self {
        let (width, color_class) = value;
        Self {
            width: width.into(),
            color_class,
        }
    }
}

#[cfg(feature = "egui")]
impl Into<egui::Stroke> for Stroke {
    fn into(self) -> egui::Stroke {
        use egui::Color32;

        egui::Stroke {
            width: self.width(),
            color: match self.color_class {
                Color::Red => Color32::RED,
                Color::LightYellow => todo!(),
                Color::Yellow => Color32::YELLOW,
                Color::Purple => todo!(),
            },
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Color {
    Red,
    LightYellow,
    Yellow,
    Purple,
}

impl Visual {
    pub fn new_line_segment(
        start: (f32, f32),
        end: (f32, f32),
        stroke: impl Into<Stroke>,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        ShapeVisual::new_line_segment(start, end, stroke, visual_synchrotron).into()
    }

    pub fn new_contour(
        points: impl Iterator<Item = (f32, f32)>,
        visual_synchrotron: &mut VisualSynchrotron,
    ) -> Self {
        ShapeVisual::new_contour(points, visual_synchrotron).into()
    }
}
