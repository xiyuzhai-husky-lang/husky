use crate::*;
use any_llm::AnyLlmClient;

pub struct VdPipelineBuilder<'db> {
    config: &'db VdPipelineConfig,
    llm_client: AnyLlmClient,
}

impl<'db> VdPipelineBuilder<'db> {
    pub fn new(config: &'db VdPipelineConfig) -> Self {
        Self {
            config,
            llm_client: AnyLlmClient::new(config.cache_dir.clone()).unwrap(),
        }
    }
}
