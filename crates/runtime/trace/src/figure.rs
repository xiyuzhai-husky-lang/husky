mod control;
mod graphics2d;

use compile_time_db::HuskyLangCompileTime;
pub use control::*;
pub use graphics2d::*;

use crate::*;
use map_collect::MapCollect;
use visual_runtime::RuntimeVisualizer;
use visual_syntax::{Point2dProps, VisualProps};
use vm::{CopyableValue, MutationData, VMRuntimeResult};
use word::Identifier;

#[derive(Debug, Serialize, Clone)]
#[serde(tag = "kind")]
pub enum FigureProps {
    Primitive {
        value: CopyableValue,
    },
    Plot2d {
        plot_kind: Plot2dKind,
        point_groups: Vec<Point2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        image_layers: Vec<ImageLayerProps>,
        shapes: Vec<Shape2dProps>,
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
        compile_time: &HuskyLangCompileTime,
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
            before: if let Some(before) = mutation_data.before.as_ref() {
                Some(FigureProps::new_specific(
                    visualizer.visualize(compile_time, before.any_ref()),
                ))
            } else {
                None
            },
            after: FigureProps::new_specific(
                visualizer.visualize(compile_time, mutation_data.after.any_ref()),
            ),
            idx,
        }
    }
}

impl FigureProps {
    pub fn new_specific(visual_props: VisualProps) -> Self {
        match visual_props {
            VisualProps::BinaryImage28 { padded_rows } => FigureProps::Graphics2d {
                image_layers: vec![ImageLayerProps::binary_image_28(&padded_rows)],
                shapes: Vec::new(),
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualProps::Primitive { value } => FigureProps::Primitive { value },
            VisualProps::BinaryGrid28 { ref padded_rows } => FigureProps::Graphics2d {
                image_layers: vec![],
                shapes: vec![Shape2dProps::laser_grid28(padded_rows)],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualProps::Contour { points } => todo!(),
        }
    }

    pub fn void() -> Self {
        Self::Primitive {
            value: CopyableValue::Void(()),
        }
    }
}

#[derive(Debug, Serialize, Clone, Copy)]
pub enum Plot2dKind {
    Scatter,
}

#[derive(Debug, Serialize, Clone)]
pub struct Point2dGroup {
    pub points: Vec<Point2dProps>,
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
