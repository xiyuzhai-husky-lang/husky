mod canvas_value;
mod control;
mod graphics2d;
mod visual;

pub use control::*;
pub use graphics2d::*;
pub use visual::*;

use super::*;
use husky_signal::Signalable;
use husky_vm_primitive_value::PrimitiveValueData;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
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
        partitioned_samples: Vec<(PartitionDefnData, Vec<(SampleId, Graphics2dCanvasData)>)>,
    },
    GenericF32 {
        partitioned_samples: Vec<(PartitionDefnData, Vec<(SampleId, f32)>)>,
    },
    GenericI32 {
        partitioned_samples: Vec<(PartitionDefnData, Vec<(SampleId, i32)>)>,
    },
    EvalError {
        message: String,
    },
}

impl Signalable for FigureCanvasData {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MutationFigureData {
    pub name: String,
    pub before: Option<FigureCanvasData>,
    pub after: FigureCanvasData,
    pub idx: usize,
}

#[derive(Debug, PartialEq)]
pub struct FigureCanvasDataItd {
    pub generic: &'static FigureCanvasData,
    pub specific: &'static FigureCanvasData,
}

impl Signalable for FigureCanvasDataItd {}

impl<'a> ContainsImageLayers<'a> for FigureCanvasDataItd {
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        self.specific.image_layers()
    }
}

impl<'a> ContainsShapes<'a> for FigureCanvasDataItd {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        self.specific.shapes()
    }
}

impl<'a> ContainsImageLayers<'a> for &'a FigureCanvasData {
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        match self {
            FigureCanvasData::Graphics2d { graphics2d_data } => graphics2d_data.image_layers(),
            FigureCanvasData::Mutations { mutations } => todo!(),
            FigureCanvasData::GenericGraphics2d {
                partitioned_samples,
            } => todo!(),
            _ => vec![],
        }
    }
}
impl<'a> ContainsShapes<'a> for &'a FigureCanvasData {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        match self {
            FigureCanvasData::Graphics2d { graphics2d_data } => graphics2d_data.shapes(),
            FigureCanvasData::Mutations { mutations } => todo!(),
            FigureCanvasData::GenericGraphics2d {
                partitioned_samples,
            } => todo!(),
            _ => vec![],
        }
    }
}

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
                VisualData::BinaryGrid28 { .. }
                | VisualData::Contour { .. }
                | VisualData::Group(_)
                | VisualData::LineSegment { .. } => shapes.push(visual_data.into()),
                VisualData::Primitive { .. } => panic!(),
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

    pub fn new_specific_primitive_group(_visuals: Vec<VisualData>) -> Self {
        Self::void()
    }

    pub fn void() -> Self {
        Self::Primitive {
            value: PrimitiveValueData::Void(()),
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
pub enum Plot2dKind {
    Scatter,
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct Point2dGroup {
    pub points: Vec<Point2dData>,
    pub color: Color,
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy, PartialEq)]
#[serde(rename_all = "lowercase")]
pub enum Color {
    Red,
    Yellow,
    Green,
    Blue,
}
