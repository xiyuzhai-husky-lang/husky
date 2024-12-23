pub mod storage;

use eterned::db::EternerDb;
use idx_arena::{ArenaIdx, ArenaIdxRange};

use crate::{
    error::VdPipelineResult, input::VdPipelineInput, tracker::VdPipelineTracker, VdPipelineConfig,
};
use std::sync::Arc;

pub struct VdPipelineInstance {
    config: Arc<VdPipelineConfig>,
    input: Arc<VdPipelineInput>,
    tracker: Option<VdPipelineTracker>,
}

pub type VdPipelineInstanceIdx = ArenaIdx<VdPipelineInstance>;
pub type VdPipelineInstanceIdxRange = ArenaIdxRange<VdPipelineInstance>;

impl VdPipelineInstance {
    pub fn new(config: Arc<VdPipelineConfig>, src_file: Arc<VdPipelineInput>) -> Self {
        Self {
            config,
            input: src_file,
            tracker: None,
        }
    }
}

impl VdPipelineInstance {
    #[track_caller]
    pub fn tracker(&self) -> &VdPipelineTracker {
        self.tracker.as_ref().unwrap()
    }
}

impl VdPipelineInstance {
    pub fn run(&mut self, db: &EternerDb) -> VdPipelineResult<()> {
        assert!(self.tracker.is_none());
        self.tracker = Some(VdPipelineTracker::new(
            db,
            self.input.clone(),
            self.config.clone(),
        ));
        Ok(())
    }
}
