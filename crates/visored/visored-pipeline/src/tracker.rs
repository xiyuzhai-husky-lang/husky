use crate::{builder::VdPipelineBuilder, VdPipelineConfig};
use any_llm::AnyLlmClient;

pub struct VdPipelineTracker {}

impl VdPipelineTracker {
    pub fn new(config: &VdPipelineConfig) -> Self {
        let mut builder = VdPipelineBuilder::new(config);
        Self {}
    }
}
