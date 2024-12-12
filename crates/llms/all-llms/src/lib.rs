mod error;

use error::AllLlmsResult;
use openai_llm::OaiLlm;
use std::path::PathBuf;

pub struct AllLlms {
    openai: OaiLlm,
}

impl AllLlms {
    pub fn new(cache_dir: PathBuf) -> AllLlmsResult<Self> {
        let oai_cache_path = cache_dir.join("openai.json");
        assert!(oai_cache_path.parent().unwrap().exists());
        Ok(Self {
            openai: OaiLlm::new(oai_cache_path)?,
        })
    }
}
