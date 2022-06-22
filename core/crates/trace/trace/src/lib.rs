mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod variant;

pub use variant::TraceVariant;

use feature_gen::*;
use file::FilePtr;
use fold::Indent;
use husky_tracer_protocol::*;
use print_utils::p;
use semantics_eager::*;
use semantics_entity::*;
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, sync::Arc};
use text::{Text, TextRange};

// ts: { idx: number; parent: number | null; tokens: Token[] }
#[derive(Debug)]
pub struct Trace {
    pub variant: TraceVariant<'static>,
    pub raw_data: TraceData,
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
    pub fn id(&self) -> TraceId {
        self.raw_data.id
    }
}
