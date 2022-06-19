mod control;
mod graphics2d;
mod visual;

pub use control::*;
pub use graphics2d::*;
use sycamore::prelude::Signalable;
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
        graphics2d_data: Graphics2dCanvasData,
    },
    Mutations {
        mutations: Vec<MutationFigureData>,
    },
    GenericGraphics2d {
        partitioned_samples: Vec<(PartitionDefnData, Vec<Graphics2dCanvasData>)>,
    },
    GenericF32 {
        partitioned_samples: Vec<(PartitionDefnData, Vec<f32>)>,
    },
    GenericI32 {
        partitioned_samples: Vec<(PartitionDefnData, Vec<i32>)>,
    },
}

impl Signalable for FigureCanvasData {}

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
    pub fn new_specific(visual_data: VisualData) -> Self {
        log::info!("deprecated");
        match visual_data {
            VisualData::BinaryImage28 { padded_rows } => FigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![ImageLayerData::binary_image28(&padded_rows)],
                    shapes: Vec::new(),
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
            VisualData::Primitive { value } => FigureCanvasData::Primitive { value },
            VisualData::BinaryGrid28 { ref padded_rows } => FigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::laser_grid28(padded_rows)],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
            VisualData::Contour { points } => FigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::Contour { points }],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
            VisualData::Group(mut visuals) => {
                if visuals.len() == 0 {
                    return FigureCanvasData::void();
                }
                if visuals.len() == 1 {
                    return Self::new_specific(visuals.pop().unwrap());
                }
                match visuals[0].world() {
                    VisualWorld::Primitive => Self::new_specific_primitive_group(visuals),
                    VisualWorld::Graphics2d => Self::new_specific_graphics2d_group(visuals),
                    VisualWorld::Graphics3d => todo!(),
                }
            }
            VisualData::LineSegment { start, end } => FigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::LineSegment { start, end }],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
        }
    }

    pub fn new_specific_graphics2d_group(visuals: Vec<VisualData>) -> Self {
        let mut image_layers = Vec::new();
        let mut shapes = Vec::new();
        for visual_data in visuals {
            match visual_data {
                VisualData::BinaryImage28 { ref padded_rows } => {
                    image_layers.push(ImageLayerData::binary_image28(padded_rows))
                }
                VisualData::Primitive { value } => todo!(),
                VisualData::BinaryGrid28 { .. }
                | VisualData::Contour { .. }
                | VisualData::Group(_)
                | VisualData::LineSegment { .. } => shapes.push(visual_data.into()),
            }
        }
        FigureCanvasData::Graphics2d {
            graphics2d_data: Graphics2dCanvasData {
                image_layers,
                shapes,
                xrange: (0.0, 28.0),
                yrange: (0.0, 28.0),
            },
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

    // pub fn error() -> Self {
    //     Self::Primitive {
    //         value: PrimitiveValueData::Void(()),
    //     }
    // }

    // pub fn new_generic(
    //     partitioned_visuals: Vec<(PartitionDefnData, Vec<FigureCanvasData>)>,
    // ) -> Self {
    //     for (partition, visuals) in &partitioned_visuals {
    //         if visuals.len() > 0 {
    //             match &visuals[0] {
    //                 FigureCanvasData::Primitive { value } => match value {
    //                     PrimitiveValueData::I32(_) => todo!(),
    //                     PrimitiveValueData::F32(_) => todo!(),
    //                     PrimitiveValueData::B32(_) => todo!(),
    //                     PrimitiveValueData::B64(_) => todo!(),
    //                     PrimitiveValueData::Bool(_) => todo!(),
    //                     PrimitiveValueData::Void(_) => todo!(),
    //                 },
    //                 FigureCanvasData::Graphics2d { graphics2d_data } => {
    //                     return Self::new_generic_graphics2d(partitioned_visuals)
    //                 }
    //                 _ => panic!(),
    //             }
    //         }
    //     }
    //     panic!()
    // }

    // pub fn new_generic_graphics2d(
    //     partitioned_visuals: Vec<(PartitionDefnData, Vec<FigureCanvasData>)>,
    // ) -> Self {
    //     FigureCanvasData::GenericGraphics2d {
    //         partitioned_samples: partitioned_visuals
    //             .into_iter()
    //             .map(|(partition, visuals)| {
    //                 (
    //                     partition,
    //                     visuals
    //                         .into_iter()
    //                         .map(|visual_data| match visual_data {
    //                             FigureCanvasData::Graphics2d { graphics2d_data } => graphics2d_data,
    //                             _ => {
    //                                 println!("{:?}", visual_data);
    //                                 panic!()
    //                             }
    //                         })
    //                         .collect(),
    //                 )
    //             })
    //             .collect(),
    //     }
    // }

    fn new_generic_f32(
        partitioned_visuals: Vec<(PartitionDefnData, Vec<FigureCanvasData>)>,
    ) -> Self {
        FigureCanvasData::GenericF32 {
            partitioned_samples: partitioned_visuals
                .into_iter()
                .map(|(partition, visuals)| {
                    (
                        partition,
                        visuals
                            .into_iter()
                            .map(|visual_data| match visual_data {
                                FigureCanvasData::Primitive { value } => match value {
                                    PrimitiveValueData::F32(f) => f,
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            })
                            .collect(),
                    )
                })
                .collect(),
        }
    }

    fn new_generic_i32(
        partitioned_visuals: Vec<(PartitionDefnData, Vec<FigureCanvasData>)>,
    ) -> Self {
        FigureCanvasData::GenericI32 {
            partitioned_samples: partitioned_visuals
                .into_iter()
                .map(|(partition, visuals)| {
                    (
                        partition,
                        visuals
                            .into_iter()
                            .map(|visual_data| match visual_data {
                                FigureCanvasData::Primitive { value } => match value {
                                    PrimitiveValueData::I32(i) => i,
                                    _ => panic!(),
                                },
                                _ => panic!(),
                            })
                            .collect(),
                    )
                })
                .collect(),
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
