use std::{
    convert::Infallible,
    ops::{FromResidual, Try},
};

use super::*;
use sycamore::prelude::Signalable;

#[derive(Debug, Default, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub struct FigureControlData {
    pub opt_mutation_selection: Option<u8>,
}

impl FromResidual<std::option::Option<Infallible>> for FigureControlData {
    fn from_residual(residual: std::option::Option<Infallible>) -> Self {
        Self::default()
    }
}
impl Signalable for FigureControlData {}

impl FigureControlData {
    // pub fn loop_default(loop_trace: &Trace) -> Self {

    // }

    pub fn mutations_default(mutations_len: usize) -> Self {
        FigureControlData {
            opt_mutation_selection: if mutations_len > 0 { Some(0) } else { None },
        }
    }
}
