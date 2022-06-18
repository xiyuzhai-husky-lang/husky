use std::sync::Arc;
use vm::{AnyValueDyn, EvalValue};

pub struct LabeledData<'eval> {
    pub sample_id: usize,
    pub input: EvalValue<'eval>,
    pub label: Label,
}

pub use husky_tracer_protocol::Label;
