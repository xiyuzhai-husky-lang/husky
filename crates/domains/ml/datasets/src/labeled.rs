use std::sync::Arc;
use vm::{AnyValueDyn, EvalValue};

pub struct LabeledData<'eval> {
    pub input: EvalValue<'eval>,
    pub label: u8,
}
