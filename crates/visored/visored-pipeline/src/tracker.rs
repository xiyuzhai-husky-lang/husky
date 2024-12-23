use crate::{builder::VdPipelineBuilder, input::VdPipelineInput, VdPipelineConfig};
use all_llms::AllLlmsClient;
use eterned::db::EternerDb;
use std::sync::Arc;

pub struct VdPipelineTracker {
    input: Arc<VdPipelineInput>,
}

impl VdPipelineTracker {
    pub fn new(db: &EternerDb, config: &VdPipelineConfig, input: Arc<VdPipelineInput>) -> Self {
        let mut builder = VdPipelineBuilder::new(db, config, &*input);
        let () = builder.finish();
        Self { input }
    }
}
