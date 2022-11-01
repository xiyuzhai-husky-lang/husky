use std::{convert::Infallible, ops::FromResidual};

use super::*;
use husky_signal::Signalable;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum FigureControlData {
    Unit,
    Mutations { opt_mutation_selection: Option<u8> },
}

impl Default for FigureControlData {
    fn default() -> Self {
        FigureControlData::Unit
    }
}

impl FromResidual<std::option::Option<Infallible>> for FigureControlData {
    fn from_residual(_residual: std::option::Option<Infallible>) -> Self {
        Default::default()
    }
}

impl Signalable for FigureControlData {}

impl FigureControlData {
    pub fn mutations_default(mutations_len: usize) -> Self {
        FigureControlData::Mutations {
            opt_mutation_selection: if mutations_len > 0 { Some(0) } else { None },
        }
    }
}
