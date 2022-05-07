mod associated_traces;
mod factory;
mod figure;
mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod token;
mod variant;

pub use factory::{CreateTrace, TraceFactory, TraceId};
use feature::*;
pub use figure::*;
use file::FilePtr;
use print_utils::p;
use semantics_eager::*;
use semantics_entity::*;
pub use stalk::TraceStalk;
use text::{Text, TextRange};
pub use token::{TokenProps, TraceTokenKind};
pub use variant::TraceVariant;

use fold::Indent;
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, sync::Arc};

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Clone)]
pub struct Trace<'eval> {
    parent: Option<TraceId>,
    pub(crate) id: TraceId,
    pub variant: TraceVariant<'eval>,
    pub indent: Indent,
    pub lines: Vec<LineProps<'eval>>,
    pub range: TextRange,
    pub file: FilePtr,
    pub compile_time_version: usize,
    pub has_subtraces: bool,
    pub reachable: bool,
}

#[derive(Debug, Clone, Serialize)]
pub struct LineProps<'eval> {
    pub indent: Indent,
    pub tokens: Vec<TokenProps<'eval>>,
    pub idx: usize,
}

impl<'eval> PartialEq for Trace<'eval> {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl<'eval> Eq for Trace<'eval> {}

impl<'eval> Serialize for Trace<'eval> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Trace", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("lines", &self.lines)?;
        state.serialize_field("kind", &self.variant)?;
        state.serialize_field("has_subtraces", &self.has_subtraces)?;
        state.serialize_field("reachable", &self.reachable)?;
        state.serialize_field(
            "subtraces_container_class",
            &self.subtraces_container_class(),
        )?;
        state.end()
    }
}

impl<'eval> Trace<'eval> {
    pub(crate) fn new(
        parent: Option<TraceId>,
        indent: Indent,
        variant: TraceVariant<'eval>,
        trace_allocator: &TraceFactory<'eval>,
        text: &Text,
        compile_time_version: usize,
    ) -> Self {
        let id = trace_allocator.next_id();
        let (file, range) = variant.file_and_range();
        let reachable = variant.reachable();
        let has_subtraces = variant.has_subtraces(reachable);
        Self {
            id,
            parent,
            indent,
            lines: trace_allocator.lines(id, indent, &variant, text),
            variant,
            file,
            range,
            compile_time_version,
            has_subtraces,
            reachable,
        }
    }

    pub fn id(&self) -> TraceId {
        self.id
    }
}
