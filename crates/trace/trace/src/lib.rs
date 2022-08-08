mod stalk;
mod subtraces;
#[cfg(test)]
mod tests;
mod variant;

pub use variant::TraceVariant;

use fold::Indent;
use husky_eager_semantics::*;
use husky_entity_semantics::*;
use husky_feature_gen::*;
use husky_file::FilePtr;
use husky_print_utils::p;
use husky_text::{HuskyText, TextRange};
use husky_trace_protocol::*;
use serde::{ser::SerializeStruct, Serialize};
use std::{borrow::Cow, sync::Arc};

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
