use std::sync::Arc;

use crate::*;
use all_llms::AllLlms;
use eterned::db::EternerDb;
use input::VdPipelineInput;

pub struct VdPipelineBuilder<'a, 'db> {
    config: &'a VdPipelineConfig,
    input: &'a VdPipelineInput,
    llm_client: AllLlms<'db>,
}

impl<'a, 'db> VdPipelineBuilder<'a, 'db> {
    pub fn new(
        db: &'db EternerDb,
        config: &'a VdPipelineConfig,
        input: &'a VdPipelineInput,
    ) -> Self {
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
            llm_client: AllLlms::new(db, cache_dir).unwrap(),
        }
    }
}
