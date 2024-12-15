use eterned::db::EternerDb;

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
    pub fn run(&mut self, db: &EternerDb) -> VdPipelineResult<()> {
        assert!(self.tracker.is_none());
        self.tracker = Some(VdPipelineTracker::new(db, &self.config, self.input.clone()));
        Ok(())
    }
}
