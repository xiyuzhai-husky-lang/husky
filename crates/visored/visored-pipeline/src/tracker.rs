use crate::{
    executor::VdPipelineExecutor, input::VdPipelineInput, VdPipelineConfig, VdPipelineConfigData,
};
use all_llms::AllLlmsClient;
use eterned::db::EternerDb;
use std::sync::Arc;

pub struct VdPipelineTracker {
    input: Arc<VdPipelineInput>,
    config: Arc<VdPipelineConfig>,
    raw_solution: String,
    simplified_solution: String,
}

impl VdPipelineTracker {
    pub fn new(db: &EternerDb, input: Arc<VdPipelineInput>, config: Arc<VdPipelineConfig>) -> Self {
        let mut executor = VdPipelineExecutor::new(db, &*input, &*config);
        executor.execute_all();
        let (raw_solution, simplified_solution) = executor.finish();
        Self {
            input,
            config,
            raw_solution,
            simplified_solution,
        }
    }
}
