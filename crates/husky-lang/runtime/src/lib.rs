#![allow(warnings)]

mod error;
mod tests;

use std::borrow::Cow;

pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};

use trace::{FigureProps, Trace};
use vm::{run, AnyValueDyn, Instruction};

#[derive(Debug, Default)]
pub struct HuskyLangRuntime {}

impl HuskyLangRuntime {
    pub fn change_text(&self) {}

    pub fn root_traces(&self) -> Vec<Trace> {
        todo!()
    }

    pub fn subtraces(&self, id: usize) -> Vec<Trace> {
        trace::mock::subtraces(id)
    }

    pub fn figure(&self, id: usize) -> Option<FigureProps> {
        trace::mock::figure(id)
    }

    pub fn activate(&self, id: usize) {}

    pub fn toggle_expansion(&self, id: usize) {}
}
