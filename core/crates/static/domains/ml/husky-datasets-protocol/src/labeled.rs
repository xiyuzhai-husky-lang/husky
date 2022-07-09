use husky_trace_protocol::SampleId;
use std::sync::Arc;
use vm::{__AnyValueDyn, __EvalValue};

pub struct LabeledData<'eval> {
    pub sample_id: SampleId,
    pub input: __EvalValue<'eval>,
    pub label: Label,
}

pub use husky_trace_protocol::Label;
