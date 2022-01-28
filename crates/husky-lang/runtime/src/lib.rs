#![allow(warnings)]

mod error;
mod session;
mod tests;

use std::borrow::Cow;

pub use error::{RuntimeError, RuntimeResult, RuntimeResultArc};

use trace::Trace;
use vm::{run, AnyValueDyn, Instruction};

#[derive(Debug, Default)]
pub struct HuskyLangRuntime {}

impl HuskyLangRuntime {
    pub fn change_text(&self) {}

    pub fn main_trace(&self) {}

    pub fn root_traces(&self) -> Cow<'static, [Trace]> {
        Cow::Borrowed(trace::mock::ROOT_TRACES)
    }
}
