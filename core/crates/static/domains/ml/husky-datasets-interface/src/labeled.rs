use crate::*;
use husky_trace_protocol::SampleId;
use std::sync::Arc;

pub struct LabeledData<'eval> {
    pub sample_id: SampleId,
    pub input: __Register<'eval>,
    pub label: Label,
}

pub use husky_trace_protocol::Label;
