use crate::{executor::VdPipelineExecutor, input::VdPipelineInput, VdPipelineConfig};
use all_llms::AllLlmsClient;
use eterned::db::EternerDb;
use std::sync::Arc;

pub struct VdPipelineTracker {
    input: Arc<VdPipelineInput>,
    raw_solution: String,
    simplified_solution: String,
}

impl VdPipelineTracker {
    pub fn new(db: &EternerDb, config: &VdPipelineConfig, input: Arc<VdPipelineInput>) -> Self {
        let mut executor = VdPipelineExecutor::new(db, config, &*input);
        executor.execute_all();
        let (raw_solution, simplified_solution) = executor.finish();
        Self {
            input,
            raw_solution,
            simplified_solution,
        }
    }
}
