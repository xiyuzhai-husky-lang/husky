use crate::{
    executor::VdPipelineExecutor, input::VdPipelineInput, VdPipelineConfig, VdPipelineConfigData,
};
use all_llms::{transformation::AllLlmsStringTransformationRecord, AllLlmsClient};
use eterned::db::EternerDb;
use std::sync::Arc;

pub struct VdPipelineTracker {
    pub input: Arc<VdPipelineInput>,
    pub config: Arc<VdPipelineConfig>,
    pub raw_solution: String,
    pub simplified_solution: (Vec<AllLlmsStringTransformationRecord>, String),
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
