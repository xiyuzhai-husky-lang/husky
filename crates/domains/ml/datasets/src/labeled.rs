use std::sync::Arc;
use vm::AnyValueDyn;

pub struct LabeledData<'eval> {
    pub input: Arc<dyn AnyValueDyn<'eval>>,
    pub label: u8,
}
