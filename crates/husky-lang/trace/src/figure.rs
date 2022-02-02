use crate::*;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum FigureProps {
    Plot2d {
        plot_kind: Plot2dKind,
        groups: Vec<PointGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        image: Option<ImageProps>,
        shape_groups: Vec<ShapeGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct ImageProps {
    pub data: Vec<f32>,
    pub original_height: usize,
    pub original_width: usize,
}

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Plot2dKind {
    Scatter,
}

#[derive(Debug, Serialize, Clone)]
pub struct PointGroup {
    pub points: Vec<Point2d>,
    pub color: Color,
}

#[derive(Debug, Serialize, Clone)]
pub struct ShapeGroup {
    pub shapes: Vec<Shape>,
    pub line_width: f32,
    pub color: Color,
}

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "type")]
pub enum Shape {
    Arrow { from: Point2d, to: Point2d },
}

#[derive(Debug, Serialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}

#[derive(Debug, Serialize, Clone)]
pub struct Point2d {
    pub x: f32,
    pub y: f32,
}

#[derive(Debug, Serialize, Clone)]
pub struct Vector2d {
    x: f32,
    y: f32,
}
