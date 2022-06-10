mod associated_traces;
mod factory;
mod query;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod variant;

pub use factory::{ProduceTrace, TraceFactory};
pub use query::*;
pub use variant::TraceVariant;

use feature::*;
use file::FilePtr;
use fold::Indent;
use husky_debugger_protocol::*;
use print_utils::p;
use semantics_eager::*;
use semantics_entity::*;
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, sync::Arc};
use text::{Text, TextRange};

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Clone)]
pub struct Trace {
    pub variant: TraceVariant<'static>,
    pub props: TraceProps,
    pub range: TextRange,
    pub file: FilePtr,
}

impl PartialEq for Trace {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Trace {}

impl Trace {
    pub(crate) fn new(
        parent: Option<TraceId>,
        indent: Indent,
        variant: TraceVariant<'static>,
        trace_allocator: &TraceFactory,
        text: &Text,
        compile_time_version: usize,
    ) -> Self {
        let id = trace_allocator.next_id();
        let (file, range) = variant.file_and_range();
        let reachable = variant.reachable();
        let has_subtraces = variant.has_subtraces(reachable);
        let lines = trace_allocator.lines(id, indent, &variant, text, parent.is_some());
        Self {
            props: TraceProps {
                id,
                opt_parent: parent,
                indent,
                compile_time_version,
                has_subtraces,
                reachable,
                lines,
                kind: variant.kind(),
            },
            variant,
            file,
            range,
        }
    }

    pub fn id(&self) -> TraceId {
        self.props.id
    }
}
