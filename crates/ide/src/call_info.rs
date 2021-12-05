//! This module provides primitives for tracking the information about a call site.

use either::Either;
use hir::{HirDisplay, Semantics};
use ide_db::{active_parameter::callable_for_token, base_db::FilePosition};
use stdx::format_to;
use syntax::{Direction, TextRange, TextSize};

use crate::RootDatabase;

/// Contains information about a call site. Specifically the
/// `FunctionSignature`and current parameter.
#[derive(Debug)]
pub struct CallInfo {
    pub doc: Option<String>,
    pub signature: String,
    pub active_parameter: Option<usize>,
    parameters: Vec<TextRange>,
}

impl CallInfo {
    pub fn parameter_labels(&self) -> impl Iterator<Item = &str> + '_ {
        self.parameters.iter().map(move |&it| &self.signature[it])
    }

    pub fn parameter_ranges(&self) -> &[TextRange] {
        &self.parameters
    }

    fn push_param(&mut self, param: &str) {
        if !self.signature.ends_with('(') {
            self.signature.push_str(", ");
        }
        let start = TextSize::of(&self.signature);
        self.signature.push_str(param);
        let end = TextSize::of(&self.signature);
        self.parameters.push(TextRange::new(start, end))
    }
}

/// Computes parameter information for the given call expression.
pub(crate) fn call_info(db: &RootDatabase, position: FilePosition) -> Option<CallInfo> {
    todo!()
}
