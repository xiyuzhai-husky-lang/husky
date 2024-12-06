use crate::{builder::VdPipelineBuilder, input::VdPipelineInput, VdPipelineConfig};
use any_llm::AnyLlmClient;
use std::sync::Arc;

pub struct VdPipelineTracker {
    input: Arc<VdPipelineInput>,
}

impl VdPipelineTracker {
    pub fn new(config: &VdPipelineConfig, input: Arc<VdPipelineInput>) -> Self {
        let mut builder = VdPipelineBuilder::new(config, &input);
        Self { input }
    }
}
