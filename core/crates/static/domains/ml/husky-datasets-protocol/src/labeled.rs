use husky_trace_protocol::SampleId;
use std::sync::Arc;
use vm::{AnyValueDyn, EvalValue};

pub struct LabeledData<'eval> {
    pub sample_id: SampleId,
    pub input: EvalValue<'eval>,
    pub label: Label,
}

pub use husky_trace_protocol::Label;
