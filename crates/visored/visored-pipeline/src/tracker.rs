use crate::{
    executor::VdPipelineExecutor, input::VdPipelineInput, VdPipelineConfig, VdPipelineConfigData,
};
use all_llms::{transformation::AllLlmsStringTransformationRecord, AllLlmsClient};
use eterned::db::EternerDb;
use std::sync::Arc;

pub struct VdPipelineTracker {
    pub input: Arc<VdPipelineInput>,
    pub config: Arc<VdPipelineConfig>,
    pub raw_proof: String,
    pub simplified_proof: (Vec<AllLlmsStringTransformationRecord>, String),
    pub elaborated_proof: (Vec<AllLlmsStringTransformationRecord>, String),
    pub regularized_proof: (Vec<AllLlmsStringTransformationRecord>, String),
}

impl VdPipelineTracker {
    pub fn new(
        db: &EternerDb,
        tokio_runtime: Arc<tokio::runtime::Runtime>,
        input: Arc<VdPipelineInput>,
        config: Arc<VdPipelineConfig>,
    ) -> Self {
        let mut executor = VdPipelineExecutor::new(db, tokio_runtime, &*input, &*config);
        executor.execute_all();
        let (raw_proof, simplified_proof, elaborated_proof, regularized_proof) = executor.finish();
        Self {
            input,
            config,
            raw_proof,
            simplified_proof,
            elaborated_proof,
            regularized_proof,
        }
    }
}
