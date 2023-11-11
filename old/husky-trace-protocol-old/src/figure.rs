mod canvas;
mod canvas_element;
mod client;
mod control;
mod graphics2d;
mod value;
mod visual;

pub use canvas::*;
pub use canvas_element::*;
pub use control::*;
pub use graphics2d::*;
pub use value::*;
pub use visual::*;

use super::*;
use husky_vm_primitive_value::PrimitiveValueData;

// impl From<GenericFigureCanvasData> for FigureCanvasData {
//     fn from(value: GenericFigureCanvasData) -> Self {
//         FigureCanvasData::Generic(value)
//     }
// }

// impl From<SpecificFigureCanvasData> for FigureCanvasData {
//     fn from(value: SpecificFigureCanvasData) -> Self {
//         FigureCanvasData::Specific(value)
//     }
// }

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
pub struct MutationFigureData {
    pub name: String,
    pub before: Option<FigureCanvasAtom>,
    pub after: Option<FigureCanvasAtom>,
    pub idx: usize,
}

#[derive(Debug, PartialEq, Clone, Copy)]
pub struct FigureCanvasData<'a> {
    pub generic: &'a GenericFigureCanvasData,
    pub specific: &'a SpecificFigureCanvasData,
}

impl<'a> ContainsImageLayers<'a> for FigureCanvasData<'a> {
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        self.specific.image_layers()
    }
}

impl<'a> ContainsShapes<'a> for FigureCanvasData<'a> {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        self.specific.shapes()
    }
}

impl<'a> ContainsImageLayers<'a> for &'a SpecificFigureCanvasData {
    fn image_layers(&self) -> Vec<&'a ImageLayerData> {
        match self {
            SpecificFigureCanvasData::Atom(atom) => atom.image_layers(),
            SpecificFigureCanvasData::Mutations { mutations: _ } => todo!(),
            _ => vec![],
        }
    }
}
impl<'a> ContainsShapes<'a> for &'a SpecificFigureCanvasData {
    fn shapes(&self) -> Vec<&'a Shape2dData> {
        match self {
            SpecificFigureCanvasData::Atom(atom) => atom.shapes(),
            SpecificFigureCanvasData::Mutations { mutations: _ } => todo!(),
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
