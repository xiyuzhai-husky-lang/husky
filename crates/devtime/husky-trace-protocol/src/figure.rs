mod canvas_element;
mod client;
mod control;
mod graphics2d;
mod value;
mod visual;

pub use canvas_element::*;
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
        GenericFigureCanvasData::None.into()
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum SpecificFigureCanvasData {
    Atom(FigureCanvasAtom),
    Mutations { mutations: Vec<MutationFigureData> },
    EvalError { message: String },
}

impl SpecificFigureCanvasData {
    pub fn new_atom(visual_data: VisualData) -> Self {
        SpecificFigureCanvasData::Atom(FigureCanvasAtom::new(visual_data))
    }
}

impl Default for SpecificFigureCanvasData {
    fn default() -> Self {
        SpecificFigureCanvasData::Atom(Default::default())
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
#[serde(tag = "kind")]
pub enum GenericFigureCanvasData {
    None,
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
    pub before: Option<FigureCanvasAtom>,
    pub after: FigureCanvasAtom,
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
            SpecificFigureCanvasData::Atom(atom) => atom.image_layers(),
            SpecificFigureCanvasData::Mutations { mutations } => todo!(),
            _ => vec![],
        }
    }
}
impl<'a> ContainsShapes<'a> for &'a SpecificFigureCanvasData {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        match self {
            SpecificFigureCanvasData::Atom(atom) => atom.shapes(),
            SpecificFigureCanvasData::Mutations { mutations } => todo!(),
            _ => vec![],
        }
    }
}
impl GenericFigureCanvasData {}

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
