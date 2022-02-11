mod alloc;
mod figure;
mod kind;
#[cfg(test)]
mod tests;
mod token;

pub use alloc::{AllocateTrace, TraceAllocator, TraceId};
pub use figure::FigureProps;
pub use kind::TraceKind;
pub use token::{TokenProps, TraceTokenKind};

use std::{borrow::Cow, sync::Arc};

use common::*;
use fold::Indent;
use serde::{ser::SerializeStruct, Serialize};

use token::*;

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug, Clone)]
pub struct Trace {
    parent: Option<TraceId>,
    pub(crate) id: TraceId,
    pub indent: fold::Indent,
    pub kind: TraceKind,
    pub tokens: Vec<TokenProps>,
}

impl PartialEq for Trace {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

impl Eq for Trace {}

impl Serialize for Trace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        let mut state = serializer.serialize_struct("Trace", 3)?;
        state.serialize_field("id", &self.id)?;
        state.serialize_field("parent", &self.parent)?;
        state.serialize_field("indent", &self.indent)?;
        state.serialize_field("tokens", &self.tokens)?;
        state.end()
    }
}

impl Trace {
    pub(crate) fn new(
        parent: Option<TraceId>,
        indent: Indent,
        kind: TraceKind,
        trace_allocator: &TraceAllocator,
    ) -> Self {
        let id = trace_allocator.next_id();
        Self {
            id,
            parent,
            indent,
            tokens: trace_allocator.tokens(id, &kind),
            kind,
        }
    }

    // pub(crate) fn main(main_file: FilePtr, feature_block: Arc<FeatureBlock>) -> Arc<Self> {
    //     Self::new(
    //         None,
    //         0,
    //         TraceKind::Main {
    //             main_file,
    //             feature_block,
    //         },
    //     )
    // }
}
