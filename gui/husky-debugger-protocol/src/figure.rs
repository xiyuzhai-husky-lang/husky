mod control;
mod graphics2d;
mod visual;

pub use control::*;
pub use graphics2d::*;
pub use visual::*;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum FigureProps {
    Primitive {
        value: PrimitiveValueProps,
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

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MutationFigureProps {
    pub name: String,
    pub before: Option<FigureProps>,
    pub after: FigureProps,
    pub idx: usize,
}

// impl<'eval> MutationFigureProps {
//     pub fn new(
//         db: &dyn RuntimeVisualizerQueryGroup,
//         text: &Text,
//         visualizer: &RuntimeVisualizer,
//         mutation_data: &MutationData<'eval>,
//         idx: usize,
//         verbose: bool,
//     ) -> Self {
//         MutationFigureProps {
//             name: match mutation_data.kind {
//                 vm::MutationDataKind::Exec { range } => text.ranged(range),
//                 vm::MutationDataKind::Block { varname, .. } => varname.as_str().to_string(),
//             },
//             before: if let Some(before) = mutation_data.before.as_ref() {
//                 Some(FigureProps::new_specific(visualizer.visualize(
//                     db,
//                     before.any_ref(),
//                     verbose,
//                 )))
//             } else {
//                 None
//             },
//             after: FigureProps::new_specific(visualizer.visualize(
//                 db,
//                 mutation_data.after.any_ref(),
//                 verbose,
//             )),
//             idx,
//         }
//     }
// }

impl FigureProps {
    pub fn new_specific(visual_props: VisualProps) -> Self {
        match visual_props {
            VisualProps::BinaryImage28 { padded_rows } => FigureProps::Graphics2d {
                image_layers: vec![ImageLayerProps::binary_image28(&padded_rows)],
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
            VisualProps::Contour { points } => FigureProps::Graphics2d {
                image_layers: vec![],
                shapes: vec![Shape2dProps::Contour { points }],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualProps::Group(mut visuals) => {
                if visuals.len() == 0 {
                    return FigureProps::void();
                }
                if visuals.len() == 1 {
                    return Self::new_specific(visuals.pop().unwrap());
                }
                match visuals[0] {
                    VisualProps::BinaryImage28 { .. }
                    | VisualProps::BinaryGrid28 { .. }
                    | VisualProps::Contour { .. }
                    | VisualProps::LineSegment { .. } => {
                        Self::new_specific_graphics2d_group(visuals)
                    }
                    VisualProps::Primitive { .. } => Self::new_specific_primitive_group(visuals),
                    VisualProps::Group(_) => todo!(),
                }
            }
            VisualProps::LineSegment { start, end } => FigureProps::Graphics2d {
                image_layers: vec![],
                shapes: vec![Shape2dProps::LineSegment { start, end }],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
        }
    }

    pub fn new_specific_graphics2d_group(visuals: Vec<VisualProps>) -> Self {
        let mut image_layers = Vec::new();
        let mut shapes = Vec::new();
        for visual in visuals {
            match visual {
                VisualProps::BinaryImage28 { ref padded_rows } => {
                    image_layers.push(ImageLayerProps::binary_image28(padded_rows))
                }
                VisualProps::BinaryGrid28 { ref padded_rows } => {
                    shapes.push(Shape2dProps::laser_grid28(padded_rows))
                }
                VisualProps::Primitive { value } => todo!(),
                VisualProps::Contour { points } => shapes.push(Shape2dProps::Contour { points }),
                VisualProps::Group(_) => todo!(),
                VisualProps::LineSegment { start, end } => {
                    shapes.push(Shape2dProps::LineSegment { start, end })
                }
            }
        }
        FigureProps::Graphics2d {
            image_layers,
            shapes,
            xrange: (0.0, 28.0),
            yrange: (0.0, 28.0),
        }
    }

    pub fn new_specific_primitive_group(visuals: Vec<VisualProps>) -> Self {
        Self::void()
    }

    pub fn void() -> Self {
        Self::Primitive {
            value: PrimitiveValueProps::Void(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Plot2dKind {
    Scatter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point2dGroup {
    pub points: Vec<Point2dProps>,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}
