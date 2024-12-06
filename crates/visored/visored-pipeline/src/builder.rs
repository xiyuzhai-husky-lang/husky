use std::sync::Arc;

use crate::*;
use any_llm::AnyLlmClient;
use input::VdPipelineInput;

pub struct VdPipelineBuilder<'db> {
    config: &'db VdPipelineConfig,
    input: &'db VdPipelineInput,
    llm_client: AnyLlmClient,
}

impl<'db> VdPipelineBuilder<'db> {
    pub fn new(config: &'db VdPipelineConfig, input: &'db VdPipelineInput) -> Self {
        let base = input.file_path.parent().unwrap();
        let cache_dir = config.cache_dir.to_logical_path(base).join(format!(
            "{}/example-{}",
            input.file_path.file_stem().unwrap().to_str().unwrap(),
            input.index
        ));
        std::fs::create_dir_all(&cache_dir).unwrap();
        Self {
            config,
            input,
            llm_client: AnyLlmClient::new(cache_dir).unwrap(),
        }
    }
}
