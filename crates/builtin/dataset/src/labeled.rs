use std::sync::Arc;
use vm::AnyValueDyn;

pub struct LabeledData {
    pub input: Arc<dyn AnyValueDyn>,
    pub label: usize,
}
