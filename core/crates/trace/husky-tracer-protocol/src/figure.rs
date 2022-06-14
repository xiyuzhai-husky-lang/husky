mod control;
mod graphics2d;
mod visual;

pub use control::*;
pub use graphics2d::*;
pub use visual::*;

use super::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
#[serde(tag = "kind")]
pub enum FigureCanvasData {
    Primitive {
        value: PrimitiveValueData,
    },
    Plot2d {
        plot_kind: Plot2dKind,
        point_groups: Vec<Point2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        image_layers: Vec<ImageLayerData>,
        shapes: Vec<Shape2dData>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Mutations {
        mutations: Vec<MutationFigureData>,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct MutationFigureData {
    pub name: String,
    pub before: Option<FigureCanvasData>,
    pub after: FigureCanvasData,
    pub idx: usize,
}

// impl<'eval> MutationFigureProps {
//     pub fn new(
//         db: &dyn RuntimeVisualizerQueryGroup,
//          ,
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
//                 Some(FigureCanvasData::new_specific(visualizer.visualize(
//                     db,
//                     before.any_ref(),
//                     verbose,
//                 )))
//             } else {
//                 None
//             },
//             after: FigureCanvasData::new_specific(visualizer.visualize(
//                 db,
//                 mutation_data.after.any_ref(),
//                 verbose,
//             )),
//             idx,
//         }
//     }
// }

impl FigureCanvasData {
    pub fn new_specific(visual_props: VisualData) -> Self {
        match visual_props {
            VisualData::BinaryImage28 { padded_rows } => FigureCanvasData::Graphics2d {
                image_layers: vec![ImageLayerData::binary_image28(&padded_rows)],
                shapes: Vec::new(),
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualData::Primitive { value } => FigureCanvasData::Primitive { value },
            VisualData::BinaryGrid28 { ref padded_rows } => FigureCanvasData::Graphics2d {
                image_layers: vec![],
                shapes: vec![Shape2dData::laser_grid28(padded_rows)],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualData::Contour { points } => FigureCanvasData::Graphics2d {
                image_layers: vec![],
                shapes: vec![Shape2dData::Contour { points }],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
            VisualData::Group(mut visuals) => {
                if visuals.len() == 0 {
                    return FigureCanvasData::void();
                }
                if visuals.len() == 1 {
                    return Self::new_specific(visuals.pop().unwrap());
                }
                match visuals[0] {
                    VisualData::BinaryImage28 { .. }
                    | VisualData::BinaryGrid28 { .. }
                    | VisualData::Contour { .. }
                    | VisualData::LineSegment { .. } => {
                        Self::new_specific_graphics2d_group(visuals)
                    }
                    VisualData::Primitive { .. } => Self::new_specific_primitive_group(visuals),
                    VisualData::Group(_) => todo!(),
                }
            }
            VisualData::LineSegment { start, end } => FigureCanvasData::Graphics2d {
                image_layers: vec![],
                shapes: vec![Shape2dData::LineSegment { start, end }],
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
        }
    }

    pub fn new_specific_graphics2d_group(visuals: Vec<VisualData>) -> Self {
        let mut image_layers = Vec::new();
        let mut shapes = Vec::new();
        for visual in visuals {
            match visual {
                VisualData::BinaryImage28 { ref padded_rows } => {
                    image_layers.push(ImageLayerData::binary_image28(padded_rows))
                }
                VisualData::BinaryGrid28 { ref padded_rows } => {
                    shapes.push(Shape2dData::laser_grid28(padded_rows))
                }
                VisualData::Primitive { value } => todo!(),
                VisualData::Contour { points } => shapes.push(Shape2dData::Contour { points }),
                VisualData::Group(_) => todo!(),
                VisualData::LineSegment { start, end } => {
                    shapes.push(Shape2dData::LineSegment { start, end })
                }
            }
        }
        FigureCanvasData::Graphics2d {
            image_layers,
            shapes,
            xrange: (0.0, 28.0),
            yrange: (0.0, 28.0),
        }
    }

    pub fn new_specific_primitive_group(visuals: Vec<VisualData>) -> Self {
        Self::void()
    }

    pub fn void() -> Self {
        Self::Primitive {
            value: PrimitiveValueData::Void(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub enum Plot2dKind {
    Scatter,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Point2dGroup {
    pub points: Vec<Point2dData>,
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
