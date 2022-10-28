mod control;
mod graphics2d;
mod value;
mod visual;

pub use control::*;
pub use graphics2d::*;
pub use value::*;
pub use visual::*;

use super::*;
use husky_signal::Signalable;
use husky_vm_primitive_value::PrimitiveValueData;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub enum FigureCanvasData {
    Generic(GenericFigureCanvasData),
    Specific(SpecificFigureCanvasData),
}

impl FigureCanvasData {
    pub fn generic(&self) -> Option<&GenericFigureCanvasData> {
        match self {
            FigureCanvasData::Generic(generic) => Some(generic),
            FigureCanvasData::Specific(_) => None,
        }
    }
    pub fn specific(&self) -> Option<&SpecificFigureCanvasData> {
        match self {
            FigureCanvasData::Generic(_) => None,
            FigureCanvasData::Specific(specific) => Some(specific),
        }
    }
}

impl Signalable for FigureCanvasData {}

impl From<GenericFigureCanvasData> for FigureCanvasData {
    fn from(value: GenericFigureCanvasData) -> Self {
        FigureCanvasData::Generic(value)
    }
}

impl From<SpecificFigureCanvasData> for FigureCanvasData {
    fn from(value: SpecificFigureCanvasData) -> Self {
        FigureCanvasData::Specific(value)
    }
}

impl FigureCanvasData {
    pub fn void() -> Self {
        GenericFigureCanvasData::Unit.into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum SpecificFigureCanvasData {
    Primitive {
        value: PrimitiveValueData,
    },
    Graphics2d {
        graphics2d_data: Graphics2dCanvasData,
    },
    Mutations {
        mutations: Vec<MutationFigureData>,
    },
    EvalError {
        message: String,
    },
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum GenericFigureCanvasData {
    Unit,
    Plot2d {
        plot_kind: Plot2dKind,
        point_groups: Vec<Point2dGroup>,
        xrange: (f32, f32),
        yrange: (f32, f32),
    },
    Graphics2d {
        graphics2d_data: Graphics2dCanvasData,
    },
    GenericGraphics2d {
        partitioned_samples: Vec<(Partition, Vec<(SampleId, Graphics2dCanvasData)>)>,
    },
    GenericF32 {
        partitioned_samples: Vec<(Partition, Vec<(SampleId, f32)>)>,
    },
    GenericI32 {
        partitioned_samples: Vec<(Partition, Vec<(SampleId, i32)>)>,
    },
    EvalError {
        message: String,
    },
}

impl Signalable for GenericFigureCanvasData {}
impl Signalable for SpecificFigureCanvasData {}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MutationFigureData {
    pub name: String,
    pub before: Option<SpecificFigureCanvasData>,
    pub after: SpecificFigureCanvasData,
    pub idx: usize,
}

#[derive(Debug, PartialEq)]
pub struct FigureCanvasDataItd {
    pub generic: &'static GenericFigureCanvasData,
    pub specific: &'static SpecificFigureCanvasData,
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

impl<'a> ContainsImageLayers<'a> for &'a SpecificFigureCanvasData {
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        match self {
            SpecificFigureCanvasData::Graphics2d { graphics2d_data } => {
                graphics2d_data.image_layers()
            }
            SpecificFigureCanvasData::Mutations { mutations } => todo!(),
            _ => vec![],
        }
    }
}
impl<'a> ContainsShapes<'a> for &'a SpecificFigureCanvasData {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        match self {
            SpecificFigureCanvasData::Graphics2d { graphics2d_data } => graphics2d_data.shapes(),
            SpecificFigureCanvasData::Mutations { mutations } => todo!(),
            _ => vec![],
        }
    }
}
impl GenericFigureCanvasData {}
impl SpecificFigureCanvasData {
    pub fn new(visual_data: VisualData) -> Self {
        log::info!("deprecated");
        match visual_data {
            VisualData::BinaryImage28 { padded_rows } => SpecificFigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![ImageLayerData::binary_image28(&padded_rows)],
                    shapes: Vec::new(),
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
            VisualData::Primitive { value } => SpecificFigureCanvasData::Primitive { value },
            VisualData::BinaryGrid28 { ref padded_rows } => SpecificFigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::laser_grid28(padded_rows)],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
            VisualData::Contour { points } => SpecificFigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::Contour { points }],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
            VisualData::Group(mut visuals) => {
                if visuals.len() == 0 {
                    return SpecificFigureCanvasData::void();
                }
                if visuals.len() == 1 {
                    return Self::new(visuals.pop().unwrap());
                }
                match visuals[0].world() {
                    VisualWorld::Primitive => Self::new_specific_primitive_group(visuals),
                    VisualWorld::Graphics2d => Self::new_graphics2d_group(visuals),
                    VisualWorld::Graphics3d => todo!(),
                }
            }
            VisualData::LineSegment { start, end } => SpecificFigureCanvasData::Graphics2d {
                graphics2d_data: Graphics2dCanvasData {
                    image_layers: vec![],
                    shapes: vec![Shape2dData::LineSegment { start, end }],
                    xrange: (0.0, 28.0),
                    yrange: (0.0, 28.0),
                },
            },
        }
    }

    pub fn new_graphics2d_group(visuals: Vec<VisualData>) -> Self {
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
        SpecificFigureCanvasData::Graphics2d {
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
