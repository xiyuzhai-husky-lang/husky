use crate::{builder::VdPipelineBuilder, input::VdPipelineInput, VdPipelineConfig};
use all_llms::AllLlms;
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
