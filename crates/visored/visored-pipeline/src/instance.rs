pub mod storage;

use alien_seed::{attach::with_seed, AlienSeed};
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
    pub fn run(
        &mut self,
        seed: AlienSeed,
        db: &EternerDb,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
    ) -> VdPipelineResult<()> {
        assert!(self.tracker.is_none());
        with_seed(seed, || {
            self.tracker = Some(VdPipelineTracker::new(
                db,
                tokio_runtime,
                self.input.clone(),
                self.config.clone(),
            ));
        });
        Ok(())
    }
}
