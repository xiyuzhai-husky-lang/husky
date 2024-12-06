use crate::{
    error::VdPipelineResult, input::VdPipelineInput, tracker::VdPipelineTracker, VdPipelineConfig,
};
use std::sync::Arc;

pub struct VdPipelineInstance {
    config: VdPipelineConfig,
    input: Arc<VdPipelineInput>,
    tracker: Option<VdPipelineTracker>,
}

impl VdPipelineInstance {
    pub fn new(config: VdPipelineConfig, src_file: Arc<VdPipelineInput>) -> Self {
        Self {
            config,
            input: src_file,
            tracker: None,
        }
    }
}

impl VdPipelineInstance {
    pub fn run(&mut self) -> VdPipelineResult<()> {
        assert!(self.tracker.is_none());
        self.tracker = Some(VdPipelineTracker::new(&self.config, self.input.clone()));
        Ok(())
    }
}
