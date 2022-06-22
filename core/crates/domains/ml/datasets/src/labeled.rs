use husky_tracer_protocol::SampleIdx;
use std::sync::Arc;
use vm::{AnyValueDyn, EvalValue};

pub struct LabeledData<'eval> {
    pub sample_idx: SampleIdx,
    pub input: EvalValue<'eval>,
    pub label: Label,
}

pub use husky_tracer_protocol::Label;
