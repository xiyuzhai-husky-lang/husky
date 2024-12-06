use crate::{error::VdPipelineResult, tracker::VdPipelineTracker, VdPipelineConfig};
use std::sync::Arc;

pub struct VdPipelineInstance {
    config: VdPipelineConfig,
    tracker: Option<VdPipelineTracker>,
}

impl VdPipelineInstance {
    pub fn new(config: VdPipelineConfig, src_file: Arc<String>) -> Self {
        Self {
            config,
            tracker: None,
        }
    }
}

impl VdPipelineInstance {
    pub fn run(&mut self) -> VdPipelineResult<()> {
        assert!(self.tracker.is_none());
        self.tracker = Some(VdPipelineTracker::new(&self.config));
        Ok(())
    }
}
