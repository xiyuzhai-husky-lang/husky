mod atom;
mod generic;
mod specific;

pub use atom::*;
pub use generic::*;
pub use specific::*;

use super::*;

// #[derive(Debug, Serialize, Deserialize, Clone, PartialEq)]
// pub enum FigureCanvasData {
//     Generic(GenericFigureCanvasData),
//     Specific(SpecificFigureCanvasData),
// }
