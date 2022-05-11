mod control;
mod graphics2d;

pub use control::*;

use crate::*;
use graphics2d::*;
use map_collect::MapCollect;
use visual_runtime::RuntimeVisualizer;
use visual_syntax::VisualProps;
use vm::{MutationData, PrimitiveValue};
use word::Identifier;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum FigureProps {
    Primitive {
        value: PrimitiveValue,
    },
    Plot2d {
        plot_kind: Plot2dKind,
        point_groups: Vec<Point2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        image: Option<ImageProps>,
        shape_groups: Vec<Shape2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Mutations {
        mutations: Vec<MutationFigureProps>,
    },
}

#[derive(Debug, Serialize, Clone)]
pub struct MutationFigureProps {
    pub name: String,
    pub before: Option<FigureProps>,
    pub after: FigureProps,
    pub idx: usize,
}

impl<'eval> MutationFigureProps {
    pub fn new(
        text: &Text,
        visualizer: &RuntimeVisualizer,
        mutation_data: &MutationData<'eval>,
        idx: usize,
    ) -> Self {
        MutationFigureProps {
            name: match mutation_data.kind {
                vm::MutationDataKind::Exec { range } => text.ranged(range),
                vm::MutationDataKind::Block { varname, .. } => varname.as_str().to_string(),
            },
            before: mutation_data
                .before
                .as_ref()
                .map(|before| FigureProps::new_specific(visualizer.visualize(before.any_ref()))),
            after: FigureProps::new_specific(visualizer.visualize(mutation_data.after.any_ref())),
            idx,
        }
    }
}

impl FigureProps {
    pub fn new_specific(visual_props: VisualProps) -> Self {
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

    pub fn void() -> Self {
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
pub struct Point2dGroup {
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
