mod graphics2d;

use crate::*;
use graphics2d::*;
use visual_syntax::VisualProps;
use vm::PrimitiveValue;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum FigureProps {
    Primitive {
        value: PrimitiveValue,
    },
    Plot2d {
        plot_kind: Plot2dKind,
        groups: Vec<PointGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        image: Option<ImageProps>,
        shape_groups: Vec<Shape2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
}

impl FigureProps {
    pub fn new_specific(visual_props: VisualProps) -> FigureProps {
        match visual_props {
            VisualProps::BinaryImage28 { padded_rows } => FigureProps::Graphics2d {
                image: Some(ImageProps::binary_image_28(&padded_rows)),
                shape_groups: Vec::new(),
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualProps::Primitive { value } => FigureProps::Primitive { value },
        }
    }

    pub fn void() -> FigureProps {
        Self::Primitive {
            value: PrimitiveValue::Void,
        }
    }
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
